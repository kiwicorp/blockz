//! Support for common configuration behaviour and loading them from various sources.
//!
//! # Example
//!
//! This example shows how to automatically derive Configuration on a struct. This particular
//! example will showcase loading the configuration from environment variables with a particular
//! prefix.
//!
//! **NOTE**: This example requires the __env_configuration__ feature.
//!
//! ```
//! use blockz::prelude::*;
//!
//! use serde::Deserialize;
//!
//! #[derive(Configuration, Deserialize)]
//! #[configuration(env(prefix = "COOL_APP_"))]
//! struct ServerConfig {
//!     #[serde(rename = "bind_addr")]
//!     address: String,
//!     #[serde(rename = "bind_port")]
//!     port: u16,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     # std::env::set_var("COOL_APP_BIND_ADDR", "0.0.0.0");
//!     # std::env::set_var("COOL_APP_BIND_PORT", "58732");
//!     let config = <ServerConfig as EasyConfiguration>::load()
//!         .await
//!         .expect("Failed to load configuration from the environment!");
//!     # assert_eq!(config.address.as_str(), "0.0.0.0");
//!     # assert_eq!(config.port, 58732);
//!     println!("Server binding to {}:{}.", config.address, config.port);
//!     // Server binding to 0.0.0.0:58732.
//! }
//! ```


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
