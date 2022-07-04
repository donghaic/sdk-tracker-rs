use std::error::Error;
use std::time::Duration;

use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use rdkafka::error::KafkaError;
use rdkafka::message::{OwnedHeaders, OwnedMessage};

pub struct KafkaProducer(FutureProducer);

pub type Result<T, E = dyn Error> = std::result::Result<T, E>;


impl KafkaProducer {
    pub fn new(brokers: String) -> KafkaProducer {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Can't create the producer");

        KafkaProducer(producer)
    }

    pub async fn publish_message(
        &self,
        topic: &str,
        payload: Vec<u8>,
    ) -> Result<(), KafkaError> {
        let delivery_status = self.0.send(
            FutureRecord::to(topic).key("").payload(&payload), Duration::from_secs(5),
        );
        let result = delivery_status.await;
        match result {
            Ok(_) => {}
            Err(e) => println!("{:?}", e)
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use rdkafka::message::ToBytes;

    use super::*;

    #[actix_rt::test]
    async fn test_kafka() {
        let producer = KafkaProducer::new(String::from("localhost:9092"));
        let result = producer.publish_message("test", Vec::from("hello".to_bytes())).await;
    }
}