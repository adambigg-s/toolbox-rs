use std::ops::Mul;

use super::traits::{Adding, Multiplying, PrimitiveNumber};

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* MAT2 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix2<T> {
    pub inner: [[T; 2]; 2],
}

impl<T> Matrix2<T> {
    pub const DIM: usize = 2;

    pub fn build(inner: [[T; 2]; 2]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix2<T>
where
    T: PrimitiveNumber<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); 2]; 2])
    }

    pub fn identity() -> Self {
        let mut inner = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            inner[i][i] = T::one();
        });

        Self::build(inner)
    }
}

impl<T> Mul<Self> for Matrix2<T>
where
    T: Adding<T> + Multiplying<T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);
        let mut inner = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            (0..Self::DIM).for_each(|j| {
                (0..Self::DIM).for_each(|k| {
                    inner[i][j] += a[i][k] * b[k][j];
                });
            });
        });

        Self::build(inner)
    }
}
