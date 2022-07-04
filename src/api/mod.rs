use std::collections::HashMap;

use actix_web::HttpRequest;
use actix_web::web::Query;
use base64_url;
use base64_url::base64::DecodeError;
use chrono::{DateTime, Local, Timelike};

use crate::models::ad_event;
use crate::utils::web_utils;

mod comm;
pub mod tracker_api;
pub mod tracker_api_v2;
pub mod preload_api;

fn parse_event(request: &HttpRequest, query: &Query<HashMap<String, String>>) -> ad_event::AdEvent {
    let ip = web_utils::get_client_ip(request);
    let time = Local::now();

    let mut event = ad_event::AdEvent {
        date: time.timestamp(),
        event_time: time.timestamp(),
        hour: time.hour() as u8,
        event_type: String.default(),
        req_uid: web_utils::get_string_or(query, "rid", ""),
        media_id: web_utils::get_u32_or(query, "mid", 0),
        pid: web_utils::get_u32_or(query, "pid", 0),
        accept_id: web_utils::get_u32_or(query, "aid", 0),
        priority: web_utils::get_u32_or(query, "pri", 0) as u8,
        strategy_id: web_utils::get_u32_or(query, "sid", 0),
        seg_cfg_id: web_utils::get_u32_or(query, "at", 0),
        access_type: web_utils::get_u32_or(query, "at", 0) as u8,
        service_type: web_utils::get_u32_or(query, "st", 0) as u8,
        ad_slot_style: web_utils::get_u32_or(query, "as", 0) as u8,
        adslot_type: 0,
        ad_type: web_utils::get_u32_or(query, "at", 0) as u8,
        filter_id: web_utils::get_u32_or(query, "fi", 0),
        plan_id: web_utils::get_u32_or(query, "pi", 0),
        code: web_utils::get_string_or(query, "codeid", ""),
        creative_type: web_utils::get_string_or(query, "ct", ""),
        adver_id: web_utils::get_string_or(query, "advid", ""),
        cid: web_utils::get_string_or(query, "cid", ""),
        os: web_utils::get_u32_or(query, "os", 0) as u8,
        network: web_utils::get_u32_or(query, "carrier", 0) as u8,
        carrier: web_utils::get_u32_or(query, "net", 0) as u8,
        ip: ip,
        did: "".to_string(),
        usr: web_utils::get_string_or(query, "usr", ""),
        app_bundle: web_utils::get_string_or(query, "pkg", ""),
        province: "".to_string(),
        city: "".to_string(),
        status: 0,
        price: 0,
        media_cost: 0,
        cash_price: 0.0,
        grant_price: 0.0,
        bid_price: 0,
        win_price: 0,
        request: 0,
        response: 0,
        bid: 0,
        win: 0,
        win2: 0,
        show: 0,
        click: 0,
        download_start: 0,
        download_finish: 0,
        click_install: 0,
        install_finish: 0,
        download_active: 0,
        open_deep_link: 0,
        open_web_page: 0,
        total_video_play: 0,
        total_video_click: 0,
        total_video_success: 0,
        pst: 0,
        channel_id: web_utils::get_string_or(query, "chid", ""),
        inner_version: web_utils::get_string_or(query, "iv", ""),
        version_code: web_utils::get_string_or(query, "vc", ""),
        app_platform: web_utils::get_string_or(query, "apf", ""),
        os_ver: web_utils::get_string_or(query, "ov", ""),
        plugin_name: web_utils::get_string_or(query, "pn", ""),
        plugin_ver: web_utils::get_string_or(query, "pv", ""),
        bi_network: web_utils::get_string_or(query, "nk", ""),
        alg_version: "".to_string(),
        user_id: 0,
        gender: 0,
        age: 0,
        tmp_ver: "".to_string(),
    };

    let base64_fields_str = web_utils::get_string_or(query, "didm", "");
    let decode_str = base64_url::decode(base64_fields_str.as_bytes());
    println!("==== {}", base64_fields_str);
    let json_str = match decode_str {
        Ok(v) => v,
        Err(_) => Vec::new(),
    };


    if json_str.len() > 0 {
        let encode_info = serde_json::from_slice(json_str.as_slice())
            .unwrap_or_else(|_e| {
                ad_event::EncodeInfo {
                    usr: "".to_string(),
                    gender: 0,
                    age: 0,
                    adtype: 0,
                }
            });
        println!("encode_info ==== {:?}", encode_info);
        event.gender = encode_info.gender;
        event.age = encode_info.age;
        event.adslot_type = encode_info.adtype;
    }
    println!("event ==== {:?}", event);


    return event;
}