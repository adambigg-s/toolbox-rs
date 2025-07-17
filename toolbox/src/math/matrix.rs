use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use super::traits::FloatNumber;
use super::traits::Numeric;
use super::traits::One;
use super::traits::Scalar;
use super::traits::Zero;
use super::vector::Vector2;
use super::vector::Vector3;
use super::vector::Vector4;
use super::vector::Vector5;
use super::vector::VectorN;

pub trait BasicMatrixOps<T>
where
    Self: Sized,
{
    fn determinant(&self) -> T;

    fn transpose(&self) -> Self;

    fn trace(&self) -> T;

    fn cofactor(&self, row: usize, col: usize) -> T;

    fn cofactor_matrix(&self) -> Self;
}

pub trait FloatMatrixOps<T>
where
    Self: Sized,
{
    fn inverse(&self) -> Option<Self>;
}

#[macro_export]
macro_rules! matrix {
    ($a0:expr $(,)?) => {
        $crate::mat1!($a0)
    };
    ($type:ty; $a0:expr $(,)?) => {
        $crate::mat1!($type; $a0)
    };

    (
        $a0:expr, $a1:expr,
        $a2:expr, $a3:expr $(,)?
    ) => {
        $crate::mat2!(
            $a0, $a1,
            $a2, $a3
        )
    };
    ($type:ty;
        $a0:expr, $a1:expr,
        $a2:expr, $a3:expr $(,)?
    ) => {
        $crate::mat2!($type;
            $a0, $a1,
            $a2, $a3
        )
    };

    (
        $a0:expr, $a1:expr, $a2:expr,
        $a3:expr, $a4:expr, $a5:expr,
        $a6:expr, $a7:expr, $a8:expr $(,)?
    ) => {
        $crate::mat3!(
            $a0, $a1, $a2,
            $a3, $a4, $a5,
            $a6, $a7, $a8
        )
    };
    ($type:ty;
        $a0:expr, $a1:expr, $a2:expr,
        $a3:expr, $a4:expr, $a5:expr,
        $a6:expr, $a7:expr, $a8:expr $(,)?
    ) => {
        $crate::mat3!($type;
            $a0, $a1, $a2,
            $a3, $a4, $a5,
            $a6, $a7, $a8
        )
    };

    (
        $a0:expr , $a1:expr , $a2:expr , $a3:expr ,
        $a4:expr , $a5:expr , $a6:expr , $a7:expr ,
        $a8:expr , $a9:expr , $a10:expr, $a11:expr,
        $a12:expr, $a13:expr, $a14:expr, $a15:expr $(,)?
    ) => {
        $crate::mat4!(
            $a0 , $a1 , $a2 , $a3 ,
            $a4 , $a5 , $a6 , $a7 ,
            $a8 , $a9 , $a10, $a11,
            $a12, $a13, $a14, $a15
        )
    };
    ($type:ty;
        $a0:expr , $a1:expr , $a2:expr , $a3:expr ,
        $a4:expr , $a5:expr , $a6:expr , $a7:expr ,
        $a8:expr , $a9:expr , $a10:expr, $a11:expr,
        $a12:expr, $a13:expr, $a14:expr, $a15:expr $(,)?
    ) => {
        $crate::mat4!($type;
            $a0 , $a1 , $a2 , $a3 ,
            $a4 , $a5 , $a6 , $a7 ,
            $a8 , $a9 , $a10, $a11,
            $a12, $a13, $a14, $a15
        )
    };

    (
        $a0:expr , $a1:expr , $a2:expr , $a3:expr , $a4:expr ,
        $a5:expr , $a6:expr , $a7:expr , $a8:expr , $a9:expr ,
        $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr,
        $a15:expr, $a16:expr, $a17:expr, $a18:expr, $a19:expr,
        $a20:expr, $a21:expr, $a22:expr, $a23:expr, $a24:expr $(,)?
    ) => {
        $crate::mat5!(
            $a0 , $a1 , $a2 , $a3 , $a4 ,
            $a5 , $a6 , $a7 , $a8 , $a9 ,
            $a10, $a11, $a12, $a13, $a14,
            $a15, $a16, $a17, $a18, $a19,
            $a20, $a21, $a22, $a23, $a24
        )
    };
    ($type:ty;
        $a0:expr , $a1:expr , $a2:expr , $a3:expr , $a4:expr ,
        $a5:expr , $a6:expr , $a7:expr , $a8:expr , $a9:expr ,
        $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr,
        $a15:expr, $a16:expr, $a17:expr, $a18:expr, $a19:expr,
        $a20:expr, $a21:expr, $a22:expr, $a23:expr, $a24:expr $(,)?
    ) => {
        $crate::mat5!($type;
            $a0 , $a1 , $a2 , $a3 , $a4 ,
            $a5 , $a6 , $a7 , $a8 , $a9 ,
            $a10, $a11, $a12, $a13, $a14,
            $a15, $a16, $a17, $a18, $a19,
            $a20, $a21, $a22, $a23, $a24
        )
    };
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* MATRIX 1 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix1<T> {
    pub ele: T,
}

impl<T> Matrix1<T> {
    pub const DIM: usize = 1;

    pub const fn build(ele: T) -> Self {
        Self { ele }
    }
}

#[macro_export]
#[rustfmt::skip]
macro_rules! mat1 {
    ($a0:expr) => {
        Matrix1::build($a0 as f32)
    };
    ($type:ty; $a0:expr) => {
        Matrix1::build($a0 as $type)
    };
}

impl<T> Matrix1<T>
where
    T: Scalar + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build(T::zero())
    }

    pub fn identity() -> Self {
        Self::build(T::one())
    }

    pub fn splat(value: T) -> Self {
        Self::build(value)
    }
}

