//! #[derive(Configuration)].

#[cfg(feature = "envy_configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "envy_configuration")))]
mod envy;

use darling::FromDeriveInput;

use proc_macro2::TokenStream;

use syn::DeriveInput;

#[cfg(feature = "envy_configuration")]
use self::envy::EnvyConfigurationFactory;
#[cfg(feature = "envy_configuration")]
use self::envy::EnvyConfigurationOpts;

#[cfg(feature = "envy_configuration")]
type DefaultFactory = EnvyConfigurationFactory;

/// A factory that builds implementations for the Configuration trait.
pub(crate) struct ConfigurationFactory<'f> {
    input: &'f DeriveInput,
    opts: ConfigurationOpts,
}

#[derive(FromDeriveInput)]
#[darling(attributes(configuration))]
struct ConfigurationOpts {
    #[cfg(feature = "envy_configuration")]
    #[darling(default)]
    envy: Option<EnvyConfigurationOpts>,
}

impl<'f> ConfigurationFactory<'f> {
    /// Create a new configuration factory.
    pub fn new(input: &'f DeriveInput) -> Result<Self, darling::Error> {
        Ok(Self {
            input,
            opts: ConfigurationOpts::from_derive_input(input)?,
        })
    }

    /// Build the configuration trait impl.
    pub fn build(self) -> syn::Result<TokenStream> {
        // let tokens: TokenStream;

        // if let Some(value) = self.opts.envy {
        //     tokens = EnvyConfigurationFactory::new(&self.input.ident, &value).build();
        // } else {
        //     // the default factory, as of right now only envy is supported
        //     // fixme 12/03/21: fix default behaviour
        //     tokens =
        //         EnvyConfigurationFactory::new(&self.input.ident, &EnvyConfigurationOpts::default())
        //             .build();
        // }

        // Ok(tokens)
        Ok(quote::quote! {})
    }
}
