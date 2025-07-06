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

macro_rules! impl_zero {
    ($type:ty) => {
        impl Zero<$type> for $type {
            fn zero() -> $type {
                return 0 as $type;
            }
        }
    };
}

impl_zero!(u8);
impl_zero!(u16);
impl_zero!(u32);
impl_zero!(u64);
impl_zero!(u128);
impl_zero!(i8);
impl_zero!(i16);
impl_zero!(i32);
impl_zero!(i64);
impl_zero!(i128);
impl_zero!(usize);
impl_zero!(isize);
impl_zero!(f32);
impl_zero!(f64);

pub trait One<T> {
    fn one() -> T;
}

macro_rules! impl_one {
    ($type:ty) => {
        impl One<$type> for $type {
            fn one() -> $type {
                return 1 as $type;
            }
        }
    };
}

impl_one!(u8);
impl_one!(u16);
impl_one!(u32);
impl_one!(u64);
impl_one!(u128);
impl_one!(i8);
impl_one!(i16);
impl_one!(i32);
impl_one!(i64);
impl_one!(i128);
impl_one!(usize);
impl_one!(isize);
impl_one!(f32);
impl_one!(f64);