impl<T> BasicMatrixOps<T> for Matrix1<T>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        self.ele
    }

    fn transpose(&self) -> Self {
        *self
    }

    fn trace(&self) -> T {
        self.ele
    }

    fn cofactor(&self, _: usize, _: usize) -> T {
        self.ele
    }

    fn cofactor_matrix(&self) -> Self {
        *self
    }
}

impl<T> FloatMatrixOps<T> for Matrix1<T>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        Some(*self)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* MATRIX 2 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix2<T> {
    pub inner: [[T; 2]; 2],
}

#[macro_export]
#[rustfmt::skip]
macro_rules! mat2 {
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        Matrix2::build([
            [$a0 as f32, $a1 as f32],
            [$a2 as f32, $a3 as f32],
        ])
    };
    ($type:ty; $a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        Matrix2::build([
            [$a0 as $type, $a1 as $type],
            [$a2 as $type, $a3 as $type],
        ])
    };
}

impl<T> Matrix2<T> {
    pub const DIM: usize = 2;

    pub const fn build(inner: [[T; 2]; 2]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix2<T>
where
    T: Scalar + Zero<T> + One<T>,
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

impl<T> BasicMatrixOps<T> for Matrix2<T>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        let m = self.inner;

        m[0][0] * m[1][1] - m[0][1] * m[1][0]
    }

    fn transpose(&self) -> Self {
        let mut m = self.inner;

        (m[0][1], m[1][0]) = (m[1][0], m[0][1]);

        Self::build(m)
    }

    fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1]
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        #![allow(clippy::needless_range_loop)]

        let m = self.inner;
        let mut sub = Matrix1::zeros();

        for y in 0..Self::DIM {
            if y == row {
                continue;
            }

            for x in 0..Self::DIM {
                if x == col {
                    continue;
                }

                sub.ele = m[y][x];
            }
        }

        sub.determinant()
    }

    fn cofactor_matrix(&self) -> Self {
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
}

impl<T> FloatMatrixOps<T> for Matrix2<T>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        let m = self.inner;

        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjugate = Self::build([[m[1][1], -m[0][1]], [-m[1][0], m[0][0]]]);

        Some(adjugate * inv_det)
    }
}

impl<T> Add<Self> for Matrix2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        Self::build([
            [a[0][0] + b[0][0], a[0][1] + b[0][1]],
            [a[1][0] + b[1][0], a[1][1] + b[1][1]],
        ])
    }
}

impl<T> Sub<Self> for Matrix2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        Self::build([
            [a[0][0] - b[0][0], a[0][1] - b[0][1]],
            [a[1][0] - b[1][0], a[1][1] - b[1][1]],
        ])
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

