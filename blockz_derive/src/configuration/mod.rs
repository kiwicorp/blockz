//! #[derive(Configuration)].

mod envy;

use darling::FromDeriveInput;

use proc_macro2::TokenStream;

use syn::DeriveInput;

use self::envy::EnvyConfigurationFactory;
use self::envy::EnvyConfigurationOpts;

/// A factory that builds implementations for the Configuration trait.
pub(crate) struct ConfigurationFactory<'f> {
    input: &'f DeriveInput,
    opts: ConfigurationOpts,
}

#[derive(FromDeriveInput)]
#[darling(attributes(configuration))]
struct ConfigurationOpts {
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
        let tokens: TokenStream;

        if let Some(value) = self.opts.envy {
            tokens = EnvyConfigurationFactory::new(&self.input.ident, &value).build();
        } else {
            // the default factory, as of right now only envy is supported
            // fixme 12/03/21: fix default behaviour
            tokens =
                EnvyConfigurationFactory::new(&self.input.ident, &EnvyConfigurationOpts::default())
                    .build();
        }

        Ok(tokens)
    }
}
