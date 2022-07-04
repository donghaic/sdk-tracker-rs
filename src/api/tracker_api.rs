use std::collections::HashMap;
use std::ops::Deref;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Query;

use crate::models::ad_event::AdEvent;
use crate::utils::web_utils;

pub async fn handler(
    request: HttpRequest,
    query: Query<HashMap<String, String>>,
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    // 监控事件类型
    let event_type = web_utils::get_string_or(&query, "type", "");

    match event_type.as_str() {
        "bid" => handle_bid(&request, &query, app_data).await,
        "imp" => super::comm::handle_imp(&request, &query, app_data).await,
        "clk" => super::comm::handle_click(&request, &query, app_data).await,
        "video" => super::comm::handle_video(&request, &query, app_data).await,
        "union_pv" => handle_pv(&request, &query, app_data).await,
        _ => (),
    };

    HttpResponse::Ok().finish()
}

/// 请求宏  __REQ__    1：请求， 2：填充， 3：超时，4：取消
/// 竞价宏  __BID__    1：赢得展示， 2：失败
/// 详细逻辑（前面的数字代表请求宏，后面的数据代表竞价宏）：
/// １＆２＝请求成功，填充失败
/// ２＆１＝请求成功，填充成功，竞价成功
/// ２＆２＝请求成功，填充成功，竞价失败
/// ３＆２＝请求超时（包含所有超时）
/// ４＆２＝因网络问题等问题，无法发起请求。
async fn handle_bid(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: web::Data<crate::AppState>) {
    let req_status = web_utils::get_u32_or(&query, "rs", 0);  // 1请求， 2：填充， 3：超时，4：取消
    let bid_status = web_utils::get_u32_or(&query, "bs", 0);  // 1：赢得展示， 2：失败

    let mut ad_event = super::parse_event(&request, &query);
    println!("{:?}", ad_event);

    let need_check = super::comm::is_need_check(&mut ad_event);
    if need_check {
        // app_data.redis_service.get_str();
        ();
    }

    ad_event.request = 1;
    if req_status == 2 {
        ad_event.response = 1;
        ad_event.bid = 1;
    }

    if bid_status == 1 {
        ad_event.win = 1;
    }

    ad_event.event_type = "req".to_string();
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("test", event_json).await;

    ad_event.event_type = "bid".to_string();
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("test", event_json).await;
}


async fn handle_pv(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: web::Data<crate::AppState>) {
    if is_fit_priority(query) {
        let req_status = web_utils::get_u32_or(&query, "rs", 0); // 1: 请求， 2：填充
        let bid_status = web_utils::get_u32_or(&query, "bs", 0); // 1：赢得展示， 2：失败
        let mut ad_event = super::parse_event(&request, &query);

        let priority = web_utils::get_u32_or(&query, "pri", 0);
        let full_pri: Vec<u32> = web_utils::get_string_or(&query, "pa", "").split(',')
            .map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();

        for p in full_pri {
            ad_event.request = 1;
            ad_event.status = 10; //pv 媒体报表优先级时需要用到的字段
            if p == priority {
                // 成功竞价优先级 处理竞价、填充、赢价，其他优先级只有请求
                ad_event.status = 0;
                if req_status == 2 {
                    ad_event.response = 1;
                    ad_event.bid = 1;
                }
                if bid_status == 1 {
                    ad_event.win = 1;
                }

                break;
            }
            ad_event.event_type = "pv".to_string();
            let event_json = serde_json::to_vec(&ad_event).unwrap();
            app_data.kafka_producer.publish_message("test", event_json).await;
        }
    }
}

fn is_fit_priority(query: &Query<HashMap<String, String>>) -> bool {
    let priority = web_utils::get_string_or(&query, "pri", "");
    let full_pri = web_utils::get_string_or(&query, "pa", "").split(',')
        .map(|s| s.to_string()).find(|s| s.eq(&priority));

    let is_fit = match full_pri {
        None => false,
        Some(_) => true
    };
    is_fit
}
