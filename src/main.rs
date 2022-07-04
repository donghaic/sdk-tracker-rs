use std::collections::HashMap;
use std::time::Duration;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::Query;
use chrono::Local;
use job_scheduler::{Job, JobScheduler};
use redis::ConnectionAddr;
use tokio;
use tokio::runtime;
use tokio::runtime::Runtime;

use union_monitor::api;
use union_monitor::AppState;
use union_monitor::services::kafka_service::KafkaProducer;
use union_monitor::services::redis_service::RedisService;
use union_monitor::store::local_store;
use union_monitor::store::sled;
use union_monitor::tasks;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listen to :8080");


    let info = redis::ConnectionInfo {
        addr: Box::new(ConnectionAddr::Tcp(String::from("localhost"), 6379)),
        db: 0,
        username: None,
        passwd: None,
    };
    let redis_client = redis::Client::open(info).expect("can create redis client");
    let store = sled::SledStore::open("./ads_db").unwrap();
    let local_store = local_store::LocalStore::from(store.clone());

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .thread_name("sync-pool")
        .enable_all()
        .build()
        .unwrap();

    tasks::start(tokio_runtime, local_store);

    HttpServer::new(move || {
        let redis_service = RedisService::new(redis_client.clone());
        let kafka_producer = KafkaProducer::new(String::from("localhost:9092"));
        let local_store = local_store::LocalStore::from(store.clone());

        let app_state = AppState { redis_service, kafka_producer, local_store };

        App::new()
            .data(app_state)
            .route("/hey", web::get().to(manual_hello))
            .route("/union-track", web::get().to(api::tracker_api::handler))
            .route("/v2/union-track", web::get().to(api::tracker_api_v2::get_handler))
            .route("/v2/union-track", web::post().to(api::tracker_api_v2::post_handler))
            .route("/preload-track", web::post().to(api::preload_api::handler))
            .route("/ping", web::get().to(ping))
            .route("/ping", web::head().to(ping))
    }).bind("0.0.0.0:8080")?.run().await
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn manual_hello(req: HttpRequest, query: Query<HashMap<String, String>>) -> impl Responder {
    let name = query.get("name").unwrap();
    let connection_info = req.connection_info().clone();
    let ip = connection_info.remote_addr().unwrap_or("");
    HttpResponse::Ok().body(format!("Hey there! {}, ip {}", name, ip))
}
