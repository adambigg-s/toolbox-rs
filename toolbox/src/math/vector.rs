use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

use super::traits::FloatNumber;
use super::traits::Numeric;
use super::traits::One;
use super::traits::Scalar;
use super::traits::Zero;

pub trait BasicVectorOps<T>
where
    Self: Sized,
{
    fn inner_product(self, other: Self) -> T;

    fn squared_length(self) -> T;
}

pub trait FloatVectorOps<T>
where
    Self: Sized,
{
    fn length(self) -> T;

    fn normalize(self) -> Self;
}

#[macro_export]
macro_rules! vector {
    ($a:expr $(,)?) => {
        $crate::vec1!($a)
    };
    ($type:ty; $a:expr $(,)?) => {
        $crate::vec1!($type; $a)
    };

    ($a:expr, $b:expr $(,)?) => {
        $crate::vec2!($a, $b)
    };
    ($type:ty; $a:expr, $b: expr $(,)?) => {
        $crate::vec2!($type; $a, $b)
    };

    ($a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::vec3!($a, $b, $c)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::vec3!($type; $a, $b, $c)
    };

    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        $crate::vec4!($a, $b, $c, $d)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        $crate::vec4!($type; $a, $b, $c, $d)
    };

    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        $crate::vec5!($a, $b, $c, $d, $e)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        $crate::vec5!($type; $a, $b, $c, $d, $e)
    };
}

#[macro_export]
macro_rules! swizzle {
    ($vec:expr; $($wild:tt),+ $(,)?) => {
        $crate::vector!($(swizzle!(@comp; $vec, $wild)),+)
    };

    (@comp; $vec:expr, $field:ident) => {
        $vec.$field
    };
    (@comp; $vec:expr, $value:expr) => {
        $value
    };
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 1 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector1<T> {
    pub ele: T,
}

#[macro_export]
macro_rules! vec1 {
    ($a:expr) => {
        Vector1::build($a as f32)
    };
    ($type:ty; $a:expr, $b: expr) => {
        Vector2::build($a as $type)
    };
}

impl<T> Vector1<T> {
    pub fn build(ele: T) -> Self {
        Self { ele }
    }
}

impl<T> BasicVectorOps<T> for Vector1<T>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.ele * other.ele
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
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

impl<T> Vector2<T>
where
    T: Numeric<T>,
{
    pub fn determinant(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T> BasicVectorOps<T> for Vector2<T>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
}

impl<T> FloatVectorOps<T> for Vector2<T>
where
    T: FloatNumber<T>,
{
    fn length(self) -> T {
        self.squared_length().sqrt()
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

    fn add(self, other: Self) -> Self::Output {
        Self::build(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub<Self> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::build(self.x - other.x, self.y - other.y)
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::build(self.x * scalar, self.y * scalar)
    }
}

impl<T> Mul<Self> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_product(other)
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::build(self.x / scalar, self.y / scalar)
    }
}

impl<T> Neg for Vector2<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::build(-self.x, -self.y)
    }
}

impl<T> Rem<Self> for Vector2<T>
where
    T: FloatNumber<T>,
{
    type Output = T;

    fn rem(self, other: Self) -> Self::Output {
        self.determinant(other)
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

impl<T> Vector3<T>
where
    T: Numeric<T>,
{
    pub fn cross_product(self, other: Self) -> Self {
        Self::build(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> BasicVectorOps<T> for Vector3<T>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
}

impl<T> FloatVectorOps<T> for Vector3<T>
where
    T: FloatNumber<T>,
{
    fn length(self) -> T {
        self.squared_length().sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T> Add<Self> for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::build(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub<Self> for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::build(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::build(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl<T> Mul<Self> for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_product(other)
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::build(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl<T> Neg for Vector3<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::build(-self.x, -self.y, -self.z)
    }
}

impl<T> Rem<Self> for Vector3<T>
where
    T: FloatNumber<T>,
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        self.cross_product(other)
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

impl<T> Vector4<T>
where
    T: Numeric<T>,
{
    pub fn array(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl<T> BasicVectorOps<T> for Vector4<T>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
}

impl<T> FloatVectorOps<T> for Vector4<T>
where
    T: FloatNumber<T>,
{
    fn length(self) -> T {
        self.squared_length().sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T> Add<Self> for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::build(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl<T> Sub<Self> for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::build(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl<T> Mul<T> for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::build(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl<T> Mul<Self> for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_product(other)
    }
}

impl<T> Div<T> for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::build(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}

impl<T> Neg for Vector4<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::build(-self.x, -self.y, -self.z, -self.w)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 5 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector5<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub t: T,
}

#[macro_export]
macro_rules! vec5 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        Vector4::build($a as f32, $b as f32, $c as f32, $d as f32, $e as f32)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        Vector4::build($a as $type, $b as $type, $c as $type, $d as $type, $e as $type)
    };
}

impl<T> Vector5<T> {
    pub fn build(x: T, y: T, z: T, w: T, t: T) -> Self {
        Self { x, y, z, w, t }
    }
}

impl<T> Vector5<T>
where
    T: Numeric<T>,
{
    pub fn array(&self) -> [T; 5] {
        [self.x, self.y, self.z, self.w, self.t]
    }
}

impl<T> BasicVectorOps<T> for Vector5<T>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
}

impl<T> FloatVectorOps<T> for Vector5<T>
where
    T: FloatNumber<T>,
{
    fn length(self) -> T {
        self.squared_length().sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T> Add<Self> for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::build(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w, self.t + other.t)
    }
}

impl<T> Sub<Self> for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::build(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w, self.t - other.t)
    }
}

impl<T> Mul<T> for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::build(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar, self.t * scalar)
    }
}

impl<T> Mul<Self> for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_product(other)
    }
}

impl<T> Div<T> for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::build(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar, self.t / scalar)
    }
}

