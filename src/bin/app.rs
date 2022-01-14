//! This is the main file for WickedSpeech.

use tokio::signal;
use tracing::info;
use wickedspeech::app;

#[tokio::main]
async fn main() -> wickedspeech::Result<()> {
    tracing_subscriber::fmt::try_init()?;

    info!("Initializing WickedSpeech");
    app::run(signal::ctrl_c()).await;

    Ok(())
}
