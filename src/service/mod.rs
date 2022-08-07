mod api;
mod signer;

use signer::Singer;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{config, records::tweets::Tweet};

pub async fn run(cfg: &config::Config) {
    let (sender, receiver): (Sender<Tweet>, Receiver<Tweet>) = mpsc::channel(1000);

    let mut signer = Singer::new(receiver, cfg.db.clone());
    tokio::spawn(async move {
        signer.start().await;
    });

    api::run(cfg, sender).await;
}
