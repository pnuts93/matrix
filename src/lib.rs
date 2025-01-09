use num::Complex;

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
        *self == *v
    }
}

impl Equals for Complex<f32> {
    fn equals(&self, v: &Self) -> bool {
        *self == *v
    }
}
