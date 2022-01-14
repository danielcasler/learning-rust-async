use crate::{MockActor, Shutdown};

use std::{thread, time};
use tokio::sync::{broadcast, mpsc};
use tracing::info;

use crate::actor::Actor;

#[derive(Debug)]
pub struct Supervisor {
    pub notify_error: mpsc::UnboundedSender<String>,
    pub shutdown: Shutdown,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Supervisor {
    pub fn new(
        notify_error: mpsc::UnboundedSender<String>,
        shutdown: Shutdown,
        shutdown_complete_tx: mpsc::Sender<()>,
    ) -> Supervisor {
        Supervisor {
            notify_error,
            shutdown,
            shutdown_complete_tx,
        }
    }

    pub async fn launch<T: Actor>(mut self, actor: T) {
        tokio::spawn(async move {
            let _ = self.shutdown_complete_tx.clone();
            let run = actor.start().await;
        });

        info!("WAITING FOR SIGNAL");

        // Wait for shutdown
        tokio::select! {
            // todo: WAIT ON ERRORS
            // todo: WAIT ON RESTART

            _ = self.shutdown.recv() => {
                info!("operation timed out");
            }
        }
        info!("GOT HERE");
    }
}
