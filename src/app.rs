use crate::Launcher;
// use color_eyre::Report;
use std::future::Future;
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};

pub async fn run(shutdown: impl Future) {
    let (notify_error, _) = mpsc::unbounded_channel::<String>();
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut app = Launcher {
        notify_error,
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
    };

    tokio::select! {
        res = app.run() => {
            if let Err(err) = res {
                error!(cause = %err, "failed to launch supervisors");
            }
        }
        _ = shutdown => {
            info!("shutting down");
        }
    }

    // Extract the `shutdown_complete` receiver and transmitter
    // explicitly drop `shutdown_transmitter`. This is important, as the
    // `.await` below would otherwise never complete.
    let Launcher {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = app;

    // When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
    // receive the shutdown signal and can exit
    drop(notify_shutdown);
    // Drop final `Sender` so the `Receiver` below can complete
    drop(shutdown_complete_tx);

    // Wait for all active connections to finish processing.
    let _ = shutdown_complete_rx.recv().await;
}
