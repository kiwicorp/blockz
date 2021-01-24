//! Configurations.

/// Common behaviour of configurations.
#[async_trait::async_trait]
pub trait Configuration {
    /// The inner type of the configuration that can be loaded.
    type Inner: Send;

    /// The type of options container this Configuration accepts for the purpose of loading the
    /// configuration.
    type Opts: Send;

    /// Load the configuration.
    async fn load(opts: Self::Opts) -> anyhow::Result<Self::Inner>;
}

#[cfg(feature = "env")]
mod env;

#[cfg(feature = "json")]
mod json;
