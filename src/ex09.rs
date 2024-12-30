use num::{complex::ComplexFloat, Complex};

use crate::matrix::Matrix;

impl<K: Copy + Default + std::ops::AddAssign + Conjugate, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn transpose(&self) -> Matrix<K, M, N> {
        let mut data = [[K::default(); M]; N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[j][i].conjugate();
            }
        }
        Matrix { data }
    }
}

pub trait Conjugate {
    fn conjugate(&self) -> Self;
}

impl Conjugate for f32 {
    fn conjugate(&self) -> f32 {
        *self
    }
}

impl Conjugate for f64 {
    fn conjugate(&self) -> f64 {
        *self
    }
}

impl Conjugate for Complex<f32> {
    fn conjugate(&self) -> Complex<f32> {
        Complex::new(self.re(), -self.im())
    }
}

impl Conjugate for Complex<f64> {
    fn conjugate(&self) -> Complex<f64> {
        Complex::new(self.re(), -self.im())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert!(u.transpose().equals(&Matrix::from([[1., 0.], [0., 1.]])));
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert!(u.transpose().equals(&Matrix::from([[2., 4., -2.], [-5., 3., 3.], [0., 7., 4.]])));
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);
        assert!(u.transpose().equals(&Matrix::from([[-2., 1.], [-8., -23.], [4., 4.]])));
    }
}
