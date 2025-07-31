pub type DynamicsFunction<T, D, const N: usize> = fn(&[T; N], D) -> [T; N];

pub trait NumericalIntegrationStep<T>
where
    Self: Sized,
{
    const TOLERANCE: f64 = 1e-9;

    fn integration_step(&self) -> T;
}
