//! #[derive(Configuration)].

use crate::paths;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;

/// A factory that builds implementations for the Configuration trait.
pub(crate) struct ConfigurationFactory {}

/// The source for a configuration.
pub enum Source {
    /// Environment variables.
    Envy,
    /// A JSON file.
    Json,
    /// A TOML file.
    Toml,
    /// A YAML file.
    Yaml,
    /// Variables from the AWS Parameter Store.
    EnvyStore,
}

pub(crate) fn derive_configuration(input: DeriveInput) -> TokenStream {
    let type_name = &input.ident;

    let impl_configuration = impl_configuration_trait(type_name);

    quote! {
        #impl_configuration
    }
}

fn impl_configuration_trait(type_name: &Ident) -> TokenStream {
    let anyhow = paths::anyhow_path();
    let blockz = paths::blockz_path();
    let envy = paths::envy_path();
    quote! {
        #[async_trait::async_trait]
        impl #blockz::configuration::Configuration for #type_name {
            type Inner = #type_name;
            type Opts = Option<String>;

            async fn load(opts: Self::Opts) -> #anyhow::Result<Self::Inner> {
                if let Some(prefix) = opts {
                    Ok(#envy::prefixed(prefix).from_env::<Self::Inner>()?)
                } else {
                    Ok(#envy::from_env::<Self::Inner>()?)
                }
            }
        }
    }
}
