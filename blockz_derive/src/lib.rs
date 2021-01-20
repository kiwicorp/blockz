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
