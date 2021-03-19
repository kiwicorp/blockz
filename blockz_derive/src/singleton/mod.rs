//! Singleton macro.

mod derive_static;
mod derive_trait;
mod facade_fn;
mod impl_fn;
mod lock;
mod singleton_fns;

use convert_case::Case;
use convert_case::Casing;

use darling::FromDeriveInput;

use proc_macro2::Ident;
use proc_macro2::TokenStream;

use quote::format_ident;
use quote::quote;

use syn::DeriveInput;
use syn::ItemFn;

use std::convert::TryFrom;

use crate::factory::Factory;

use self::derive_static::SingletonStaticFactory;
use self::derive_trait::SingletonTraitFactory;
use self::facade_fn::FacadeFnFactory;
use self::impl_fn::ImplFnFactory;
use self::lock::SingletonLock;
use self::singleton_fns::SingletonFnType;

/// Prefix for the generated singleton static.
const SINGLETON_STATIC_PREFIX: &str = "BLOCKZ_SINGLETON_STATIC_";

/// A factory that builds singletons.
pub(crate) struct SingletonFactory<'i> {
    input: &'i DeriveInput,
    opts: SingletonOpts,
}

#[derive(FromDeriveInput)]
#[darling(attributes(singleton))]
pub(crate) struct SingletonOpts {
    #[darling(default)]
    lock: SingletonLock,
}

/// A factory that builds singleton fns.
pub(crate) struct SingletonFnFactory<'f> {
    // the base function
    base: &'f ItemFn,
    // the function type that will be built by the factory
    fn_type: SingletonFnType<'f>,
}

impl<'i> SingletonFactory<'i> {
    /// Create a new singleton factory.
    pub fn new(input: &'i DeriveInput) -> Result<Self, darling::Error> {
        Ok(Self {
            input,
            opts: SingletonOpts::from_derive_input(input)?,
        })
    }

    /// Create the name of the static variable that holds the singleton.
    fn create_static_ident(src: &Ident) -> Ident {
        // convert the type name to upper snake case and add a prefix
        let type_name_upper = src.to_string().to_case(Case::UpperSnake);
        format_ident!("{}{}", SINGLETON_STATIC_PREFIX, type_name_upper)
    }
}

impl<'i> Factory for SingletonFactory<'i> {
    type Product = syn::Result<TokenStream>;

    /// Build the Singleton implementation.
    fn build(self) -> Self::Product {
        let static_ident = Self::create_static_ident(&self.input.ident);
        let singleton_static =
            SingletonStaticFactory::new(&static_ident, &self.input.ident, &self.opts.lock)
                .build()?;
        let singleton_trait =
            SingletonTraitFactory::new(&static_ident, &self.input.ident, &self.opts.lock).build();
        Ok(quote! {
            #singleton_static
            #singleton_trait
        })
    }
}

impl<'f> SingletonFnFactory<'f> {
    /// Create a new singleton fn factory.
    pub fn new(base: &'f ItemFn) -> syn::Result<Self> {
        Ok(Self {
            base,
            fn_type: SingletonFnType::try_from(base)?,
        })
    }
}

impl<'f> Factory for SingletonFnFactory<'f> {
    type Product = syn::Result<TokenStream>;

    /// Build the singleton fn facade and impl.
    fn build(self) -> Self::Product {
        let impl_fn = ImplFnFactory::new(self.base, &self.fn_type).build()?;
        let facade_fn = FacadeFnFactory::new(self.base, &self.fn_type, &impl_fn).build()?;
        Ok(quote! {
            #facade_fn
            #impl_fn
        })
    }
}
