//! #[derive(Configuration)].

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use syn::LitStr;

#[cfg(feature = "no_absolute_paths")]
pub fn impl_configuration_trait(type_name: &Ident, prefix: Option<LitStr>) -> TokenStream {
    let type_name_str = type_name.to_string();
    let environment_source = if let Some(value) = prefix {
        quote! {
            config::Environment::with_prefix(#value)
        }
    } else {
        quote! {
            config::Environment::new()
        }
    };
    quote! {
        #[async_trait::async_trait]
        impl blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;

            /// Load the #type_name configuration.
            async fn load() -> anyhow::Result<Self::Inner> {
                let config_raw = {
                    let mut config_raw = config::Config::default();
                    config_raw.merge(#environment_source)?;
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
pub fn impl_configuration_trait(type_name: &Ident, prefix: Option<LitStr>) -> TokenStream {
    let type_name_str = type_name.to_string();
    let environment_source = if let Some(value) = prefix {
        quote! {
            config::Environment::with_prefix(#value)
        }
    } else {
        quote! {
            config::Environment::new()
        }
    };
    quote! {
        #[async_trait::async_trait]
        impl ::blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;

            /// Load the #type_name configuration.
            async fn load() -> ::anyhow::Result<Self::Inner> {
                let config_raw = {
                    let mut config_raw = config::Config::default();
                    config_raw.merge(#environment_source)?;
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
