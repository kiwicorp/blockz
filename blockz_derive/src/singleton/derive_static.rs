//! Implement the singleton static.

use crate::common;
use crate::paths;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

/// Factory that builds a singleton static.
pub(super) struct SingletonStaticFactory<'f> {
    static_ident: &'f Ident,
    type_name: &'f Ident,
}

impl<'f> SingletonStaticFactory<'f> {
    /// Create a new singleton static factory.
    pub fn new(static_ident: &'f Ident, type_name: &'f Ident) -> Self {
        Self {
            static_ident,
            type_name,
        }
    }

    /// Build the singleton static.
    pub fn build(self) -> syn::Result<TokenStream> {
        // get paths to deps
        let once_cell = paths::once_cell_path();
        let tokio = paths::tokio_path();

        // create doc comment
        let doc = common::create_doc(format!("Singleton for {}.", self.type_name));

        // return implementation
        let static_ident = self.static_ident;
        let type_name = self.type_name;
        Ok(quote! {
            #doc
            #[automatically_derived]
            static #static_ident: #once_cell::sync::OnceCell<#tokio::sync::Mutex<#type_name>> =
                #once_cell::sync::OnceCell::new();
        })
    }
}
