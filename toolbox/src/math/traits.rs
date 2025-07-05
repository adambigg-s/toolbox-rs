use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait PrimitiveNumber
where
    Self: Default + Clone + Copy,
{
}

impl<T> PrimitiveNumber for T where T: Default + Clone + Copy {}

pub trait Adds<T>
where
    Self: PrimitiveNumber + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + Zero<T>,
{
}

impl<T> Adds<T> for T where
    T: PrimitiveNumber + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + Zero<T>
{
}

pub trait Multiplicative<T>
where
    Self: PrimitiveNumber
        + Mul<Output = T>
        + Div<Output = T>
        + MulAssign
        + DivAssign
        + Neg<Output = T>
        + One<T>,
{
}

impl<T> Multiplicative<T> for T where
    T: PrimitiveNumber + Mul<Output = T> + Div<Output = T> + MulAssign + DivAssign + Neg<Output = T> + One<T>
{
}

pub trait Numeric<T>
where
    Self: Adds<T> + Multiplicative<T> + PartialEq + PartialOrd,
{
}

impl<T> Numeric<T> for T where T: Adds<T> + Multiplicative<T> + PartialEq + PartialOrd {}

pub trait FloatNumber<T>
where
    Self: Numeric<T>,
{
}

impl<T> FloatNumber<T> for T where T: Numeric<T> {}

pub trait Zero<T> {
    fn zero() -> T;
}

impl Zero<i32> for i32 {
    fn zero() -> i32 {
        0
    }
}

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        0.
    }
}

pub trait One<T> {
    fn one() -> T;
}

impl One<i32> for i32 {
    fn one() -> i32 {
        1
    }
}

impl One<f32> for f32 {
    fn one() -> f32 {
        1.
    }
}
