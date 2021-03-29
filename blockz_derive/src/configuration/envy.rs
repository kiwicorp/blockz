//! Envy configuration factory.

use darling::FromMeta;

use proc_macro2::TokenStream;

use quote::quote;
use syn::DeriveInput;

use crate::common;
use crate::factory::ReusableFactory;
use crate::paths;

use super::ConfigurationOpts;
use super::DynFactory;

/// Factory that builds a Configuration implementation based on envy.
pub(super) struct EnvyConfigurationFactory {
    input: DeriveInput,
    opts: EnvyConfigurationOpts,
}

/// Configuration options for a configuration backed by envy.
#[derive(Default, FromMeta)]
pub(super) struct EnvyConfigurationOpts {
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    prefix_source: Option<String>,
}

impl EnvyConfigurationFactory {
    /// Create a new envy configuration factory.
    pub fn new_dyn(input: DeriveInput, opts: &mut ConfigurationOpts) -> DynFactory {
        let envy = opts.envy.take().unwrap_or_default();
        Box::new(Self { input, opts: envy })
    }

    fn get_configuration_impl_opts(&self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let type_name = &self.input.ident;
        let default_opts = quote! {
            <#blockz::configuration::EnvyConfiguration<#type_name> as #blockz::configuration::Configuration>::Opts
        };

        if self.opts.prefix.is_some() {
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
}

impl ReusableFactory for EnvyConfigurationFactory {
    type Product = TokenStream;

    /// Build the envy configuration trait implementation.
    fn build(&mut self) -> Self::Product {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let opts = self.get_configuration_impl_opts();
        let load_arg = self.get_configuration_impl_load_arg();

        // return the implementation
        let type_name = &self.input.ident;
        quote! {
            #[automatically_derived]
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
