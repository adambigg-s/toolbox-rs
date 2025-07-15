use crate::math::traits::FloatNumber;

pub const TOLERANCE: f64 = 1e-9;

pub type DynamicsFunction<T, const N: usize> = fn(&[T; N]) -> [T; N];

pub trait NumericalIntegrationStep<T>
where
    Self: Sized,
    T: FloatNumber<T>,
{
    fn step(&self) -> T;
}
