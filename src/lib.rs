//! Blockz is an opinionated library that aims to make it a pleasure to develop
//! networked applications in Rust.

#[cfg(feature = "configuration")]
#[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
pub mod configuration;

#[cfg(feature = "singleton")]
#[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
pub mod singleton;

pub use blockz_derive::*;

/// Blockz prelude - useful re-exports.
pub mod prelude {
    #[cfg(feature = "configuration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
    pub use crate::configuration::Configuration;

    #[cfg(feature = "configuration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "configuration")))]
    pub use crate::configuration::EasyConfiguration;

    pub use blockz_derive::*;

    #[cfg(feature = "singleton")]
    #[cfg_attr(docsrs, doc(cfg(feature = "singleton")))]
    pub use crate::singleton::Singleton;
}

/// Tests for the derive crate.
#[cfg(test)]
mod test {
    macro_rules! ui_tests {
        ($t: ident, pass, $feat: literal, [$( $index:literal ),*]) => {
            $(
                $t.pass(format!("tests/ui/{}-p-{}.rs", $feat, $index));
            )*
        };
        ($t: ident, fail, $feat: literal, [$( $index:literal ),*]) => {
            $(
                $t.compile_fail(format!("tests/ui/{}-f-{}.rs", $feat, $index));
            )*
        };
    }

    /// Test the `singleton` feature.
    #[test]
    #[cfg(feature = "singleton")]
    fn test_singleton() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "singleton", [0, 1, 2]);
        ui_tests!(t, fail, "singleton", [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    /// Test the direct configuration.
    #[test]
    #[cfg(all(feature = "configuration", not(any(feature = "env_configuration",))))]
    fn test_direct_configuration() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "direct_configuration", [0, 1, 2]);
        ui_tests!(t, fail, "direct_configuration", [0, 1]);
    }

    /// Test the `env_configuration` feature.
    #[test]
    #[cfg(feature = "env_configuration")]
    fn test_envy_configuration() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "env_configuration", [0, 1, 2, 3, 4, 5]);
        ui_tests!(t, fail, "env_configuration", [0, 1, 2, 3]);
    }
}