#[macro_export]
#[rustfmt::skip]
macro_rules! mat3 {
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr) => {
        Matrix3::build([
            [$a0 as f32, $a1 as f32, $a2 as f32],
            [$a3 as f32, $a4 as f32, $a5 as f32],
            [$a6 as f32, $a7 as f32, $a8 as f32],
        ])
    };
    ($type:ty; $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr) => {
        Matrix3::build([
            [$a0 as $type, $a1 as $type, $a2 as $type],
            [$a3 as $type, $a4 as $type, $a5 as $type],
            [$a6 as $type, $a7 as $type, $a8 as $type],
        ])
    };
}

impl<T> Matrix3<T> {
    pub const DIM: usize = 3;

    pub const fn build(inner: [[T; 3]; 3]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix3<T>
where
    T: Scalar + Zero<T> + One<T>,
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

impl<T> BasicMatrixOps<T> for Matrix3<T>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        let m = self.inner;

        let a = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]);
        let b = m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]);
        let c = m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);

        a - b + c
    }

    fn transpose(&self) -> Self {
        let mut m = self.inner;

        (m[0][1], m[0][2], m[1][2], m[1][0], m[2][0], m[2][1]) =
            (m[1][0], m[2][0], m[2][1], m[0][1], m[0][2], m[1][2]);

        Self::build(m)
    }

    fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1] + m[2][2]
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        #![allow(clippy::needless_range_loop)]

        let m = self.inner;
        let mut sub = Matrix2::zeros().inner;

        let mut i = 0;
        for y in 0..Self::DIM {
            if y == row {
                continue;
            }

            let mut j = 0;
            for x in 0..Self::DIM {
                if x == col {
                    continue;
                }

                sub[i][j] = m[y][x];

                j += 1;
            }

            i += 1;
        }

        Matrix2::build(sub).determinant()
    }

    fn cofactor_matrix(&self) -> Self {
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
}

impl<T> FloatMatrixOps<T> for Matrix3<T>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjugate = self.cofactor_matrix().transpose();

        Some(adjugate * inv_det)
    }
}

impl<T> Add<Self> for Matrix3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        Self::build([
            [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2]],
            [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2]],
            [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2]],
        ])
    }
}

