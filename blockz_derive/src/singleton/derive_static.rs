//! Implement the singleton static.

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::quote;

use crate::common;
use crate::factory::Factory;
use crate::paths;

use super::lock::SingletonLock;

/// Factory that builds a singleton static.
pub(super) struct SingletonStaticFactory<'f> {
    static_ident: &'f Ident,
    type_name: &'f Ident,
    lock: &'f SingletonLock,
}

impl<'f> SingletonStaticFactory<'f> {
    /// Create a new singleton static factory.
    pub fn new(static_ident: &'f Ident, type_name: &'f Ident, lock: &'f SingletonLock) -> Self {
        Self {
            static_ident,
            type_name,
            lock,
        }
    }
}

impl<'f> Factory for SingletonStaticFactory<'f> {
    type Product = syn::Result<TokenStream>;

    /// Build the singleton static.
    fn build(self) -> Self::Product {
        // get paths to deps
        let once_cell = paths::once_cell_path();

        // create doc comment
        let doc = common::create_doc(format!("Singleton for {}.", self.type_name));

        // get lock type
        let lock_type = self.lock.to_type(self.type_name);

        // return implementation
        let static_ident = self.static_ident;
        Ok(quote! {
            #doc
            #[automatically_derived]
            static #static_ident: #once_cell::sync::OnceCell<#lock_type> =
                #once_cell::sync::OnceCell::new();
        })
    }
}
