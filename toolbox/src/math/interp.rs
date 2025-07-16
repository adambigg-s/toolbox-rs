use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use super::traits::Numeric;

pub struct Interpolator<T, D> {
    pub curr: T,
    pub step: T,
    steps: D,
}

impl<T, D> Interpolator<T, D>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Copy,
    D: Numeric<D>,
{
    pub fn build(start: T, end: T, steps: D) -> Self
    where
        T: Div<D, Output = T>,
    {
        Self { curr: start, step: (end - start) / steps, steps }
    }

    #[inline(always)]
    pub fn progress(&mut self) -> T {
        let out = self.curr;
        self.curr = self.curr + self.step;
        out
    }

    #[inline(always)]
    pub fn regress(&mut self) -> T {
        let out = self.curr;
        self.curr = self.curr - self.step;
        out
    }
}

impl<'d, T, D> IntoIterator for &'d mut Interpolator<T, D>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Copy,
    D: Numeric<D>,
{
    type Item = T;

    type IntoIter = Iter<'d, T, D>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter<'d, T, D> {
    interp: &'d mut Interpolator<T, D>,
    steps: D,
}

impl<'d, T, D> Iter<'d, T, D>
where
    D: Numeric<D>,
{
    pub fn new(interp: &'d mut Interpolator<T, D>) -> Self {
        let steps = interp.steps;
        Self { interp, steps }
    }
}

impl<'d, T, D> Iterator for Iter<'d, T, D>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Copy,
    D: Numeric<D>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps <= D::zero() {
            return None;
        }

        self.steps -= D::one();
        Some(self.interp.progress())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation() {
        let mut interp = Interpolator::build(0, 1000, 1000);

        let mut counter = 0;
        interp.into_iter().for_each(|interp_value| {
            assert!(interp_value == counter);
            counter += 1;
        });
    }
}
