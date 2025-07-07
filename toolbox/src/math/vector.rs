use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::traits::{FloatNumber, Numeric};

pub trait VectorOps<T>
where
    Self: Sized,
{
    fn inner_product(self, other: Self) -> T;

    fn length(self) -> T;

    fn squared_length(self) -> T;

    fn normalize(self) -> Self;
}

#[macro_export]
macro_rules! vector {
    ($a:expr, $b:expr) => {
        vec2!($a, $b)
    };
    ($type:ty; $a:expr, $b: expr) => {
        vec2!($type; $a, $b)
    };

    ($a:expr, $b:expr, $c:expr) => {
        vec3!($a, $b, $c)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr) => {
        vec3!($type; $a, $b, $c)
    };

    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        vec4!($a, $b, $c, $d)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr) => {
        vec4!($type; $a, $b, $c, $d)
    };
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 2 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        Vector2::build($a as f32, $b as f32)
    };
    ($type:ty; $a:expr, $b: expr) => {
        Vector2::build($a as $type, $b as $type)
    };
}

impl<T> Vector2<T> {
    pub fn build(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> VectorOps<T> for Vector2<T>
where
    T: FloatNumber<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn length(self) -> T {
        self.squared_length().sqrt()
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T> Add<Self> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Sub<Self> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> Mul<Self> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> Neg for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl<T> Rem<Self> for Vector2<T>
where
    T: FloatNumber<T>,
{
    type Output = T;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 3 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[macro_export]
macro_rules! vec3 {
    ($a:expr, $b:expr, $c:expr) => {
        Vector3::build($a as f32, $b as f32, $c as f32)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr) => {
        Vector3::build($a as $type, $b as $type, $c as $type)
    };
}

impl<T> Vector3<T> {
    pub fn build(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 4 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[macro_export]
macro_rules! vec4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Vector4::build($a as f32, $b as f32, $c as f32, $d as f32)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr) => {
        Vector4::build($a as $type, $b as $type, $c as $type, $d as $type)
    };
}

impl<T> Vector4<T> {
    pub fn build(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR N*/
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorN<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> VectorN<T, N> {
    pub fn build(inner: [T; N]) -> Self {
        Self { inner }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* TEST */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macros() {
        let vector = vector!(i32; 4, 10, 4.3, -5);
        assert!(vector == Vector4::build(4, 10, 4, -5));
    }
}
