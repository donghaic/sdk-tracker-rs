use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;
use chrono::{Local, Timelike};

use crate::models::ad_event::PreloadTrackEvent;
use crate::utils::web_utils;

pub async fn handler(
    request: HttpRequest,
    query: Query<HashMap<String, String>>,
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    let ip = web_utils::get_client_ip(&request);
    let time = Local::now();
    let pre_time = web_utils::get_u32_or(&query, "ts", 0);
    let duration = time.timestamp() - pre_time as i64;

    let event = PreloadTrackEvent {
        date: time.timestamp(),
        hour: time.hour() as u8,
        event_time: time.timestamp(),
        strategy_id: web_utils::get_u32_or(&query, "sid", 0),
        adver_id: web_utils::get_string_or(&query, "aid", ""),
        cid: web_utils::get_string_or(&query, "cid", ""),
        ip,
        channel_id: web_utils::get_string_or(&query, "cnn", ""),
        usr: web_utils::get_string_or(&query, "usr", ""),
        did: web_utils::get_string_or(&query, "did", ""),
        os: web_utils::get_string_or(&query, "os", ""),
        model: web_utils::get_string_or(&query, "model", ""),
        make: web_utils::get_string_or(&query, "make", ""),
        app_id: web_utils::get_string_or(&query, "appid", ""),
        app_bundle: web_utils::get_string_or(&query, "pkg", ""),
        province: web_utils::get_string_or(&query, "cid", ""),
        city: web_utils::get_string_or(&query, "cid", ""),
        duration,
    };

    let event_json = serde_json::to_vec(&event).unwrap();
    app_data.kafka_producer.publish_message("preload_track", event_json).await;

    HttpResponse::Ok().finish()
}