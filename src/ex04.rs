use num::{complex::ComplexFloat, Complex};
use num_traits::Pow;

use crate::vector::Vector;

impl<
        K: Copy
            + Default
            + From<f32>
            + std::cmp::PartialOrd
            + num::Signed
            + PartialEq
            + std::ops::Mul<Output = K>
            + std::ops::Add<Output = K>
            + Modulus,
        const N: usize,
    > Vector<K, N>
{
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|x| x.modulus()).sum()
    }

    pub fn norm(&self) -> f32 {
        let mut res = f32::default();
        for i in 0..self.size() {
            let modulus = self.data[i].modulus();
            res = modulus.mul_add(modulus, res);
        }
        res.pow(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        self.data.iter().map(|x| x.modulus()).fold(0., f32::max)
    }
}

pub trait Modulus {
    fn modulus(&self) -> f32;
}

impl Modulus for f32 {
    fn modulus(&self) -> f32 {
        let res = *self;
        if res < 0. {
            return -res;
        }
        res
    }
}

impl Modulus for f64 {
    fn modulus(&self) -> f32 {
        let res = *self as f32;
        if res < 0. {
            return -res;
        }
        res
    }
}

impl Modulus for Complex<f32> {
    fn modulus(&self) -> f32 {
        (self.re().pow(2) as f32 + self.im().pow(2) as f32).pow(0.5)
    }
}

impl Modulus for Complex<f64> {
    fn modulus(&self) -> f32 {
        (self.re().pow(2) as f32 + self.im().pow(2) as f32).pow(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_1() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.);
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.);
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.);
    }

    #[test]
    fn test_norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm(), 0.);
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm(), 3.74165738);
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm(), 2.236067977);
    }

    #[test]
    fn test_norm_inf() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_inf(), 0.);
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_inf(), 3.);
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_inf(), 2.);
    }
}
