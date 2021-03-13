//! Blockz.

pub use blockz_derive as derive;

#[cfg(feature = "configuration")]
pub mod configuration;
#[cfg(feature = "singleton")]
pub mod singleton;

/// Prelude for blockz.
pub mod prelude {
    #[cfg(feature = "configuration")]
    pub use crate::configuration::Configuration;
    #[cfg(feature = "configuration")]
    pub use crate::configuration::EasyConfiguration;
    pub use crate::derive::*;
    #[cfg(feature = "singleton")]
    pub use crate::singleton::Singleton;
}

/// Do not compile if `envy_configuration` is enabled, but `configuration` is not.
#[cfg(all(feature = "envy_configuration", not(feature = "configuration")))]
compile_error!("The `envy_configuration` feature requires the `configuration` feature.");

/// Tests for the derive crate.
///
/// These actually just try to compile the examples.
#[cfg(test)]
mod test {
    /// Test the singleton example.
    #[test]
    #[cfg(feature = "singleton")]
    fn singleton_example() {
        let t = trybuild::TestCases::new();

        t.pass("examples/singleton.rs");
        t.pass("examples/configuration.rs");
    }
}
