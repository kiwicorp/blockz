//! Env configuration.

use crate::configuration::Configuration;

use serde::Deserialize;

/// Load configuration using envy.
pub struct EnvConfiguration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    _inner: I,
    _opts: Option<&'c str>,
}

#[async_trait::async_trait]
impl<'c, I> Configuration for EnvConfiguration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    type Inner = I;
    type Opts = Option<&'c str>;

    async fn load(opts: Self::Opts) -> anyhow::Result<Self::Inner> {
        if let Some(prefix) = opts {
            Ok(envy::prefixed(prefix).from_env::<Self::Inner>()?)
        } else {
            Ok(envy::from_env::<Self::Inner>()?)
        }
    }
}
