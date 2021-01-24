//! Env configuration.

use serde::Deserialize;

/// Options accepted by env configuration.
pub type Opts<'s> = Option<&'s str>;

/// Load configuration using envy.
pub struct Configuration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    _inner: I,
    _opts: Option<&'c str>,
}

#[async_trait::async_trait]
impl<'c, I> crate::configuration::Configuration for Configuration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    type Inner = I;
    type Opts = Opts<'c>;

    async fn load(opts: Self::Opts) -> anyhow::Result<Self::Inner> {
        if let Some(prefix) = opts {
            Ok(envy::prefixed(prefix).from_env::<Self::Inner>()?)
        } else {
            Ok(envy::from_env::<Self::Inner>()?)
        }
    }
}
