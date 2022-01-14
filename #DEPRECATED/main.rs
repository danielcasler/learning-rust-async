//! This is the main file for WickedSpeech.

#![allow(dead_code)]

use color_eyre::Report;
use crossbeam::sync::WaitGroup;
use tokio::signal;
use tokio::sync::{broadcast, mpsc};

mod core;
use crate::core::Core;

mod launcher;
use launcher::{Launcher, MockProcess};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::try_init()?;
    let core = Core::new("Test App");
    println!("APP {}", core.name());

    let (error_tx, mut error_rx) = mpsc::unbounded_channel::<String>();
    let (shutdown_tx, shutdown_rx) = broadcast::channel::<bool>(16);
    let wg = WaitGroup::new();

    let launcher = Launcher::new(error_tx, &shutdown_tx, &wg);
    // let wg = launcher.clone_wg();

    // let (error_tx, mut error_rx) = launcher.channels.error;

    println!("Hello from a (so far completely unnecessary) async runtime");

    let process = MockProcess::new();

    launcher.launch(process);

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("ATTEMPTING SHUTDOWN");
            wg.wait();
            println!("Shutdown successful!")
        },
        _ = error_rx.recv() => {},
    }
    Ok(())
}
