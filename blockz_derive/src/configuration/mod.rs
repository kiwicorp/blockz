//! #[derive(Configuration)].

mod direct;
#[cfg(feature = "env_configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "env_configuration")))]
mod env;

use darling::FromDeriveInput;

use proc_macro2::TokenStream;

use syn::DeriveInput;

use crate::factory::Factory;
use crate::factory::ReusableFactory;
use crate::ProcMacroErrorExt;

use self::direct::DirectConfigurationFactory;
#[cfg(feature = "env_configuration")]
use self::env::EnvConfigurationFactory;
#[cfg(feature = "env_configuration")]
use self::env::EnvConfigurationOpts;

/// A product produced by a dyn factory.
type DynFactoryProduct = Result<TokenStream, Box<dyn ProcMacroErrorExt>>;

/// A dynamic factory that produces a token stream.
type DynFactory = Box<dyn ReusableFactory<Product = DynFactoryProduct>>;

/// A function that creates a specialized factory.
type FnNewDynFactory = Box<dyn FnOnce(DeriveInput, &mut ConfigurationOpts) -> DynFactory>;

/// A macro that automatically builds the functions for selecting the dynamic factory used for
/// producing the macro output.
macro_rules! feature_factory {
    ($feature_name: literal, $function_name: ident, $opts_field: ident, $factory_new_dyn: path) => {
        #[allow(unused_variables)]
        fn $function_name(opts: Option<&ConfigurationOpts>) -> Option<FnNewDynFactory> {
            #[cfg(not(feature = $feature_name))]
            {
                None
            }
            #[cfg(feature = $feature_name)]
            {
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
        }
    };
}

/// A factory that builds implementations for the Configuration trait.
pub(crate) struct ConfigurationFactory {
    input: DeriveInput,
    opts: ConfigurationOpts,
}

/// Options used by the configuration factory.
#[derive(FromDeriveInput)]
#[darling(attributes(configuration))]
struct ConfigurationOpts {
    #[cfg(feature = "env_configuration")]
    #[cfg_attr(feature = "env_configuration", darling(default))]
    env: Option<EnvConfigurationOpts>,
    #[darling(default)]
    direct: bool,
}

impl ConfigurationFactory {
    /// Create a new configuration factory.
    pub fn new(input: DeriveInput) -> Result<Self, darling::Error> {
        ConfigurationOpts::from_derive_input(&input).map(|opts| Self { input, opts })
    }

    fn direct_confiuration_factory(opts: &ConfigurationOpts) -> Option<FnNewDynFactory> {
        if opts.direct {
            Some(Box::new(DirectConfigurationFactory::new_dyn))
        } else {
            None
        }
    }

    feature_factory!(
        "env_configuration",
        env_configuration_factory,
        env,
        EnvConfigurationFactory::new_dyn
    );

    fn pick_new_dyn_factory_fn(opts: Option<&ConfigurationOpts>) -> Option<FnNewDynFactory> {
        Self::env_configuration_factory(opts).or_else(Option::default)
    }

    /// Returns a function that creates the new dynamic factory or None if the options did not
    /// specify a preference.
    fn get_new_dyn_factory_fn(&self) -> FnNewDynFactory {
        Self::pick_new_dyn_factory_fn(Some(&self.opts))
            .or_else(|| Self::direct_confiuration_factory(&self.opts))
            .or_else(|| Self::pick_new_dyn_factory_fn(None))
            .unwrap_or(Box::new(DirectConfigurationFactory::new_dyn))
    }
}

impl Factory for ConfigurationFactory {
    type Product = DynFactoryProduct;

    fn build(mut self) -> Self::Product {
        let factory_builder: FnNewDynFactory = self.get_new_dyn_factory_fn();
        let mut factory = factory_builder(self.input, &mut self.opts);
        factory.build()
    }
}
