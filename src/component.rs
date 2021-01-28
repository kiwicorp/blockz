//! Components.

/// A set of functions every component should implement.
#[async_trait::async_trait]
pub trait ComponentExt<'c, 'p>
where
    'p: 'c,
{
    /// The inner type of the component that should be returned upon initialization.
    type Inner: Send + 'c;

    /// Parameters consumed during initialization.
    type InitParams: Send + 'p;

    /// Parameters consumed during the start process.
    type StartParams: Send + 'p;

    /// Parameters consumed during the stop process.
    type StopParams: Send + 'p;

    /// Parameters consumed during deinitialization.
    type DeinitParams: Send + 'p;

    /// Initialize the component.
    async fn init(config: Self::InitParams) -> anyhow::Result<Self::Inner>;

    /// Start the component.
    async fn start(&'c mut self, config: Self::StartParams) -> anyhow::Result<()>;

    /// Stop the component.
    async fn stop(&'c mut self, config: Self::StopParams) -> anyhow::Result<()>;

    /// Deinitialize the component.
    async fn deinit(&'c mut self, config: Self::DeinitParams) -> anyhow::Result<()>;
}
