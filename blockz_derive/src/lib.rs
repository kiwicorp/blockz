//! Blockz derive.

#[cfg(feature = "configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
mod configuration;

#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
mod singleton;

mod common;
mod errors;
mod factory;
mod paths;

use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::DeriveInput;
use syn::ItemFn;

use self::configuration::ConfigurationFactory;
use self::errors::ProcMacroErrorExt;
use self::factory::Factory;
use self::singleton::SingletonFactory;
use self::singleton::SingletonFnFactory;

/// Use a factory to produce a token stream.
macro_rules! factory {
    ($new_factory_stmt: expr) => {
        $new_factory_stmt
            .map_or_else(
                |mut err| err.as_compile_errors(),
                |factory| {
                    factory
                        .build()
                        .unwrap_or_else(|mut err| err.as_compile_errors())
                },
            )
            .into()
    };
}

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
#[proc_macro_derive(Singleton, attributes(singleton))]
#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    factory!(SingletonFactory::new(&input))
}

/// Modify a method on a type that implements the Singleton trait.
///
/// The modified method becomes a function that will use the underlying singleton.
///
/// Caveats: you may not name any function args as any other identifier found in it's
/// implementation.
#[proc_macro_attribute]
#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
pub fn singleton_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    factory!(SingletonFnFactory::new(&input))
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
#[proc_macro_derive(Configuration, attributes(configuration))]
#[cfg(feature = "configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
pub fn derive_configuration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    factory!(ConfigurationFactory::new(input))
}