impl<T> Sub<Self> for Matrix3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        Self::build([
            [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2]],
            [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2]],
            [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2]],
        ])
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
/* MATRIX 4 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix4<T> {
    pub inner: [[T; 4]; 4],
}

#[macro_export]
#[rustfmt::skip]
macro_rules! mat4 {
    (
        $a0:expr , $a1:expr , $a2:expr , $a3:expr ,
        $a4:expr , $a5:expr , $a6:expr , $a7:expr ,
        $a8:expr , $a9:expr , $a10:expr, $a11:expr,
        $a12:expr, $a13:expr, $a14:expr, $a15:expr
    ) => {
        Matrix4::build([
            [$a0 as f32 , $a1 as f32 , $a2 as f32 , $a3 as f32 ],
            [$a4 as f32 , $a5 as f32 , $a6 as f32 , $a7 as f32 ],
            [$a8 as f32 , $a9 as f32 , $a10 as f32, $a11 as f32],
            [$a12 as f32, $a13 as f32, $a14 as f32, $a15 as f32],
        ])
    };
    ($type:ty;
        $a0:expr , $a1:expr , $a2:expr , $a3:expr ,
        $a4:expr , $a5:expr , $a6:expr , $a7:expr ,
        $a8:expr , $a9:expr , $a10:expr, $a11:expr,
        $a12:expr, $a13:expr, $a14:expr, $a15:expr
    ) => {
        Matrix4::build([
            [$a0 as $type , $a1 as $type , $a2 as $type , $a3 as $type ],
            [$a4 as $type , $a5 as $type , $a6 as $type , $a7 as $type ],
            [$a8 as $type , $a9 as $type , $a10 as $type, $a11 as $type],
            [$a12 as $type, $a13 as $type, $a14 as $type, $a15 as $type],
        ])
    };
}

impl<T> Matrix4<T> {
    pub const DIM: usize = 4;

    pub const fn build(inner: [[T; 4]; 4]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix4<T>
where
    T: Scalar + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); 4]; 4])
    }

    pub fn identity() -> Self {
        let mut inner = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            inner[i][i] = T::one();
        });

        Self::build(inner)
    }

    pub fn splat(value: T) -> Self {
        Self::build([[value; 4]; 4])
    }
}

impl<T> BasicMatrixOps<T> for Matrix4<T>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        let m = self.inner;

        m[0][0] * self.cofactor(0, 0)
            + m[0][1] * self.cofactor(0, 1)
            + m[0][2] * self.cofactor(0, 2)
            + m[0][3] * self.cofactor(0, 3)
    }

    fn transpose(&self) -> Self {
        let mut m = self.inner;

        for i in 0..Self::DIM {
            for j in 0..Self::DIM {
                if i == j {
                    continue;
                }

                (m[i][j], m[j][i]) = (m[j][i], m[i][j]);
            }
        }

        Self::build(m)
    }

    fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1] + m[2][2] + m[3][3]
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        #![allow(clippy::needless_range_loop)]

        let m = self.inner;
        let mut sub = Matrix3::zeros().inner;

        let mut i = 0;
        for y in 0..Self::DIM {
            if y == row {
                continue;
            }

            let mut j = 0;
            for x in 0..Self::DIM {
                if x == col {
                    continue;
                }

                sub[i][j] = m[y][x];

                j += 1;
            }

            i += 1;
        }

        Matrix3::build(sub).determinant()
    }

    fn cofactor_matrix(&self) -> Self {
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
}

impl<T> FloatMatrixOps<T> for Matrix4<T>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjugate = self.cofactor_matrix().transpose();

        Some(adjugate * inv_det)
    }
}

impl<T> Add<Self> for Matrix4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        #[rustfmt::skip]
        let out = Self::build([
            [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2], a[0][3] + b[0][3]],
            [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2], a[1][3] + b[1][3]],
            [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2], a[2][3] + b[2][3]],
            [a[3][0] + b[3][0], a[3][1] + b[3][1], a[3][2] + b[3][2], a[3][3] + b[3][3]],
        ]);
        out
    }
}

impl<T> Sub<Self> for Matrix4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        #[rustfmt::skip]
        let out = Self::build([
            [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2], a[0][3] - b[0][3]],
            [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2], a[1][3] - b[1][3]],
            [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2], a[2][3] - b[2][3]],
            [a[3][0] - b[3][0], a[3][1] - b[3][1], a[3][2] - b[3][2], a[3][3] - b[3][3]],
        ]);
        out
    }
}

impl<T> Mul<T> for Matrix4<T>
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

impl<T> Mul<Vector4<T>> for Matrix4<T>
where
    T: Numeric<T>,
{
    type Output = Vector4<T>;

    fn mul(self, vec: Vector4<T>) -> Self::Output {
        let (m, v) = (self.inner, vec);

        #[rustfmt::skip]
        let out = Vector4::build(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * v.w,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * v.w,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * v.w,
            m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3] * v.w,
        );
        out
    }
}

impl<T> Mul<Self> for Matrix4<T>
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
/* MATRIX 5 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix5<T> {
    pub inner: [[T; 5]; 5],
}

#[macro_export]
#[rustfmt::skip]
macro_rules! mat5 {
    (
        $a0:expr , $a1:expr , $a2:expr , $a3:expr , $a4:expr ,
        $a5:expr , $a6:expr , $a7:expr , $a8:expr , $a9:expr ,
        $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr,
        $a15:expr, $a16:expr, $a17:expr, $a18:expr, $a19:expr,
        $a20:expr, $a21:expr, $a22:expr, $a23:expr, $a24:expr
    ) => {
        Matrix5::build([
            [$a0 as f32 , $a1 as f32 , $a2 as f32 , $a3 as f32 , $a4 as f32 ],
            [$a5 as f32 , $a6 as f32 , $a7 as f32 , $a8 as f32 , $a9 as f32 ],
            [$a10 as f32, $a11 as f32, $a12 as f32, $a13 as f32, $a14 as f32],
            [$a15 as f32, $a16 as f32, $a17 as f32, $a18 as f32, $a19 as f32],
            [$a20 as f32, $a21 as f32, $a22 as f32, $a23 as f32, $a24 as f32],
        ])
    };
    ($type:ty;
        $a0:expr , $a1:expr , $a2:expr , $a3:expr , $a4:expr ,
        $a5:expr , $a6:expr , $a7:expr , $a8:expr , $a9:expr ,
        $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr,
        $a15:expr, $a16:expr, $a17:expr, $a18:expr, $a19:expr,
        $a20:expr, $a21:expr, $a22:expr, $a23:expr, $a24:expr
    ) => {
        Matrix5::build([
            [$a0 as $type , $a1 as $type , $a2 as $type , $a3 as $type , $a4 as $type ],
            [$a5 as $type , $a6 as $type , $a7 as $type , $a8 as $type , $a9 as $type ],
            [$a10 as $type, $a11 as $type, $a12 as $type, $a13 as $type, $a14 as $type],
            [$a15 as $type, $a16 as $type, $a17 as $type, $a18 as $type, $a19 as $type],
            [$a20 as $type, $a21 as $type, $a22 as $type, $a23 as $type, $a24 as $type],
        ])
    };
}

impl<T> Matrix5<T> {
    pub const DIM: usize = 5;

    pub const fn build(inner: [[T; 5]; 5]) -> Self {
        Self { inner }
    }
}

impl<T> Matrix5<T>
where
    T: Scalar + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); 5]; 5])
    }

    pub fn identity() -> Self {
        let mut inner = Self::zeros().inner;

        (0..Self::DIM).for_each(|i| {
            inner[i][i] = T::one();
        });

        Self::build(inner)
    }

    pub fn splat(value: T) -> Self {
        Self::build([[value; 5]; 5])
    }
}

impl<T> BasicMatrixOps<T> for Matrix5<T>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        let m = self.inner;

        m[0][0] * self.cofactor(0, 0)
            + m[0][1] * self.cofactor(0, 1)
            + m[0][2] * self.cofactor(0, 2)
            + m[0][3] * self.cofactor(0, 3)
            + m[0][4] * self.cofactor(0, 4)
    }

    fn transpose(&self) -> Self {
        let mut m = self.inner;

        for i in 0..Self::DIM {
            for j in 0..Self::DIM {
                if i == j {
                    continue;
                }

                (m[i][j], m[j][i]) = (m[j][i], m[i][j]);
            }
        }

        Self::build(m)
    }

    fn trace(&self) -> T {
        let m = self.inner;

        m[0][0] + m[1][1] + m[2][2] + m[3][3] + m[4][4]
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        #![allow(clippy::needless_range_loop)]

        let m = self.inner;
        let mut sub = Matrix4::zeros().inner;

        let mut i = 0;
        for y in 0..Self::DIM {
            if y == row {
                continue;
            }

            let mut j = 0;
            for x in 0..Self::DIM {
                if x == col {
                    continue;
                }

                sub[i][j] = m[y][x];

                j += 1;
            }

            i += 1;
        }

        Matrix4::build(sub).determinant()
    }

    fn cofactor_matrix(&self) -> Self {
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
}

impl<T> FloatMatrixOps<T> for Matrix5<T>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjugate = self.cofactor_matrix().transpose();

        Some(adjugate * inv_det)
    }
}

impl<T> Add<Self> for Matrix5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        #[rustfmt::skip]
        let out = Self::build([
            [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2], a[0][3] + b[0][3], a[0][4] + b[0][4]],
            [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2], a[1][3] + b[1][3], a[1][4] + b[1][4]],
            [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2], a[2][3] + b[2][3], a[2][4] + b[2][4]],
            [a[3][0] + b[3][0], a[3][1] + b[3][1], a[3][2] + b[3][2], a[3][3] + b[3][3], a[3][4] + b[3][4]],
            [a[4][0] + b[4][0], a[4][1] + b[4][1], a[4][2] + b[4][2], a[4][3] + b[4][3], a[4][4] + b[4][4]],
        ]);
        out
    }
}

impl<T> Sub<Self> for Matrix5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);

        #[rustfmt::skip]
        let out = Self::build([
            [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2], a[0][3] - b[0][3], a[0][4] - b[0][4]],
            [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2], a[1][3] - b[1][3], a[1][4] - b[1][4]],
            [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2], a[2][3] - b[2][3], a[2][4] - b[2][4]],
            [a[3][0] - b[3][0], a[3][1] - b[3][1], a[3][2] - b[3][2], a[3][3] - b[3][3], a[3][4] - b[3][4]],
            [a[4][0] - b[4][0], a[4][1] - b[4][1], a[4][2] - b[4][2], a[4][3] - b[4][3], a[4][4] - b[4][4]],
        ]);
        out
    }
}

impl<T> Mul<T> for Matrix5<T>
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

impl<T> Mul<Vector5<T>> for Matrix5<T>
where
    T: Numeric<T>,
{
    type Output = Vector5<T>;

    fn mul(self, vec: Vector5<T>) -> Self::Output {
        let (m, v) = (self.inner, vec);

        #[rustfmt::skip]
        let out = Vector5::build(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * v.w + m[0][4] * v.t,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * v.w + m[1][4] * v.t,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * v.w + m[2][4] * v.t,
            m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3] * v.w + m[3][4] * v.t,
            m[4][0] * v.x + m[4][1] * v.y + m[4][2] * v.z + m[4][3] * v.w + m[4][4] * v.t,
        );
        out
    }
}

impl<T> Mul<Self> for Matrix5<T>
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
/* MATRIX N */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatrixN<T, const N: usize> {
    pub inner: [[T; N]; N],
}

