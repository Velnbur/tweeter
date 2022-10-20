mod raw;

use tweeter_config::config::Configer;

pub struct Config {
    pub db: sqlx::PgPool,
    pub mq: tweeter_mq::rabbit::RabbitChannel,
}

impl Configer for Config {
    type Raw = raw::Raw;
}
