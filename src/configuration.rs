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
    C: Configuration<Inner = I, Opts = O, Error = E>,
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

#[cfg(feature = "envy_configuration")]
mod envy_configuration {
    use super::Configuration;
    use serde::Deserialize;
    use std::marker::PhantomData;

    /// Configuration that can be sourced via envy.
    pub struct EnvyConfiguration<T>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        _phantom: PhantomData<T>,
    }

    #[cfg(feature = "envy_configuration")]
    #[async_trait::async_trait]
    impl<T> Configuration for EnvyConfiguration<T>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        type Inner = T;
        type Opts = Option<String>;
        type Error = envy::Error;

        async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
            if let Some(prefix) = opts {
                Ok(envy::prefixed(prefix).from_env::<Self::Inner>()?)
            } else {
                Ok(envy::from_env::<Self::Inner>()?)
            }
        }
    }
}
#[cfg(feature = "envy_configuration")]
pub use envy_configuration::*;
