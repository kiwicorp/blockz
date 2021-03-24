//! #[derive(Configuration)].

#[cfg(feature = "envy_configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "envy_configuration")))]
mod envy;

use darling::FromDeriveInput;

use proc_macro2::TokenStream;

use syn::DeriveInput;
use syn::Ident;

use crate::factory::ReusableFactory;

#[cfg(feature = "envy_configuration")]
use self::envy::EnvyConfigurationFactory;
#[cfg(feature = "envy_configuration")]
use self::envy::EnvyConfigurationOpts;

macro_rules! feature_factory {
    ($feature_name: literal, $function_name: ident, $opts_field: ident, $factory_new_dyn: path) => {
        #[cfg(not(feature = $feature_name))]
        fn $function_name(_: Option<&ConfigurationOpts>) -> Option<FnNewDynFactory> {
            None
        }

        #[cfg(feature = $feature_name)]
        fn $function_name(opts: Option<&ConfigurationOpts>) -> Option<FnNewDynFactory> {
            let opts = if let Some(value) = opts {
                value
            } else {
                return Some(Box::new($factory_new_dyn));
            };
            if opts.$opts_field.is_none() {
                None
            } else {
                Some(Box::new($factory_new_dyn))
            }
        }
    };
}

/// A dynamic factory that produces a token stream.
type DynFactory = Box<dyn ReusableFactory<Product = TokenStream>>;

/// A function that creates a specialized factory.
type FnNewDynFactory = Box<dyn FnOnce(&mut ConfigurationOpts) -> DynFactory>;

/// A factory that builds implementations for the Configuration trait.
pub(crate) struct ConfigurationFactory {
    opts: ConfigurationOpts,
}

/// Options used by the configuration factory.
#[derive(FromDeriveInput)]
#[darling(from_ident, attributes(configuration))]
struct ConfigurationOpts {
    ident: Ident,
    #[cfg(feature = "envy_configuration")]
    #[darling(default)]
    envy: Option<EnvyConfigurationOpts>,
}

impl From<Ident> for ConfigurationOpts {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            envy: Option::default(),
        }
    }
}

impl ConfigurationFactory {
    /// Create a new configuration factory.
    pub fn new(input: &DeriveInput) -> Result<Self, darling::Error> {
        Ok(Self {
            opts: ConfigurationOpts::from_derive_input(input)?,
        })
    }

    feature_factory!(
        "envy_configuration",
        envy_configuration_factory,
        envy,
        EnvyConfigurationFactory::new_dyn
    );

    fn pick_new_dyn_factory_fn(opts: Option<&ConfigurationOpts>) -> Option<FnNewDynFactory> {
        Self::envy_configuration_factory(opts)
            .or(None)
    }

    /// Returns a function that creates the new dynamic factory or None if the options did not
    /// specify a preference.
    fn get_new_dyn_factory_fn(&self) -> FnNewDynFactory {
        Self::pick_new_dyn_factory_fn(Some(&self.opts))
            .or(Self::pick_new_dyn_factory_fn(None))
            .unwrap() // should be unwrap_or(direct configuration)
    }
}

impl ReusableFactory for ConfigurationFactory {
    type Product = TokenStream;

    fn build(&mut self) -> Self::Product {
        let factory_builder: FnNewDynFactory = self.get_new_dyn_factory_fn();
        let mut factory = factory_builder(&mut self.opts);
        factory.build()
    }
}
