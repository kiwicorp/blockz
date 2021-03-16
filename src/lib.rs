//! Blockz.

pub use blockz_derive as derive;

#[cfg(feature = "configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
pub mod configuration;

#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
pub mod singleton;

/// Prelude for blockz.
pub mod prelude {
    #[cfg(feature = "configuration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
    pub use crate::configuration::Configuration;

    #[cfg(feature = "configuration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
    pub use crate::configuration::EasyConfiguration;

    pub use crate::derive::*;

    #[cfg(feature = "singleton")]
    #[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
    pub use crate::singleton::Singleton;
}

/// Tests for the derive crate.
///
/// These actually just try to compile the examples.
#[cfg(test)]
mod test {
    /// Test the `singleton` feature.
    #[test]
    #[cfg(feature = "singleton")]
    fn test_feature_singleton() {
        let t = trybuild::TestCases::new();

        t.pass("tests/ui/singleton_good.rs");
    }

    /// Test the `envy_configuration` feature.
    #[test]
    #[cfg(feature = "envy_configuration")]
    fn envy_configuration_example() {
        let t = trybuild::TestCases::new();

        t.pass("tests/ui/envy_configuration_good.rs");
    }
}
