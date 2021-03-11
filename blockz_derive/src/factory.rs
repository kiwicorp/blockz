//! Factory trait

/// Trait that defines the behaviour of code-producing factories.
pub(crate) trait Factory {
    /// The product produced by this factory.
    type Product;

    /// Build the product.
    fn build(self) -> syn::Result<Self::Product>;
}
