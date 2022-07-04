use anyhow::Result;
use r_cache::cache::Cache;
use redis::{Client, Commands};

use crate::models::entity::Strategy;

pub struct MetaCache {
    s_cache: Cache<u32, Strategy>,
    redis_client: Client,
}

impl MetaCache {
    pub fn get_strategyById(&self, id: u32) -> Result<Strategy> {
        let x = self.s_cache.get(&id).await;
        let option = x.or_else(|| {
            let conn = self.redis_client.get_connection()?;
            let result = connection.get(String::from(id));
            return None;
        });
        todo!()
    }
}