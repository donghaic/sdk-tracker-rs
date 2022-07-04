extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

// 广告事件对接
#[derive(Debug, Serialize, Deserialize)]
pub struct AdEvent {
    pub date: i64,
    pub hour: u8,
    pub event_time: i64,
    pub event_type: String,
    #[serde(rename = "uid")]
    pub req_uid: String,
    pub media_id: u32,
    pub pid: u32,
    pub accept_id: u32,
    pub priority: u8,
    pub strategy_id: u32,
    pub seg_cfg_id: u32,
    pub access_type: u8,
    pub service_type: u8,
    pub ad_slot_style: u8,
    pub adslot_type: u8,
    pub ad_type: u8,
    pub filter_id: u32,
    pub plan_id: u32,
    pub code: String,
    pub creative_type: String,
    pub adver_id: String,
    pub cid: String,
    pub os: u8,
    pub network: u8,
    pub carrier: u8,
    pub ip: String,
    pub did: String,
    pub usr: String,
    pub app_bundle: String,
    pub province: String,
    pub city: String,
    pub status: u8,
    pub price: u64,
    pub media_cost: u64,
    pub cash_price: f64,
    pub grant_price: f64,
    pub bid_price: u64,
    pub win_price: u64,
    pub request: u8,
    pub response: u8,
    pub bid: u8,
    pub win: u8,
    pub win2: u8,
    pub show: u8,
    pub click: u8,
    pub download_start: u8,
    pub download_finish: u8,
    pub click_install: u8,
    pub install_finish: u8,
    pub download_active: u8,
    pub open_deep_link: u8,
    pub open_web_page: u8,
    pub total_video_play: u8,
    pub total_video_click: u8,
    pub total_video_success: u8,
    pub pst: u8,

    pub channel_id: String,
    pub inner_version: String,
    pub version_code: String,
    pub app_platform: String,
    pub os_ver: String,
    pub plugin_name: String,
    pub plugin_ver: String,
    pub bi_network: String,
    pub alg_version: String,
    pub user_id: u64,

    pub gender: u8,
    pub age: u8,

    #[serde(skip)]
    pub tmp_ver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodeInfo {
    pub usr: String,

    #[serde(rename = "z1")]
    pub gender: u8,

    #[serde(rename = "z2")]
    pub age: u8,

    pub adtype: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidStatus {
    pub priority: u8,
    pub status: Vec<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub sid: u32,
    pub req: u32,
    pub bid: u32,
    pub price: u64,
}


/// 预加载状态上报
#[derive(Debug, Serialize)]
pub struct PreloadTrackEvent {
    pub date: i64,
    pub hour: u8,
    pub event_time: i64,
    pub strategy_id: u32,
    pub adver_id: String,
    pub cid: String,
    pub ip: String,
    pub channel_id: String,
    pub usr: String,
    pub did: String,
    pub os: String,
    pub model: String,
    pub make: String,
    pub app_id: String,
    pub app_bundle: String,
    pub province: String,
    pub city: String,
    pub duration: i64,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let n = AdEvent {
            date: 0,
            hour: 0,
            event_time: 10000,
            event_type: String::from("imp"),
            req_uid: String::from("imp"),
            media_id: 100,
            pid: 0,
            accept_id: 0,
            priority: 0,
            strategy_id: 0,
            seg_cfg_id: 0,
            access_type: 0,
            service_type: 0,
            ad_slot_style: 0,
            adslot_type: 0,
            ad_type: 0,
            filter_id: 0,
            plan_id: 0,
            code: String::from("imp"),
            creative_type: String::from("imp"),
            adver_id: String::from("imp"),
            cid: String::from("imp"),
            os: 0,
            network: 0,
            did: String::from("1"),
            usr: String::from("imp"),
            app_bundle: String::from("imp"),
            province: String::from("imp"),
            city: String::from("imp"),
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
            channel_id: String::from("imp"),
            inner_version: String::from("imp"),
            version_code: String::from("imp"),
            app_platform: String::from("imp"),
            os_ver: String::from("imp"),
            plugin_name: String::from("imp"),
            plugin_ver: String::from("imp"),
            bi_network: String::from("imp"),
            alg_version: String::from("imp"),
            user_id: 0,
            gender: 0,
            ip: String::from("1.2.3.4"),
            carrier: 0,
            age: 0,
            tmp_ver: "".to_string(),
        };
        println!("{:?}", n);

        let result = serde_json::to_string(&n);
        println!("{}", result.unwrap_or(String::from("")));
    }
}


