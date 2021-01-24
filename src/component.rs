//! Components.

/// A set of functions every component should implement.
#[async_trait::async_trait]
pub trait ComponentExt {
    type Inner;
    type Config: Sync;

    /// Initialize the component.
    async fn init(config: &Self::Config) -> anyhow::Result<Self::Inner>;

    /// Start the component.
    async fn start(&mut self, config: &Self::Config) -> anyhow::Result<()> {
        Ok(())
    }

    /// Stop the component.
    async fn stop(&mut self, config: &Self::Config) -> anyhow::Result<()> {
        Ok(())
    }

    /// Deinitialize the component.
    async fn deinit(&mut self, config: &Self::Config) -> anyhow::Result<()>;
}

/// A component.
pub enum Component<C>
where
    C: ComponentExt + 'static,
{
    Cold(ColdComponent<C>),
    Initializing(InitializingComponent<C>),
    Starting(StartingComponent<C>),
    Running(RunningComponent<C>),
    Stopping(StoppingComponent<C>),
    Deinitializing(DeinitializingComponent<C>),
    Failed(FailedComponent<C>),
}

/// A component that awaits to be initialized.
pub struct ColdComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A component that can be initialized.
pub struct InitializingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// An initialized component that can be started.
pub struct StartingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A started component.
pub struct RunningComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A started component that can be stopped.
pub struct StoppingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A stopped component that can be deinitialized.
pub struct DeinitializingComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}

/// A component that failed in another stage.
pub struct FailedComponent<C>
where
    C: ComponentExt,
{
    inner: C,
}
