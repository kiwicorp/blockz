//! Envy configuration factory.

use darling::FromMeta;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use crate::common;
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
    prefix: Option<String>,
}

impl<'f> EnvyConfigurationFactory<'f> {
    /// Create a new envy configuration factory.
    pub fn new(type_name: &'f Ident, opts: &'f EnvyConfigurationOpts) -> Self {
        Self { type_name, opts }
    }

    fn get_configuration_impl_opts(&self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let type_name = self.type_name;
        let default_opts = quote! {
            <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Opts
        };

        if let Some(_) = &self.opts.prefix {
            quote! { () }
        } else {
            default_opts
        }
    }

    fn get_configuration_impl_load_arg(&self) -> TokenStream {
        if let Some(prefix) = &self.opts.prefix {
            let lit_prefix = common::create_lit_str(prefix.clone());
            quote! { Some(#lit_prefix.to_string()) }
        } else {
            quote! { opts }
        }
    }

    /// Build the envy configuration trait implementation.
    pub fn build(self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let opts = self.get_configuration_impl_opts();
        let load_arg = self.get_configuration_impl_load_arg();

        // return the implementation
        let type_name = self.type_name;
        quote! {
            #[async_trait::async_trait]
            impl #blockz::configuration::Configuration for #type_name {
                type Inner = #type_name;
                type Opts = #opts;
                type Error = <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Error;

                async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
                    <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::load(#load_arg).await
                }
            }
        }
    }
}
