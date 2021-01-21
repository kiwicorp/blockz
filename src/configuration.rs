//! Configurations.

/// Common behaviour of configurations.
#[async_trait::async_trait]
pub trait Configuration {
    /// The inner type of the configuration that can be loaded.
    type Inner: Send;

    /// Load the configuration.
    async fn load() -> anyhow::Result<Self::Inner>;
}
