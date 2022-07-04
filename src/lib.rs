use crate::services::*;
use crate::store::local_store;

pub mod models;
pub mod api;
pub mod services;
pub mod config;
pub mod utils;
pub mod store;
pub mod tasks;
mod db;
mod meta;
mod ipip;


pub struct ServiceContainer {}

pub struct AppState {
    pub redis_service: redis_service::RedisService,
    pub kafka_producer: kafka_service::KafkaProducer,
    pub local_store: local_store::LocalStore,
}

