//! Components.

/// A set of functions every component should implement.
#[async_trait::async_trait]
pub trait ComponentExt {
    /// The inner type of the component.
    ///
    /// Can be () in case the component itself should not be actually returned.
    type Inner: Send;

    /// The configuration type used by this component.
    type Config: Send;

    /// Initialize the component.
    async fn init(config: &Self::Config) -> anyhow::Result<Self::Inner>;

    /// Start the component.
    async fn start(&mut self, config: &Self::Config) -> anyhow::Result<()>;

    /// Stop the component.
    async fn stop(&mut self, config: &Self::Config) -> anyhow::Result<()>;

    /// Deinitialize the component.
    async fn deinit(&mut self, config: &Self::Config) -> anyhow::Result<()>;
}
