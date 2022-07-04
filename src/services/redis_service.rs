use redis::{aio::Connection, AsyncCommands, Client, FromRedisValue, RedisError};

pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub fn new(client: Client) -> RedisService {
        RedisService { client }
    }
}

impl RedisService {
    async fn get_con(&self) -> Result<Connection, RedisError> {
        self.client.get_async_connection().await
    }

    pub async fn get_str(&self, key: &str) -> Result<String, RedisError> {
        let mut conn = self.get_con().await?;
        let value = conn.get(key).await?;
        FromRedisValue::from_redis_value(&value)
    }

    pub async fn set_str(&self, key: &str, val: &str) -> Result<(), RedisError> {
        let result: Result<(), RedisError> = self.get_con().await?.set(key, val).await;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }


    #[actix_rt::test]
    async fn test_redis() {
        async {
            let redis_client = redis::Client::open("redis://127.0.0.1/").expect("can create redis client");
            let redis_service = RedisService::new(redis_client);
            redis_service.set_str("hello", "world").await;
            let x = redis_service.get_str("hello").await;
            println!("{:?}", x)
        }.await
    }

    #[test]
    fn test_block() {
        let redis_client = redis::Client::open("redis://127.0.0.1/").expect("can create redis client");
        let redis_service = RedisService::new(redis_client);
        aw!(redis_service.set_str("hello", "world2"));
        let x = aw!(redis_service.get_str("hello"));
        println!("{:?}", x.unwrap())
    }
}
