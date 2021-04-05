//! Configurations.

#[cfg(feature = "env_configuration")]
use serde::Deserialize;

use std::future::Future;
use std::marker::PhantomData;

/// Common behaviour of configurations.
#[async_trait::async_trait]
pub trait Configuration {
    /// The type of options container this Configuration accepts for the purpose of loading the
    /// configuration.
    type Opts: Send;

    /// The result type that can be produced by loading the configuration.
    type Result: Send;

    /// Load the configuration.
    async fn load(opts: Self::Opts) -> Self::Result;
}

/// An easy configuration is a configuration that can be loaded without other parameters.
#[async_trait::async_trait]
pub trait EasyConfiguration {
    /// The result type that can be produced by loading the configuration.
    type Result: Send;

    /// Load the configuration.
    async fn load() -> Self::Result;
}

/// Automatically implement EasyConfiguration on Configuration implementations whose Opts type is
/// Default.
#[async_trait::async_trait]
impl<C, O, R> EasyConfiguration for C
where
    C: Configuration<Opts = O, Result = R>,
    O: Default + Send + 'static,
    R: Send + 'static,
{
    type Result = R;

    async fn load() -> Self::Result {
        C::load(O::default()).await
    }
}

/// Behaviour expected from a function that sources options used for loading a configuration.
// R: the return type of the function.
pub trait OptsSourceFn<R>
where
    R: Send,
{
    /// The return type of the function.
    type Return: Future<Output = R> + Send;

    fn call_once(self) -> Self::Return;
}

// R: the return type of the function.
// F: the function to be executed.
// Fr: the future produced by the function F.
impl<R, F, Fr> OptsSourceFn<R> for F
where
    F: FnOnce() -> Fr,
    Fr: Future<Output = R> + Send,
    R: Send,
{
    type Return = Fr;

    fn call_once(self) -> Self::Return {
        self()
    }
}

/// Direct configuration that just returns the passed value.
pub struct DirectConfiguration<T>
where
    T: Send,
{
    _phantom: PhantomData<T>,
}

#[async_trait::async_trait]
impl<T> Configuration for DirectConfiguration<T>
where
    T: Send,
{
    type Opts = T;
    type Result = T;

    async fn load(opts: Self::Opts) -> Self::Result {
        opts
    }
}

/// Configuration that can be sourced from environment variables.
#[cfg(feature = "env_configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "env_configuration")))]
pub struct EnvConfiguration<T>
where
    T: for<'de> Deserialize<'de> + Send,
{
    _phantom_t: PhantomData<T>,
}

#[cfg(feature = "env_configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "env_configuration")))]
#[async_trait::async_trait]
impl<T> Configuration for EnvConfiguration<T>
where
    T: for<'de> Deserialize<'de> + Send,
{
    type Opts = Option<String>;
    type Result = Result<T, envy::Error>;

    async fn load(opts: Self::Opts) -> Self::Result {
        if let Some(prefix) = opts {
            envy::prefixed(prefix).from_env::<T>()
        } else {
            envy::from_env::<T>()
        }
    }
}
