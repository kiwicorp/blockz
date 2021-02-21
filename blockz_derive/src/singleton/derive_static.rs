//! Implement the singleton static.

use crate::common;
use crate::paths;

use convert_case::Case;
use convert_case::Casing;

use proc_macro2::Span;

use quote::quote;

use syn::Ident;

/// Prefix for the generated singleton static.
const SINGLETON_STATIC_PREFIX: &str = "BLOCKZ_SINGLETON_STATIC_";

/// Create the singleton static name.
pub(super) fn create_singleton_static_name(ident: &Ident) -> Ident {
    // convert the type name to upper snake case and add a prefix
    let upper_snake = {
        let original = format!("{}{}", SINGLETON_STATIC_PREFIX, ident);
        original.to_case(Case::UpperSnake)
    };
    Ident::new(upper_snake.as_str(), Span::call_site())
}

/// Implement the singleton static.
pub(super) fn impl_singleton_static(
    type_name: &Ident,
    singleton_name: &Ident,
) -> proc_macro2::TokenStream {
    // get paths to deps
    let once_cell = paths::once_cell_path();
    let tokio = paths::tokio_path();
    // create doc comment
    let doc = common::create_doc(format!("Singleton for {}.", type_name));
    // return implementation
    quote! {
        #doc
        #[automatically_derived]
        static #singleton_name: #once_cell::sync::OnceCell<#tokio::sync::Mutex<#type_name>> =
            #once_cell::sync::OnceCell::new();
    }
}
