//! Configurations.

/// Common behaviour of configurations.
#[async_trait::async_trait]
pub trait Configuration {
    /// The inner type of the configuration that can be loaded.
    type Inner: Send;

    /// The type of options container this Configuration accepts for the purpose of loading the
    /// configuration.
    type Opts: Send;

    /// The error type that can be produced while loading the configuration.
    type Error: Send;

    /// Load the configuration.
    async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error>;
}

/// An easy configuration is a configuration that can be loaded without other parameters.
#[async_trait::async_trait]
pub trait EasyConfiguration {
    /// The inner type of the configuration that can be loaded.
    type Inner: Send;

    /// The error type that can be produced while loading the configuration.
    type Error: Send;

    /// Load the configuration.
    async fn load() -> Result<Self::Inner, Self::Error>;
}

/// Automatically implement EasyConfiguration on Configuration implementations whose Opts type is
/// Default.
#[async_trait::async_trait]
impl<C, I, O, E> EasyConfiguration for C
where
    C: Configuration<Inner=I, Opts=O, Error=E>,
    I: Send + 'static,
    O: Default + Send + 'static,
    E: Send + 'static,
{
    type Inner = I;
    type Error = E;

    async fn load() -> Result<Self::Inner, Self::Error> {
        C::load(O::default()).await
    }
}
