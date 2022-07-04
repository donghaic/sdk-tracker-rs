use std::collections::HashMap;

use actix_web::{HttpRequest, web};
use actix_web::web::Query;
use base64_url::base64::DecodeError;

use crate::models::ad_event::AdEvent;
use crate::utils::web_utils;

pub(crate) fn is_need_check(ad_event: &mut AdEvent) -> bool {
    match ad_event.req_uid {
        _ if ad_event.req_uid.eq("__REQID__") => true,
        _ => false
    }
}

pub(crate) async fn handle_imp(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: web::Data<crate::AppState>) {
    let mut ad_event = super::parse_event(&request, &query);
    ad_event.show = 1;
    let price = web_utils::get_string_or(&query, "c0", "0").parse().unwrap_or(0);
    if price > 0 {
        ad_event.price = price;
    } else {
        let price_base64 = web_utils::get_string_or(&query, "cc", "");
        let price = match base64_url::decode(price_base64.as_str()) {
            Ok(p) => String::from_utf8(p).map_or_else(|e| 0, |v| v.parse().unwrap_or(0)),
            Err(_) => 0 as u64,
        };
        ad_event.price = price;
    }

    ad_event.event_type = String::from("imp");
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    println!("{:?}", event_json);
    app_data.kafka_producer.publish_message("monitor", event_json).await;
}

pub(crate) async fn handle_click(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: web::Data<crate::AppState>) {
    let mut ad_event = super::parse_event(&request, &query);
    ad_event.click = 1;
    ad_event.event_type = String::from("click");
    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("monitor", event_json).await;
}

pub(crate) async fn handle_video(request: &HttpRequest, query: &Query<HashMap<String, String>>, app_data: web::Data<crate::AppState>) {
    let mut ad_event = super::parse_event(&request, &query);
    // 替换宏 __ACTION__
    // 1 - 广告位展示成功,
    // 2 - 广告位点击成功
    // 3 - 激励视频播放完毕
    // 4 - 激励视频点击成功
    // 5 - 激励视频播放成功

    //ios激励视频改成竞价逻辑，请求等用gurl的。安卓经讨论保持原先模式，20201209。os=1表示安卓
    if ad_event.os == 1 {
        ad_event.request = 1;
        ad_event.response = 1;
        ad_event.bid = 1;
        ad_event.win = 1;
    }

    let price_base64 = web_utils::get_string_or(&query, "cc", "");
    let price = match base64_url::decode(price_base64.as_str()) {
        Ok(p) => String::from_utf8(p).map_or_else(|e| 0, |v| v.parse().unwrap_or(0)),
        Err(_) => 0 as u64,
    };
    ad_event.price = price;

    let action = web_utils::get_string_or(&query, "m0", "");
    match action.as_str() {
        "1" => {
            ad_event.show = 1;
            ad_event.event_type = String::from("imp");
            ;
        }
        "2" => {
            ad_event.click = 1;
            ad_event.event_type = String::from("click");
            ;
        }
        "3" => {
            ad_event.total_video_play = 1;
            ad_event.event_type = String::from("video");
        }
        "4" => {
            ad_event.total_video_click = 1;
            ad_event.event_type = String::from("video");
        }
        "5" => {
            ad_event.total_video_success = 1;
            ad_event.event_type = String::from("video");
        }
        _ => ()
    }

    let event_json = serde_json::to_vec(&ad_event).unwrap();
    app_data.kafka_producer.publish_message("monitor", event_json).await;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        let price = base64_url::decode("MzAw");
        let result = String::from_utf8(price.unwrap()).unwrap();
        println!("{}", result);
        let base64_fields = base64_url::decode("eyJ1c3IiOiJpMzE4MDg3NzA3NyIsImRpZCI6IjkzNTMxNmE1MWFjMWE4ZGMiLCJvcyI6MSwidmVyIjoiNy40NS4wXzQxMyJ9");
        let fields = String::from_utf8(base64_fields.unwrap()).unwrap();
        println!("{}", fields);
    }

    #[test]
    fn test_split() {
        let full_pri: Vec<u32> = "1,2,,3,5,".split(',')
            .map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();

        println!("{:?}", full_pri);

        let priority = String::from("5");
        let full_pri = "1,2,,3,5,".split(',')
            .map(|s| s.to_string()).find(|s| s.eq(&priority));
        println!("{:?}", full_pri.unwrap());
    }
}