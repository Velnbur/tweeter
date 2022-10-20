mod common;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use tweeter_mq::rabbit::RabbitChannel;
use tweeter_mq::{Consumer, Producer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub msg: String,
}

const MSG: &'static str = "Hello, world!";

#[derive(Error, Debug)]
enum Error {
    #[error("test messgae")]
    #[warn(dead_code)]
    Err,
}

#[tokio::test]
async fn test_one_publish_consume() {
    let channel = common::setup_rabbit().await;

    let q_name = String::from_str(common::QUEUE_NAME).unwrap();

    let rabbit = RabbitChannel::new(q_name, channel);

    let msg = String::from_str(MSG).unwrap();
    let payload = Payload { msg: msg.clone() };

    rabbit.publish(payload).await.unwrap();

    let payload: Payload = rabbit.get().await.unwrap().unwrap();
}
