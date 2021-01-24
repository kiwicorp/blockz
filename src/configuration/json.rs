//! Json configuration.

use crate::configuration::Configuration;

use std::path::Path;

use serde::Deserialize;

/// Load configuration using a JSON file.
pub struct JsonConfiguration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{
    _inner: I,
    _opts: JsonConfigurationOpts<'c>,
}

pub enum JsonConfigurationOpts<'o> {
    File(&'o Path),
    // make is possible to load a json configuration from a url, using reqwest.
    // Url,
}

#[async_trait::async_trait]
impl<'c, I> Configuration for JsonConfiguration<'c, I>
where
    I: Send + for<'de> Deserialize<'de>,
{


    type Inner = I;

    type Opts = JsonConfigurationOpts<'c>;

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
