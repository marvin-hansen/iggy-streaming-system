use crate::ImsDataIntegrationError;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsShutdownIntegration: Send)]
pub trait LocalImsShutdownIntegration {
    async fn shutdown(&self) -> Result<(), ImsDataIntegrationError>;
}
