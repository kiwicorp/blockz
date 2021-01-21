//! #[derive(Configuration)].

use proc_macro2::Ident;
use proc_macro2::TokenStream;

#[cfg(feature = "no_absolute_paths")]
pub fn impl_configuration_trait(type_name: &Ident) -> TokenStream {
    let type_name_str = type_name.to_string();
    quote! {
        #[async_trait::async_trait]
        impl blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;

            /// Load the #type_name configuration.
            async fn load() -> anyhow::Result<Self::Inner> {
                let config_raw = {
                    let mut config_raw = config::Config::default();
                    config_raw.merge(config::Environment::new())?;
                    config_raw
                };
                match config_raw.try_into() {
                    Ok(value) => Ok(value),
                    Err(e) => anyhow::bail!("{}: load: {}", #type_name_str, e),
                }
            }
        }
    }
}

#[cfg(not(feature = "no_absolute_paths"))]
pub fn impl_configuration_trait(type_name: &Ident) -> TokenStream {
    let type_name_str = type_name.to_string();
    quote! {
        #[async_trait::async_trait]
        impl ::blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;

            /// Load the #type_name configuration.
            async fn load() -> ::anyhow::Result<Self::Inner> {
                let config_raw = {
                    let mut config_raw = config::Config::default();
                    config_raw.merge(config::Environment::new())?;
                    config_raw
                };
                match config_raw.try_into() {
                    Ok(value) => Ok(value),
                    Err(e) => anyhow::bail!("{}: load: {}", #type_name_str, e),
                }
            }
        }
    }
}
