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
    // fixme 01/04/2021: replace unsafe code
    // contribute to https://github.com/TedDriggs/darling to add a way of writing errors, either add
    // a #[derive(Clone)] or change function to use a (mutable) reference instead
    #[allow(invalid_value)]
    fn as_compile_errors(&mut self) -> TokenStream {
        let err_cloned: darling::Error = mem::replace(self, unsafe { mem::zeroed() });
        err_cloned.write_errors()
    }
}

impl ProcMacroErrorExt for proc_macro2::LexError {
    fn as_compile_errors(&mut self) -> TokenStream {
        quote::quote! { self }
    }
}
