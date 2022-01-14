use crate::{MockActor, Shutdown, Supervisor};
use color_eyre::Report;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug)]
pub struct Launcher {
    pub notify_error: mpsc::UnboundedSender<String>,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Launcher {
    pub async fn run(&mut self) -> Result<(), Report> {
        // Construct supervisors
        // Spawn supervisors
        // S1
        let sup = Supervisor::new(
            self.notify_error.clone(),
            Shutdown::new(self.notify_shutdown.subscribe()),
            self.shutdown_complete_tx.clone(),
        );
        let s1 = tokio::spawn(async move {
            sup.launch(MockActor::new()).await;
        });

        // S2
        let sup = Supervisor::new(
            self.notify_error.clone(),
            Shutdown::new(self.notify_shutdown.subscribe()),
            self.shutdown_complete_tx.clone(),
        );
        let s2 = tokio::spawn(async move {
            sup.launch(MockActor::new()).await;
        });

        // S3
        let sup = Supervisor::new(
            self.notify_error.clone(),
            Shutdown::new(self.notify_shutdown.subscribe()),
            self.shutdown_complete_tx.clone(),
        );
        let s3 = tokio::spawn(async move {
            sup.launch(MockActor::new()).await;
        });

        let (_, _, _) = tokio::join!(s1, s2, s3);

        Ok(())
    }
}
