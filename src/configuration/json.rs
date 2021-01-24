//! Json configuration.

use std::path::Path;

use serde::Deserialize;

/// Options accepted by json configuration.
pub type Opts<'o> = JsonConfigurationOpts<'o>;

/// Load configuration using a JSON file.
pub struct Configuration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    _inner: I,
    _opts: JsonConfigurationOpts<'c>,
}

/// Json configuration options.
pub enum JsonConfigurationOpts<'o> {
    File(&'o Path),
    // make is possible to load a json configuration from a url, using reqwest.
    // Url,
}

#[async_trait::async_trait]
impl<'c, I> crate::configuration::Configuration for Configuration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    type Inner = I;

    type Opts = Opts<'c>;

    async fn load(opts: Self::Opts) -> anyhow::Result<Self::Inner> {
        // boilerplate
        match opts {
            JsonConfigurationOpts::File(_path) => {
                // load json from a file
                todo!();
            },
        }
    }
}
