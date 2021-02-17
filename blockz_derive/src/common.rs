//! Common functions.

use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::LitStr;

/// Create a doc attribute.
pub fn create_doc(doc: String) -> TokenStream {
    create_lit_str(format!(" {}", doc));
    quote! {
        #[doc = #doc]
    }
}

/// Create a LitStr.
///
/// Useful when called with a format!() macro as an arg.
pub fn create_lit_str(string: String) -> LitStr {
    LitStr::new(string.as_str(), Span::call_site())
}
