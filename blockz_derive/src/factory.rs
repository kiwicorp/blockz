//! Factories that build code.

/// A factory thar produces a product.
pub(crate) trait Factory {
    /// The product produced by this factory.
    type Product;

    /// Build the product.
    fn build(self) -> Self::Product;
}

/// A factory that can be reused (by taking &mut self instead of self).
pub(crate) trait ReusableFactory {
    /// The product produced by this factory.
    type Product;

    /// Build the product.
    fn build(&mut self) -> Self::Product;
}

impl<Rf> Factory for Rf
where
    Rf: ReusableFactory
{
    type Product = Rf::Product;

    fn build(self) -> Self::Product {
        <Rf as ReusableFactory>::build(&mut self)
    }
}
