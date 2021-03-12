//! Errors.

use proc_macro2::TokenStream;

/// Common error behaviour expected from proc macro crates.
pub(crate) trait ProcMacroError {
    /// Convert the error to the appropriate compile error(s).
    fn to_compile_errors(self) -> TokenStream;
}

impl ProcMacroError for syn::Error {
    fn to_compile_errors(self) -> TokenStream {
        self.to_compile_error()
    }
}

impl ProcMacroError for darling::Error {
    fn to_compile_errors(self) -> TokenStream {
        self.write_errors()
    }
}
