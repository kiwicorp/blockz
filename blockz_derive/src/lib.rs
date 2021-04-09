//! Blockz derive.

#![cfg_attr(docsrs, feature(doc_cfg))]

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
/// This derive procedural macro accepts an attribute: `#[singleton]`. The attribute may configure
/// the underlying lock used for securing access to the singleton. Valid values:
///
/// - `#[singleton(lock = "mutex")]` _(default)_
/// - `#[singleton(lock = "rwlock")]`
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
/// The input function must have the following properties:
///
/// - the first argument must be a reference receiver (&self or &mut self)
/// - function arguments identifiers must not conflict with other identifiers from the function body
///   (such as other function names in function calls, struct fields etc)
/// - references are not allowed (use Box\<T\> or Arc\<T\> instead)
#[proc_macro_attribute]
#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
pub fn singleton_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    factory!(SingletonFnFactory::new(&input))
}

/// Derive the Configuration trait.
///
/// This requires that the struct or enum is [Send].
///
/// Configuring the implementation is done via the `#[configuration]` attribute:
///
/// - by default, direct configuration will be implemented, if no other configuration feature is
///   enabled
/// - direct configuration: `#[configuration(direct)]`
/// - env configuration with no prefix: `#[configuraton(env())]`
/// - env configuration with a prefix: `#[configuraton(env(prefix = "MY_PREFIX"))]`
/// - env configuration with a prefix source: `#[configuraton(env(prefix_source = "MY_SOURCE"))]`
///
/// The prefix source will be interpreted as an expression that will be used to source the prefix.
/// You can use either constants or functions.
///
/// Required available imports:
/// - [anyhow]
/// - [async_trait]
/// - [blockz]
/// - [config]
///
/// [Send]: https://doc.rust-lang.org/stable/std/marker/trait.Send.html
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
