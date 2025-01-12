use num::complex::ComplexFloat;

pub mod ex00;
pub mod ex01;
pub mod ex02;
pub mod ex03;
pub mod ex04;
pub mod ex05;
pub mod ex06;
pub mod ex07;
pub mod ex08;
pub mod ex09;
pub mod ex10;
pub mod ex11;
pub mod ex12;
pub mod ex13;
pub mod ex14;
pub mod ex15;
pub mod matrix;
pub mod vector;

pub trait Equals {
    fn equals(&self, v: &Self) -> bool;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex<K>(num::Complex<K>);

impl Equals for f32 {
    fn equals(&self, v: &Self) -> bool {
        (*self - *v).abs() < 1e-6
    }
}

impl Equals for f64 {
    fn equals(&self, v: &Self) -> bool {
        (*self - *v).abs() < 1e-6
    }
}

impl Equals for Complex<f64> {
    fn equals(&self, v: &Self) -> bool {
        (self.0.re() - v.0.re()).abs() < 1e-6 && (self.0.im() - v.0.im()).abs() < 1e-6
    }
}

impl Equals for Complex<f32> {
    fn equals(&self, v: &Self) -> bool {
        (self.0.re() - v.0.re()).abs() < 1e-6 && (self.0.im() - v.0.im()).abs() < 1e-6
    }
}

pub trait Magnitude {
    fn magnitude(&self) -> f64;
}

impl Magnitude for Complex<f32> {
    fn magnitude(&self) -> f64 {
        self.0.norm() as f64
    }
}

impl Magnitude for Complex<f64> {
    fn magnitude(&self) -> f64 {
        self.0.norm()
    }
}

impl PartialOrd for Complex<f32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl PartialOrd for Complex<f64> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl From<f32> for Complex<f32> {
    fn from(f: f32) -> Self {
        Complex(num::Complex::new(f, 0.))
    }
}

impl From<f64> for Complex<f64> {
    fn from(f: f64) -> Self {
        Complex(num::Complex::new(f, 0.))
    }
}

impl<K> std::ops::Add for Complex<K>
where
    K: Copy + num_traits::Num,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0)
    }
}

impl<K> std::ops::AddAssign for Complex<K>
where
    K: Copy
        + num_traits::Num
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<K> std::ops::Sub for Complex<K>
where
    K: Copy + num_traits::Num,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex(self.0 - rhs.0)
    }
}

impl<K> std::ops::SubAssign for Complex<K>
where
    K: Copy
        + num_traits::Num
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<K> std::ops::Mul<Self> for Complex<K>
where
    K: Copy + num_traits::Num,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex(self.0 * rhs.0)
    }
}

impl<K> std::ops::Mul<K> for Complex<K>
where
    K: Copy + num_traits::Num,
{
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        Complex(self.0 * rhs)
    }
}

impl<K> std::ops::MulAssign for Complex<K>
where
    K: Copy
        + num_traits::Num
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<K> std::ops::Div for Complex<K>
where
    K: Copy + num_traits::Num,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Complex(self.0 / rhs.0)
    }
}

impl<K> std::ops::Neg for Complex<K>
where
    K: Copy + num_traits::Num + std::ops::Neg<Output = K>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex(-self.0)
    }
}

impl<K> num_traits::MulAdd<Self, Self> for Complex<K>
where
    K: Copy + num_traits::Num + num_traits::MulAdd<Output = K>,
{
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        Complex(self.0.mul_add(a.0, b.0))
    }
}

impl<K> num_traits::MulAdd<K, Self> for Complex<K>
where
    K: Copy + num_traits::Num + num_traits::MulAdd<Output = K>,
{
    type Output = Self;

    fn mul_add(self, a: K, b: Self) -> Self::Output {
        Complex(self.0.mul_add(num::Complex::from(a), b.0))
    }
}

impl<K> num_traits::Pow<f32> for Complex<K>
where
    K: num::Float + std::convert::From<f32>,
{
    type Output = Self;

    fn pow(self, rhs: f32) -> Self::Output {
        Complex(self.0.pow(rhs))
    }
}
