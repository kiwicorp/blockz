//! Errors.

use std::mem;

use proc_macro2::TokenStream;

/// Common error behaviour expected from proc macro crates.
pub(crate) trait ProcMacroErrorExt {
    /// Convert the error to the appropriate compile error(s).
    fn as_compile_errors(&mut self) -> TokenStream;
}

impl ProcMacroErrorExt for syn::Error {
    fn as_compile_errors(&mut self) -> TokenStream {
        self.to_compile_error()
    }
}

impl ProcMacroErrorExt for darling::Error {
    /// This renders the error as unusable!
    fn as_compile_errors(&mut self) -> TokenStream {
        // replace the original error with an empty one and consume the actual error
        // this renders the old error as unusable!
        mem::replace(self, darling::Error::custom("")).write_errors()
    }
}

impl ProcMacroErrorExt for proc_macro2::LexError {
    fn as_compile_errors(&mut self) -> TokenStream {
        quote::quote! { self }
    }
}
