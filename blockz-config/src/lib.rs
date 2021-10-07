//! Convenient application configuration.

/// An application configuration.
pub trait AppConfig<'a>: serde::Deserialize<'a> {
    const NAME: &'static str;

    /// Load the configuration.
    fn load() -> Self {
        let provider = figment::providers::Env::prefixed(Self::NAME);
        match figment::Figment::new()
            .join(provider)
            .extract() {
            Ok(value) => value,
            Err(e) => panic!("{}: app configuration: failed to load: {}", Self::NAME, e),
        }
    }
}
