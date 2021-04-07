//! Env configuration factory.

use darling::FromMeta;

use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;
use syn::Expr;

use crate::common;
use crate::factory::ReusableFactory;
use crate::paths;
use crate::ProcMacroErrorExt;

use super::ConfigurationOpts;
use super::DynFactory;
use super::DynFactoryProduct;

/// Factory that builds a Configuration implementation based on envy.
pub(super) struct EnvConfigurationFactory {
    input: DeriveInput,
    opts: EnvConfigurationOpts,
}

/// Configuration options for a configuration backed by envy.
#[derive(Default, FromMeta)]
pub(super) struct EnvConfigurationOpts {
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    prefix_source: Option<String>,
}

impl EnvConfigurationFactory {
    /// Create a new envy configuration factory.
    pub fn new_dyn(input: DeriveInput, opts: &mut ConfigurationOpts) -> DynFactory {
        let env = opts.env.take().unwrap_or_default();
        Box::new(Self { input, opts: env })
    }

    fn get_configuration_impl_opts(&self) -> TokenStream {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let type_name = &self.input.ident;
        let default_opts = quote! {
            <#blockz::configuration::EnvConfiguration<#type_name> as #blockz::configuration::Configuration>::Opts
        };

        if self.opts.prefix.is_some() || self.opts.prefix_source.is_some() {
            quote! { () }
        } else {
            default_opts
        }
    }

    fn get_configuration_impl_load_arg(&self) -> Result<TokenStream, syn::Error> {
        let tokens: TokenStream = if let Some(prefix) = &self.opts.prefix {
            let lit_prefix = common::create_lit_str(prefix.clone());
            quote! { Some(#lit_prefix.to_string()) }
        } else if let Some(prefix_source) = &self.opts.prefix_source {
            let expr: Expr = syn::parse_str(prefix_source).map_err(|err: syn::Error| {
                syn::Error::new(
                    err.span(),
                    format!("failed to parse prefix source tokens: {}", err.to_string()),
                )
            })?;
            quote! { Some(#expr) }
        } else {
            quote! { opts }
        };
        Ok(tokens)
    }
}

impl ReusableFactory for EnvConfigurationFactory {
    type Product = DynFactoryProduct;

    /// Build the envy configuration trait implementation.
    fn build(&mut self) -> Self::Product {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        let opts = self.get_configuration_impl_opts();
        let load_arg = self.get_configuration_impl_load_arg().map_err(|err| {
            let err: Box<dyn ProcMacroErrorExt> = Box::new(err);
            err
        })?;

        // return the implementation
        let type_name = &self.input.ident;
        Ok(quote! {
            #[automatically_derived]
            #[async_trait::async_trait]
            impl #blockz::configuration::Configuration for #type_name {
                type Opts = #opts;
                type Result = <#blockz::configuration::EnvConfiguration<#type_name> as #blockz::configuration::Configuration>::Result;

                async fn load(opts: Self::Opts) -> Self::Result {
                    <#blockz::configuration::EnvConfiguration<#type_name> as #blockz::configuration::Configuration>::load(#load_arg).await
                }
            }
        })
    }
}