impl<T, const N: usize> MatrixN<T, N> {
    pub const fn build(inner: [[T; N]; N]) -> Self {
        Self { inner }
    }
}

impl<T, const N: usize> MatrixN<T, N>
where
    T: Scalar + Zero<T> + One<T>,
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

    pub fn splat(value: T) -> Self {
        Self::build([[value; N]; N])
    }
}

impl<T, const N: usize> BasicMatrixOps<T> for MatrixN<T, N>
where
    T: Numeric<T>,
{
    fn determinant(&self) -> T {
        let m = self.inner;
        let mut out = T::zero();

        for i in 0..N {
            out += m[0][i] * self.cofactor(0, i);
        }

        out
    }

    fn transpose(&self) -> Self {
        let mut m = self.inner;
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    continue;
                }

                (m[i][j], m[j][i]) = (m[j][i], m[i][j]);
            }
        }

        Self::build(m)
    }

    fn trace(&self) -> T {
        let m = self.inner;
        let mut out = T::zero();

        (0..N).for_each(|i| {
            out += m[i][i];
        });

        out
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        #![allow(clippy::needless_range_loop)]

        let m = self.inner;
        let mut sub = Matrix3::zeros().inner;

        let mut i = 0;
        for y in 0..N {
            if y == row {
                continue;
            }

            let mut j = 0;
            for x in 0..N {
                if x == col {
                    continue;
                }

                sub[i][j] = m[y][x];

                j += 1;
            }

            i += 1;
        }

        Matrix3::build(sub).determinant()
    }

    fn cofactor_matrix(&self) -> Self {
        let mut m = Self::zeros().inner;

        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                let sign = match (i + j) % 2 == 0 {
                    | true => T::one(),
                    | false => -T::one(),
                };

                m[i][j] = self.cofactor(i, j) * sign;
            });
        });

        Self::build(m)
    }
}

