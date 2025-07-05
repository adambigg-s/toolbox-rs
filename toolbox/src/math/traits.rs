use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait PrimitiveNumber<T>
where
    Self: Zero<T> + One<T> + Default + Clone + Copy,
{
}

pub trait FloatNumber<T>
where
    Self: PrimitiveNumber<T>,
{
}

pub trait Adding<T>
where
    Self: PrimitiveNumber<T> + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
}

pub trait Multiplying<T>
where
    Self: PrimitiveNumber<T> + Mul<Output = T> + Div<Output = T> + MulAssign + DivAssign + Neg<Output = T>,
{
}

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
