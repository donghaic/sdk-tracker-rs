use std::{fs, path};

use serde::{Deserialize, Serialize};

/// 应用配置
#[derive(Serialize, Deserialize, Debug)]
struct AppCfg {
    pub port: u16,
    pub pidfile: String,
    pub log_dir: String,
    pub ip_lib: String,
    pub redis: RedisCfg,
    pub mysql: MySQLCfg,
    pub kafka: KafkaCfg,
    pub topics: Topics,
    pub loader_interval_second: u8,
}


impl AppCfg {
    /// 加载配置文件
    pub fn load(config_path: &path::Path) -> Result<AppCfg, serde_yaml::Error> {
        let config = serde_yaml::from_str(&fs::read_to_string(config_path).unwrap())?;
        println!("load config from `{}`", config_path.to_string_lossy());
        Ok(config)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct RedisCfg {
    pub host: String,
    pub port: i16,
    pub db: u16,
    pub username: Option<String>,
    pub passwd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MySQLCfg {
    pub host: String,
    pub port: i16,
    pub db: String,
    pub username: Option<String>,
    pub passwd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KafkaCfg {
    pub client_id: String,
    pub brokers: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Topics {
    pub app_req: String,
    pub dsp_req: String,
    pub monitor: String,
    pub preload_track: String,
    pub event_status: String,
    pub pv_req: String,
    pub ac_req: String,
    pub report_err: String,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cfg() {
        let file = "./app.yaml";
        let path = Path::new(file);
        let mut cfg = AppCfg::load(path).unwrap();
        println!("{}", cfg.port)
    }
}