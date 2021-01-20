//! Blockz derive.

#[macro_use]
extern crate quote;

mod singleton;

use convert_case::Case;
use convert_case::Casing;

use proc_macro2::Ident;
use proc_macro2::Span;

use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::DeriveInput;

/// Derive the Singleton trait.
///
/// This requires that the struct or enum is [Send].
///
/// Required available imports:
/// - [anyhow]
/// - [async_trait]
/// - [blockz]
/// - [once_cell]
/// - [tokio]
///
/// [Send]: https://doc.rust-lang.org/stable/std/marker/trait.Send.html
/// [anyhow]: https://docs.rs/anyhow
/// [async_trait]: https://docs.rs/async_trait
/// [blockz]: https://github.com/selftechio/blockz
/// [once_cell]: https://docs.rs/once_cell
/// [tokio]: https://docs.rs/tokio
#[proc_macro_derive(Singleton)]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let upper_snake = {
        let original = format!("{}", &input.ident);
        original.to_case(Case::UpperSnake)
    };
    let singleton_name = &Ident::new(upper_snake.as_str(), Span::call_site());
    let type_name = &input.ident;

    let impl_singleton = singleton::impl_singleton_trait(type_name, singleton_name);
    let singleton_static = singleton::impl_singleton_static(type_name, singleton_name);

    let expanded = quote! {
        #singleton_static
        #impl_singleton
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();

        t.pass("test/basic_derive.rs");
    }
}
