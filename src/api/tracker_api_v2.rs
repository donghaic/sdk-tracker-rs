use std::collections::HashMap;

use actix_web::{error, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::StreamExt;

use crate::models::ad_event::*;
use crate::models::entity::{Accept, Strategy};
use crate::store::error::{Result as StorageResult, StorageError};
use crate::utils::web_utils;

const MAX_SIZE: usize = 262_144; // max payload size is 256k


pub async fn post_handler(
    request: HttpRequest,
    query: Query<HashMap<String, String>>,
    app_data: web::Data<crate::AppState>,
    mut payload: web::Payload,
) -> impl Responder {

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let bid_status = serde_json::from_slice::<Vec<BidStatus>>(&body)?;

    handle_global_bid(&request, &query, &bid_status, app_data);

    Ok(HttpResponse::Ok().json(bid_status))
}

pub async fn get_handler(
    request: HttpRequest,
    query: Query<HashMap<String, String>>,
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    // 监控事件类型
    let event_type = web_utils::get_string_or(&query, "type", "");

    if "imp".eq(event_type.as_str()) {
        super::comm::handle_imp(&request, &query, app_data).await
    } else if "clk".eq(event_type.as_str()) {
        super::comm::handle_click(&request, &query, app_data).await
    } else if "video".eq(event_type.as_str()) {
        super::comm::handle_video(&request, &query, app_data).await
    }
    HttpResponse::Ok().finish()
}

async fn handle_global_bid(request: &HttpRequest, query: &Query<HashMap<String, String>>, bid_items: &Vec<BidStatus>, app_data: web::Data<crate::AppState>) {
    println!("pri={}", bid_items.len());

    let mut has_win = false;
    let size = bid_items.len();

    for i in 0..size {
        let bid_item = bid_items.get(i).unwrap();
        let pri = bid_item.priority;

        for status in bid_item.status.as_slice() {
            handle_strategy_bid(request, query, &app_data, pri, status).await;
            if status.bid == 1 {
                has_win = true;
            }
        }

        // 当前层级存在赢价后面的就没有用，防止客户端上报过多数据
        if has_win {
            break;
        }
    }
}

async fn handle_strategy_bid(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: &web::Data<crate::AppState>, pri: u8, status: &Status) {
    let mut ad_event = super::parse_event(&request, &query);
    ad_event.priority = pri;
    ad_event.strategy_id = status.sid;

    if status.req == 1 {
        ad_event.request = 1;
    } else if status.req == 2 {
        ad_event.request = 1;
        ad_event.response = 1;
        ad_event.bid = 1;
        ad_event.bid_price = status.price;
    } else if status.req == 3 {
        ad_event.request = 1;
        ad_event.status = 11;
    } else if status.req == 4 {
        ad_event.request = 1;
        ad_event.status = 12;
    }

    if status.bid == 1 {
        ad_event.win = 1;
        ad_event.win_price = status.price
    }

    let strategy_result: StorageResult<Option<Strategy>> = app_data.local_store.get(format!("strategy_{}", status.sid)).await;


    if strategy_result.is_ok() {
        let option1 = strategy_result.unwrap();
        if option1.is_some() {
            let strategy = option1.unwrap();
            ad_event.accept_id = strategy.accept_id;
            ad_event.access_type = strategy.access_type;
            ad_event.filter_id = strategy.targeting_filter_id;
            ad_event.code = strategy.code_id;
            // TODO 文案


            let accept_result: StorageResult<Option<Accept>> = app_data.local_store.get(format!("accept_{}", strategy.accept_id)).await;
            if accept_result.is_ok() {
                let option = accept_result.unwrap();

                if option.is_some() {
                    let accept = option.unwrap();

                    if accept.url.len() == 0 {
                        if ad_event.win == 1 {
                            ad_event.request = 0;
                            ad_event.response = 0;
                            ad_event.bid = 0;
                            ad_event.win = 0;
                            ad_event.win2 = 1;
                        }
                    } else {
                        ad_event.win2 = ad_event.win;
                    }
                }
            }
        }
    }

    ad_event.event_type = String::from("bid");
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("monitor", event_json).await;

    ad_event.event_type = String::from("req");
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("monitor", event_json).await;
}

