//! Blockz derive.

#[cfg(feature = "configuration")]
mod configuration;

#[cfg(feature = "singleton")]
mod singleton;

mod common;
mod paths;

use singleton::SingletonFactory;
use singleton::SingletonFnFactory;

use proc_macro::TokenStream;

use quote::quote;

use syn::parse_macro_input;
use syn::DeriveInput;
use syn::ItemFn;

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
#[cfg(feature = "singleton")]
#[proc_macro_derive(Singleton, attributes(singleton))]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    SingletonFactory::new(&input)
        .build()
        .unwrap_or_else(to_compile_error)
        .into()
}

/// Modify a method on a type that implements the Singleton trait.
///
/// The modified method becomes a function that will use the underlying singleton.
///
/// Caveats: you may not name any function args as any other identifier found in it's
/// implementation.
#[cfg(feature = "singleton")]
#[proc_macro_attribute]
pub fn singleton_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    SingletonFnFactory::new(&input)
        .map_or_else(
            |err| to_compile_error(err),
            |factory| factory.build().unwrap_or_else(to_compile_error),
        )
        .into()
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
#[cfg(feature = "configuration")]
pub fn derive_configuration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(configuration::derive_configuration(input))
}

/// Map a syn error to a compile error.
fn to_compile_error(error: syn::Error) -> proc_macro2::TokenStream {
    let compile_error = error.to_compile_error();
    quote! { #compile_error }
}
