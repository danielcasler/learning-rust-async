use crate::Shutdown;

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
        let (actor_notify_error, _) = mpsc::unbounded_channel::<String>();
        let (actor_notify_shutdown, _) = broadcast::channel::<()>(1);
        // let (actor_shutdown_complete_tx, actor_shutdown_complete_rx) = mpsc::channel::<()>(1);
        // p.start();
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

    // pub async fn launch_long<T: Process>(mut self, _: T) {
    //     // p.start();
    //     tokio::spawn(async move {
    //         let _ = self.shutdown_complete_tx.clone();
    //         thread::sleep(time::Duration::from_secs(20));
    //         info!("FINISHED SLEEPING FROM Supervisor LONG");
    //     });

    //     info!("WAITING FOR SIGNAL");

    //     // Wait for shutdown
    //     tokio::select! {
    //         // todo: WAIT ON ERRORS
    //         // todo: WAIT ON RESTART
    //         _ = self.shutdown.recv() => {
    //             info!("operation timed out");
    //         }
    //     }
    //     info!("GOT HERE");
    // }
}
