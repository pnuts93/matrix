use num::complex::ComplexFloat;

use crate::{matrix::Matrix, Complex};

impl<K: Copy + Default + std::ops::AddAssign + Conjugate> Matrix<K> {
    /// Computes the conjugate transpose of a matrix.
    ///
    /// # Returns
    ///
    /// The conjugate transpose of the matrix.
    pub fn transpose(&self) -> Matrix<K> {
        let mut data = vec![vec![K::default(); self.shape()[1]]; self.shape()[0]];
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
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
        Complex(num::Complex::new(self.0.re(), -self.0.im()))
    }
}

impl Conjugate for Complex<f64> {
    fn conjugate(&self) -> Complex<f64> {
        Complex(num::Complex::new(self.0.re(), -self.0.im()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.transpose(), Matrix::from([[1., 0.], [0., 1.]]));
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[2., 4., -2.], [-5., 3., 3.], [0., 7., 4.]])
        );
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[-2., 1.], [-8., -23.], [4., 4.]])
        );
    }
}
