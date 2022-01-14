// use super::process::Process;
use crate::Actor;
use async_trait::async_trait;
use std::{thread, time};
use tracing::info;

pub struct MockActor {}

impl MockActor {
    pub fn new() -> MockActor {
        MockActor {}
    }
}

#[async_trait]
impl Actor for MockActor {
    async fn start(&self) {
        thread::sleep(time::Duration::from_secs(20));
        info!("FINISHED SLEEPING FROM MOCK ACTOR");
    }
    async fn stop(&self) {
        println!("STOPPED");
    }
}
