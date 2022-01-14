use async_trait::async_trait;

// Trait all actors must implement
#[async_trait]
pub trait Actor: Send {
    async fn start(&self); //you'll need an argument list here
    async fn stop(&self);
}
