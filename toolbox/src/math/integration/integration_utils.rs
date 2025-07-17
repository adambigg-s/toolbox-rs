pub type DynamicsFunction<T, const N: usize> = fn(&[T; N]) -> [T; N];

pub trait NumericalIntegrationStep<T>
where
    Self: Sized,
{
    const TOLERANCE: f64 = 1e-9;

    fn integration_step(&self) -> T;
}
