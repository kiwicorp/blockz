//! Factories that build code.

/// A factory thar produces a product.
pub(crate) trait Factory {
    /// The product produced by this factory.
    type Product;

    /// Build the product.
    fn build(self) -> Self::Product;
}