impl<T, const N: usize> FloatMatrixOps<T> for MatrixN<T, N>
where
    T: FloatNumber<T>,
{
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let adjugate = self.cofactor_matrix().transpose();

        Some(adjugate * inv_det)
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

impl<T, const N: usize> Mul<VectorN<T, N>> for MatrixN<T, N>
where
    T: Numeric<T>,
{
    type Output = VectorN<T, N>;

    fn mul(self, vec: VectorN<T, N>) -> Self::Output {
        let (m, v) = (self.inner, vec.inner);
        let mut out = VectorN::zeros().inner;

        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                out[i] += m[i][j] * v[j];
            });
        });

        VectorN::build(out)
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
    pub const fn build(inner: [[T; N]; M]) -> Self {
        Self { inner }
    }
}

impl<T, const M: usize, const N: usize> MatrixMxN<T, M, N>
where
    T: Scalar + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([[T::zero(); N]; M])
    }

    pub fn splat(value: T) -> Self {
        Self::build([[value; N]; M])
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

impl<T, const M: usize, const N: usize> Mul<VectorN<T, N>> for MatrixMxN<T, M, N>
where
    T: Numeric<T>,
{
    type Output = VectorN<T, M>;

    fn mul(self, vec: VectorN<T, N>) -> Self::Output {
        let (m, v) = (self.inner, vec.inner);
        let mut out = VectorN::zeros().inner;

        (0..M).for_each(|i| {
            (0..N).for_each(|j| {
                out[i] += m[i][j] * v[j];
            });
        });

        VectorN::build(out)
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
}
