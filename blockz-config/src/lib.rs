//! Convenient application configuration.

use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

use serde::de::DeserializeOwned;

/// Result type for app configurations.
pub type Result<T> = std::result::Result<T, AppConfigError<T>>;

/// An application configuration.
pub trait AppConfig: DeserializeOwned + Debug {
    const PACKAGE: &'static str;

    /// Load the configuration.
    fn load() -> Result<Self> {
        let provider = figment::providers::Env::prefixed(Self::PACKAGE);
        match figment::Figment::new().join(provider).extract() {
            Ok(value) => Ok(value),
            Err(e) => Err(AppConfigError::new(e)),
        }
    }
}

/// Error that can be returned by the `AppConfig` trait.
#[derive(Debug)]
pub struct AppConfigError<T> {
    inner: figment::Error,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> AppConfigError<T> {
    pub(crate) fn new(inner: figment::Error) -> Self {
        Self {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: AppConfig> Display for AppConfigError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!(
                "{}: app config error: {:#}",
                T::PACKAGE,
                self.inner
            ))
        } else {
            f.write_fmt(format_args!(
                "{}: app config error: {}",
                T::PACKAGE,
                self.inner
            ))
        }
    }
}

impl<T: AppConfig> Error for AppConfigError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}
