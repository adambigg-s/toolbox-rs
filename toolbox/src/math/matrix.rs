use std::ops::Mul;

use super::traits::Numeric;
use super::traits::One;
use super::traits::PrimitiveNumber;
use super::traits::Zero;

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
    T: PrimitiveNumber + Zero<T> + One<T>,
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

impl<T> Matrix2<T>
where
    T: Numeric<T>,
{
    pub fn determinant(self) -> T {
        let m = self.inner;

        m[0][0] * m[1][1] - m[0][1] * m[1][0]
    }

    pub fn transpose(self) -> Self {
        let mut m = self.inner;

        (m[0][1], m[1][0]) = (m[1][0], m[0][1]);

        Self::build(m)
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let m = self.inner;

        let inner = [[m[1][1], -m[0][1]], [-m[1][0], m[0][0]]];

        Some(Self::build(inner) * inv_det)
    }
}

impl<T> Mul<T> for Matrix2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let mut inner = self.inner;

        (0..Self::DIM).for_each(|i| {
            (0..Self::DIM).for_each(|j| {
                inner[i][j] *= scalar;
            });
        });

        Self::build(inner)
    }
}

impl<T> Mul<Self> for Matrix2<T>
where
    T: Numeric<T>,
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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* TEST */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix2() {
        let matrix1 = Matrix2::<i32>::identity();
        let matrix2 = Matrix2::<i32>::identity();
        let output = matrix1 * matrix2;
        assert!(output == Matrix2::<i32>::identity());

        let matrix1 = Matrix2::<f32>::build([[1., 2.], [3., 4.]]);
        let matrix2 = Matrix2::<f32>::build([[4., 5.], [6., 7.]]);
        let output = matrix1 * matrix2;
        assert!(output == Matrix2::build([[16., 19.,], [36., 43.,]]));

        let matrix = Matrix2::build([[1., 2.], [3., 4.]]);
        let inverse = matrix.inverse().unwrap();
        assert!(inverse == Matrix2::build([[-2., 1.], [3. / 2., -1. / 2.]]));
    }
}
