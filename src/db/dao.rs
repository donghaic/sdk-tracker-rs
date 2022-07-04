use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::Row;

use crate::models::entity::Strategy;

struct Dao<'a> {
    pool: &'a MySqlPool,
}


impl<'a> Dao<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Dao { pool }
    }
}

impl<'a> Dao<'a> {
    pub async fn get_all(&mut self) -> sqlx::Result<Vec<Strategy>> {
        let stream = sqlx::query("SELECT * FROM ssp_strategy").map(|row: MySqlRow| {
            Ok(Strategy {
                id: row.try_get("id")?,
                accept_id: 0,
                access_type: 0,
                service_type: 0,
                targeting_filter_id: 0,
                plan: 0,
                code_id: "".to_string(),
            })
        }).fetch(self.pool);


        Ok(results)
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::LevelFilter;
    use sqlx::ConnectOptions;
    use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

    use super::*;

    #[actix_rt::test]
    async fn test_dao() {
        let mut op = MySqlConnectOptions::new()
            .username("root")
            .password("")
            .host("localhost")
            .port(3306)
            .database("zy_ssp");

        op.log_slow_statements(LevelFilter::Debug, Duration::new(10, 0));
        op.log_statements(LevelFilter::Off);

        let pool = MySqlPoolOptions::new()
            .max_connections(100)
            .connect_with(op).await.unwrap();

        let mut dao = Dao::new(&pool);
        let x = dao.get_all().await;
        let vec = x.unwrap();
        println!("{:?}", vec)
    }
}

