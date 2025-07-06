use std::ops::Mul;

use super::traits::Numeric;
use super::traits::One;
use super::traits::PrimitiveNumber;
use super::traits::Zero;
use super::vector::Vector2;
use super::vector::Vector3;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* MATRIX 2 */
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

    pub fn splat(value: T) -> Self {
        Self::build([[value; 2]; 2])
    }
}

impl<T> Matrix2<T>
where
    T: Numeric<T>,
{
    pub fn determinant(&self) -> T {
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

        #[rustfmt::skip]
        let inner = [
            [ m[1][1], -m[0][1]],
            [-m[1][0],  m[0][0]],
        ];

        Some(Self::build(inner) * inv_det)
    }

    pub fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1]
    }

    #[allow(clippy::needless_range_loop)]
    pub fn cofactor(&self, i: usize, j: usize) -> T {
        let m = self.inner;
        let mut s = T::zero();

        for y in 0..Self::DIM {
            if y == i {
                continue;
            }
            for x in 0..Self::DIM {
                if x == j {
                    continue;
                }
                s = m[y][x];
            }
        }

        s
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

impl<T> Mul<Vector2<T>> for Matrix2<T>
where
    T: Numeric<T>,
{
    type Output = Vector2<T>;

    fn mul(self, vec: Vector2<T>) -> Self::Output {
        let (m, v) = (self.inner, vec);

        #[rustfmt::skip]
        let out = Vector2::build(
            m[0][0] * v.x + m[0][1] * v.y,
            m[1][0] * v.x + m[1][1] * v.y,
        );
        out
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
/* MATRIX 3 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix3<T> {
    pub inner: [[T; 3]; 3],
}

impl<T> Matrix3<T> {
    pub const DIM: usize = 3;

    pub fn build(inner: [[T; 3]; 3]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix3<T>
where
    T: PrimitiveNumber + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); 3]; 3])
    }

    pub fn identity() -> Self {
        let mut inner = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            inner[i][i] = T::one();
        });

        Self::build(inner)
    }

    pub fn splat(value: T) -> Self {
        Self::build([[value; 3]; 3])
    }
}

impl<T> Matrix3<T>
where
    T: Numeric<T>,
{
    pub fn determinant(&self) -> T {
        let m = self.inner;

        let a = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]);
        let b = m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]);
        let c = m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);

        a - b + c
    }

    pub fn transpose(self) -> Self {
        let mut m = self.inner;

        (m[0][1], m[0][2], m[1][2], m[1][0], m[2][0], m[2][1]) =
            (m[1][0], m[2][0], m[2][1], m[0][1], m[0][2], m[1][2]);

        Self::build(m)
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjoint = self.cofactor_matrix().transpose();

        Some(adjoint * inv_det)
    }

    pub fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1] + m[2][2]
    }

    pub fn cofactor_matrix(&self) -> Self {
        let mut m = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            (0..Self::DIM).for_each(|j| {
                let sign = match (i + j) % 2 == 0 {
                    | true => T::one(),
                    | false => -T::one(),
                };
                m[i][j] = self.cofactor(i, j) * sign;
            });
        });

        Self::build(m)
    }

    #[allow(clippy::needless_range_loop)]
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let m = self.inner;
        let mut s = Matrix2::zeros().inner;

        let mut i = 0;
        for y in 0..Self::DIM {
            let mut j = 0;
            if y == row {
                continue;
            }
            for x in 0..Self::DIM {
                if x == col {
                    continue;
                }

                s[i][j] = m[y][x];

                j += 1;
            }

            i += 1;
        }

        Matrix2::build(s).determinant()
    }
}

impl<T> Mul<T> for Matrix3<T>
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

impl<T> Mul<Vector3<T>> for Matrix3<T>
where
    T: Numeric<T>,
{
    type Output = Vector3<T>;

    fn mul(self, vec: Vector3<T>) -> Self::Output {
        let (m, v) = (self.inner, vec);

        #[rustfmt::skip]
        let out = Vector3::build(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        );
        out
    }
}

impl<T> Mul<Self> for Matrix3<T>
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
/* MATRIX NxN */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatrixN<T, const N: usize> {
    pub inner: [[T; N]; N],
}

impl<T, const N: usize> MatrixN<T, N> {
    pub fn build(inner: [[T; N]; N]) -> Self {
        Self { inner }
    }
}

impl<T, const N: usize> MatrixN<T, N>
where
    T: PrimitiveNumber + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); N]; N])
    }

    pub fn identity() -> Self {
        let mut inner = Self::zeros().inner;

        (0..N).for_each(|i| {
            inner[i][i] = T::one();
        });

        Self::build(inner)
    }
}

impl<T, const N: usize> Mul<T> for MatrixN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let mut inner = self.inner;

        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                inner[i][j] *= scalar;
            });
        });

        Self::build(inner)
    }
}

impl<T, const N: usize> Mul<Self> for MatrixN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);
        let mut inner = Self::zeros().inner;

        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                (0..N).for_each(|k| {
                    inner[i][j] += a[i][k] * b[k][j];
                });
            });
        });

        Self::build(inner)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* MATRIX MxN */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatrixMxN<T, const M: usize, const N: usize> {
    pub inner: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> MatrixMxN<T, M, N> {
    pub fn build(inner: [[T; N]; M]) -> Self {
        Self { inner }
    }
}

impl<T, const M: usize, const N: usize> MatrixMxN<T, M, N>
where
    T: PrimitiveNumber + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); N]; M])
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for MatrixMxN<T, M, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let mut inner = self.inner;

        (0..M).for_each(|i| {
            (0..N).for_each(|j| {
                inner[i][j] *= scalar;
            });
        });

        Self::build(inner)
    }
}

impl<T, const M: usize, const N: usize, const H: usize> Mul<MatrixMxN<T, M, N>> for MatrixMxN<T, H, M>
where
    T: Numeric<T>,
{
    type Output = MatrixMxN<T, H, N>;

    fn mul(self, other: MatrixMxN<T, M, N>) -> Self::Output {
        let (a, b) = (self.inner, other.inner);
        let mut inner = MatrixMxN::zeros().inner;

        (0..H).for_each(|i| {
            (0..N).for_each(|j| {
                (0..M).for_each(|k| {
                    inner[i][j] += a[i][k] * b[k][j];
                });
            });
        });

        MatrixMxN::build(inner)
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

    #[test]
    fn matrix3() {
        let matrix = Matrix3::build([[1., 2., 3.], [3., 4., 4.], [4., 5., 6.]]);
        let inverse = matrix.inverse();
        println!("inverse: {inverse:?}");
        panic!();
    }
}
