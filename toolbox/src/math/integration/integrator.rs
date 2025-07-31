use crate::math::traits::FloatNumber;
use crate::math::vector::BasicVectorOps;
use crate::math::vector::VectorN;

use super::integration_utils::DynamicsFunction;
use super::integration_utils::NumericalIntegrationStep;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* RUNGE-KUTTA 4 INTEGRATOR */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy)]
pub struct Integrator<T, D, const N: usize> {
    state: VectorN<T, N>,
    dt: T,
    ddt_fn: DynamicsFunction<T, D, N>,
    ddt_fn_args: D,
    time: T,
    tolerance: T,
}

impl<T, D, const N: usize> Integrator<T, D, N>
where
    T: FloatNumber<T>,
    D: Clone + Copy,
{
    pub fn build(state: [T; N], dt: T, dynamics: DynamicsFunction<T, D, N>, args: D) -> Self {
        Self {
            state: VectorN::build(state),
            dt,
            ddt_fn: dynamics,
            ddt_fn_args: args,
            time: T::zero(),
            tolerance: T::constant(<Integrator<T, D, N> as NumericalIntegrationStep<[T; N]>>::TOLERANCE),
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

    pub fn ddt_fn(&self) -> DynamicsFunction<T, D, N> {
        self.ddt_fn
    }

    pub fn ddt_fn_mut(&mut self) -> &mut DynamicsFunction<T, D, N> {
        &mut self.ddt_fn
    }

    pub fn ddt_fn_args(&self) -> D {
        self.ddt_fn_args
    }

    pub fn ddt_fn_args_mut(&mut self) -> &mut D {
        &mut self.ddt_fn_args
    }

    pub fn time(&self) -> T {
        self.time
    }

    pub fn tolerance(&self) -> T {
        self.tolerance
    }

    pub fn tolerance_mut(&mut self) -> &mut T {
        &mut self.tolerance
    }

    pub fn step(&mut self) -> [T; N] {
        self.time += self.dt;
        self.state = VectorN::build(self.integration_step());
        self.state.inner
    }

    pub fn step_dynamic(&mut self) -> [T; N] {
        let one_2 = T::constant(1. / 2.);
        let two = T::constant(2.);

        let mut oracle = *self;
        oracle.dt = self.dt * one_2;
        (0..2).for_each(|_| {
            oracle.step();
        });
        let step = VectorN::build(self.step());

        let error = (step * -T::one() + oracle.state).squared_length();
        if error > self.tolerance {
            self.dt *= one_2;

            return self.step_dynamic();
        }

        let out = self.step();
        self.dt *= two;

        out
    }
}

impl<T, D, const N: usize> NumericalIntegrationStep<[T; N]> for Integrator<T, D, N>
where
    T: FloatNumber<T>,
    D: Clone + Copy,
{
    fn integration_step(&self) -> [T; N] {
        let one_2 = T::constant(1. / 2.);
        let one_3 = T::constant(1. / 3.);
        let one_6 = T::constant(1. / 6.);
        let args = self.ddt_fn_args;

        let k1 = VectorN::build((self.ddt_fn)(&self.state.inner, args));
        let k2 = VectorN::build((self.ddt_fn)(&(self.state + k1 * (self.dt * one_2)).inner, args));
        let k3 = VectorN::build((self.ddt_fn)(&(self.state + k2 * (self.dt * one_2)).inner, args));
        let k4 = VectorN::build((self.ddt_fn)(&(self.state + k3 * self.dt).inner, args));

        let out = self.state + (k1 + k4) * (self.dt * one_6) + (k2 + k3) * (self.dt * one_3);
        out.inner
    }
}
