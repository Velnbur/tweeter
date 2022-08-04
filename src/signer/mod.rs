use std::sync::mpsc::Receiver;

use crate::db::Pool;
use crate::records::tweets::Tweet;

pub struct Singer {
    db: Pool,
    chan: Receiver<Tweet>,
}
