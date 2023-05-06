pub trait Scalar<Rhs = Self> {
    type Output;

    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Self::Output;
    fn dot_product(&self, rhs: Rhs) -> f64;
    fn cross_product(&self, rhs: Rhs) -> Self::Output;
}
