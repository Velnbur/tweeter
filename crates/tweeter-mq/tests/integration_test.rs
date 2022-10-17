mod common;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tweeter_mq::rabbit::RabbitChannel;
use tweeter_mq::Channel;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub msg: String,
}

#[tokio::test]
async fn test_one_publish_consume() {
    let channel = common::setup_rabbit().await;

    let q_name = String::from_str(common::QUEUE_NAME).unwrap();

    let rabbit = RabbitChannel::new(q_name, channel);

    let msg = String::from_str("Hello, world!").unwrap();

    let payload = Payload { msg: msg.clone() };

    rabbit.publish(payload).await.unwrap();

    rabbit
        .consume(move |entry: Payload| {
            assert_eq!(entry.msg, msg);
            Ok(())
        })
        .await
        .unwrap();
}
