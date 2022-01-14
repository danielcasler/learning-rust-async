pub mod actor;
pub use actor::Actor;

pub mod custom_layer;
pub use custom_layer::CustomLayer;

pub mod launcher;
pub use launcher::Launcher;

pub mod app;

pub mod mock_actor;
pub use mock_actor::MockActor;

pub mod shutdown;
pub use shutdown::Shutdown;

pub mod supervisor;
pub use supervisor::Supervisor;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
