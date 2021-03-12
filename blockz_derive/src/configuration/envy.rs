//! Envy configuration factory.

use darling::FromMeta;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use crate::paths;

/// Factory that builds a Configuration implementation based on envy.
pub(super) struct EnvyConfigurationFactory<'f> {
    type_name: &'f Ident,
    opts: &'f EnvyConfigurationOpts, // fixme 12/03/21: use the factory opts
}

/// Configuration options for a configuration backed by envy.
#[derive(Default, FromMeta)]
pub(super) struct EnvyConfigurationOpts {
    #[darling(default)]
    prefix: Option<String>, // fixme 12/03/21: implement the prefix option
    #[darling(default)]
    prefix_source: Option<String>, // fixme 12/03/21: implement the prefix_source option
    #[darling(default)]
    default_prefix: Option<String>, // fixme 12/03/21: implement the default_prefix option
    #[darling(default)]
    default_prefix_source: Option<String>, // fixme 12/03/21: implement the default_prefix_source option
}

impl<'f> EnvyConfigurationFactory<'f> {
    /// Create a new envy configuration factory.
    pub fn new(type_name: &'f Ident, opts: &'f EnvyConfigurationOpts) -> Self {
        Self { type_name, opts }
    }

    /// Build the envy configuration trait implementation.
    pub fn build(self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        // return the implementation
        let type_name = self.type_name;
        quote! {
            #[async_trait::async_trait]
            impl #blockz::configuration::Configuration for #type_name {
                type Inner = #type_name;
                type Opts = <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Opts;
                type Error = <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Error;

                async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
                    <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::load(opts).await
                }
            }
        }
    }
}