impl<T> Neg for Vector5<T>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::build(-self.x, -self.y, -self.z, -self.w, -self.t)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR N */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorN<T, const N: usize> {
    pub inner: [T; N],
}

impl<T, const N: usize> VectorN<T, N> {
    pub fn build(inner: [T; N]) -> Self {
        Self { inner }
    }
}

impl<T, const N: usize> VectorN<T, N>
where
    T: Numeric<T>,
{
    pub fn array(&self) -> [T; N] {
        self.inner
    }
}

impl<T, const N: usize> VectorN<T, N>
where
    T: Scalar + Zero<T> + One<T>,
{
    pub fn zeros() -> Self {
        Self::build([T::zero(); N])
    }
}

impl<T, const N: usize> BasicVectorOps<T> for VectorN<T, N>
where
    T: Numeric<T>,
{
    fn inner_product(self, other: Self) -> T {
        let (a, b) = (self.inner, other.inner);
        let mut out = T::zero();

        (0..N).for_each(|i| {
            out += a[i] * b[i];
        });

        out
    }

    fn squared_length(self) -> T {
        self.inner_product(self)
    }
}

impl<T, const N: usize> FloatVectorOps<T> for VectorN<T, N>
where
    T: FloatNumber<T>,
{
    fn length(self) -> T {
        self.squared_length().sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T, const N: usize> Add<Self> for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);
        let mut inner = Self::zeros().inner;

        (0..N).for_each(|i| {
            inner[i] = a[i] + b[i];
        });

        Self::build(inner)
    }
}

impl<T, const N: usize> Sub<Self> for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (a, b) = (self.inner, other.inner);
        let mut inner = Self::zeros().inner;

        (0..N).for_each(|i| {
            inner[i] = a[i] - b[i];
        });

        Self::build(inner)
    }
}

impl<T, const N: usize> Mul<T> for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let mut v = self.inner;

        (0..N).for_each(|i| {
            v[i] *= scalar;
        });

        Self::build(v)
    }
}

impl<T, const N: usize> Mul<Self> for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_product(other)
    }
}

impl<T, const N: usize> Div<T> for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let mut v = self.inner;

        (0..N).for_each(|i| {
            v[i] /= scalar;
        });

        Self::build(v)
    }
}

impl<T, const N: usize> Neg for VectorN<T, N>
where
    T: Numeric<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut v = self.inner;

        (0..N).for_each(|i| {
            v[i] *= -T::one();
        });

        Self::build(v)
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

    #[test]
    fn overloading_cross() {
        let v1 = vector!(10, 0, 0);
        let v2 = vector!(0, 0, 10);
        let product = v1 % v2;
        assert!(product == Vector3::build(0., -100., 0.));
    }

    #[test]
    fn swizzle_testing() {
        let vector = Vector3::build(99, 1, 1);
        let vec = swizzle!(vector; x);
        assert!(vec == Vector1::build(99.));
    }

    #[test]
    fn improved_swizzle() {
        let vector = Vector3::build(99, 1, 1);
        let vec = swizzle!(vector; x, z, 3);
        assert!(vec == Vector3::build(99., 1., 3.))
    }
}
