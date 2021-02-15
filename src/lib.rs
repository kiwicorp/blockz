//! Blockz.

pub use blockz_derive as derive;

pub mod component;
#[cfg(feature = "configuration")]
pub mod configuration;
#[cfg(feature = "singleton")]
pub mod singleton;

/// Prelude for blockz.
pub mod prelude {
    pub use crate::component::ComponentExt;
    #[cfg(feature = "configuration")]
    pub use crate::configuration::Configuration;
    pub use crate::derive::*;
    #[cfg(feature = "singleton")]
    pub use crate::singleton::Singleton;
}

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
    }
}
