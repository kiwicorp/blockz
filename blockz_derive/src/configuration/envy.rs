//! Envy configuration factory.

use darling::FromMeta;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use crate::paths;

/// Factory that builds a Configuration implementation based on envy.
pub(super) struct EnvyConfigurationFactory<'f> {
    type_name: &'f Ident,
}

/// Configuration options for a configuration backed by envy.
#[derive(FromMeta)]
pub(super) struct EnvyConfigurationOpts {
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    prefix_source: Option<String>,
    #[darling(default)]
    default_prefix: Option<String>,
    #[darling(default)]
    default_prefix_source: Option<String>,
}

impl<'f> EnvyConfigurationFactory<'f> {
    /// Create a new envy configuration factory.
    pub fn new(type_name: &'f Ident) -> Self {
        Self { type_name }
    }

    /// Build the envy configuration trait implementation.
    pub fn build(self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();
        let envy = paths::envy_path();

        // return the implementation
        let type_name = self.type_name;
        quote! {
            #[async_trait::async_trait]
            impl #blockz::configuration::Configuration for #type_name {
                type Inner = #type_name;
                type Opts = #blockz::configuration::<EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Opts;
                type Error = #blockz::configuration::<EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Error;

                async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
                    #blockz::configuration::<EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::load(opts)
                }
            }
        }
    }
}
