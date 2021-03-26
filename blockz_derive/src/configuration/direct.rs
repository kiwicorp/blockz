//! Direct configuration factory.

use proc_macro2::TokenStream;

use quote::quote;

use syn::DeriveInput;

use crate::factory::ReusableFactory;
use crate::paths;

use super::ConfigurationOpts;
use super::DynFactory;

/// Factory that builds a Configuration implementation based on DirectConfiguration.
pub(super) struct DirectConfigurationFactory {
    input: DeriveInput,
}

impl DirectConfigurationFactory {
    /// Create a new direct configuration factory.
    pub fn new_dyn(input: DeriveInput, _: &mut ConfigurationOpts) -> DynFactory {
        Box::new(Self { input })
    }
}

impl ReusableFactory for DirectConfigurationFactory {
    type Product = TokenStream;

    fn build(&mut self) -> Self::Product {
        // gather paths to dependencies
        let blockz = paths::blockz_path();

        // return the implementation
        let type_name = &self.input.ident;
        quote! {
            #[automatically_derived]
            #[async_trait::async_trait]
            impl #blockz::configuration::Configuration for #type_name {
                type Inner = #type_name;
                type Opts = #type_name;
                type Error = <#blockz::configuration::DirectConfiguration<#type_name> as #blockz::configuration::Configuration>::Error;

                async fn load(opts: Self::Opts) -> Result<Self::Inner, Self::Error> {
                    <#blockz::configuration::DirectConfiguration<#type_name> as #blockz::configuration::Configuration>::load(opts).await
                }
            }
        }
    }
}
