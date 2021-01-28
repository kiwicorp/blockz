//! Blockz derive.

#[macro_use]
extern crate quote;

mod configuration;
mod paths;
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

/// Derive the Configuration trait.
///
/// All fields shall be loaded from environment variables, at the moment.
///
/// This requires that the struct or enum is [Deserialize].
///
/// Required available imports:
/// - [anyhow]
/// - [async_trait]
/// - [blockz]
/// - [config]
///
/// [Deserialize]: https://docs.rs/serde/1.0.120/serde/trait.Deserialize.html
/// [anyhow]: https://docs.rs/anyhow
/// [async_trait]: https://docs.rs/async_trait
/// [blockz]: https://github.com/selftechio/blockz
/// [config]: https://docs.rs/config
#[proc_macro_derive(Configuration, attributes(config))]
pub fn derive_configuration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    let impl_configuration = configuration::impl_configuration_trait(type_name);

    let expanded = quote! {
        #impl_configuration
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
