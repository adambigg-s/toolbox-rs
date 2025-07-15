use crate::math::traits::FloatNumber;
use crate::math::vector::VectorN;

use super::integration_utils::DynamicsFunction;

pub struct Integrator<T, const N: usize> {
    state: VectorN<T, N>,
    dt: T,
    ddt_fn: DynamicsFunction<T, N>,
    time: T,
}

impl<T, const N: usize> Integrator<T, N>
where
    T: FloatNumber<T>,
{
    pub fn build(state: [T; N], dt: T, dynamics: DynamicsFunction<T, N>) -> Self {
        Self {
            state: VectorN::build(state),
            dt,
            ddt_fn: dynamics,
            time: T::zero(),
        }
    }

    pub fn state(&self) -> [T; N] {
        self.state.array()
    }

    pub fn dt(&self) -> T {
        self.dt
    }

    pub fn dt_mut(&mut self) -> &mut T {
        &mut self.dt
    }

    pub fn ddt_fn(&self) -> DynamicsFunction<T, N> {
        self.ddt_fn
    }

    pub fn ddt_fn_mut(&mut self) -> &mut DynamicsFunction<T, N> {
        &mut self.ddt_fn
    }

    pub fn step(&mut self) -> [T; N] {
        todo!()
    }

    pub fn step_dynamic(&mut self) -> [T; N] {
        todo!()
    }
}
