//! Configurations.

use serde::Deserialize;

use std::marker::PhantomData;

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

    #[inline(always)]
    async fn load() -> Result<Self::Inner, Self::Error> {
        C::load(O::default()).await
    }
}

pub struct EnvyConfiguration<T>
where
    T: for<'de> Deserialize<'de> + Send,
{
    _phantom: PhantomData<T>,
}

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

mod envy_configuration {
    #[async_trait::async_trait]
    pub trait EnvyConfiguration {
        type Inner: for<'de> serde::Deserialize<'de> + Send + 'static;
        async fn load(prefix: Option<String>) -> Result<Self::Inner, envy::Error> {
            if let Some(value) = prefix {
                envy::prefixed(value).from_env::<Self::Inner>()
            } else {
                envy::from_env::<Self::Inner>()
            }
        }
    }

    #[async_trait::async_trait]
    impl<Ec> super::Configuration for Ec
    where
        Ec: EnvyConfiguration,
    {
        type Inner = Ec::Inner;
        type Opts = Option<String>;
        type Error = envy::Error;

        #[inline(always)]
        async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
            Ec::load(opts).await
        }
    }
}

mod json_file_configuration {
    #[async_trait::async_trait]
    pub trait JsonFileConfiguration {
        type Inner: for<'de> serde::Deserialize<'de> + Send + 'static;
        async fn load(prefix: ()) -> Result<Self::Inner, ()> {
            todo!();
        }
    }

    #[async_trait::async_trait]
    impl<Jfc> super::Configuration for Jfc
    where
        Jfc: JsonFileConfiguration,
    {
        type Inner = Jfc::Inner;
        type Opts = ();
        type Error = ();

        #[inline(always)]
        async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
            Jfc::load(opts).await
        }
    }
}
