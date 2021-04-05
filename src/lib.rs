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

    // macro_rules! ui_test {
    //     (pass: [$($feat_pass: literal [$( $index_feat_pass:literal ),*]);*], fail: [$($feat_fail: literal [$( $index_feat_fail:literal ),*]);*]) => {
    //         let t = ::trybuild::TestCases::new();
    //         $(
    //             $(
    //                 t.pass(format!("tests/ui/{}-p-{}.rs", $feat_pass, $index_feat_pass));
    //             )*
    //         )*
    //     };
    // }

    /// Test the `singleton` feature.
    #[test]
    #[cfg(feature = "singleton")]
    fn test_singleton() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "singleton", [1, 2, 3]);
        ui_tests!(t, fail, "singleton", [1, 2, 3, 4]);
    }

    /// Test the direct configuration.
    #[test]
    #[cfg(all(
        feature = "configuration",
        not(any(
            feature = "env_configuration",
        ))))]
    fn test_direct_configuration() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "direct_configuration", [1, 2]);
        ui_tests!(t, fail, "direct_configuration", [1, 2]);
    }

    /// Test the `env_configuration` feature.
    #[test]
    #[cfg(feature = "env_configuration")]
    fn test_envy_configuration() {
        let t = trybuild::TestCases::new();

        ui_tests!(t, pass, "env_configuration", [1, 2, 3, 4, 5, 6, 7]);
    }
}
