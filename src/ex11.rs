use crate::matrix::Matrix;

impl<
        K: Copy
            + std::fmt::Debug
            + Default
            + std::cmp::PartialOrd
            + From<f32>
            + num::Signed
            + std::ops::Div<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::SubAssign
            + std::ops::MulAssign,
    > Matrix<K>
{
    /// Calculates the determinant of the matrix.
    /// 
    /// # Returns
    /// 
    /// The determinant of the matrix.
    pub fn determinant(&self) -> K {
        let mut switch_counter = 0;
        let row_echelon_form = self.row_echelon_count(&mut switch_counter);
        let mut determinant = K::from(1.);
        for i in 0..self.shape()[0] {
            if row_echelon_form.data[i][i] == K::default() {
                return K::default();
            }
            determinant *= row_echelon_form.data[i][i];
        }
        if switch_counter % 2 != 0 {
            determinant = -determinant;
        }
        determinant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinant() {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);
        assert_eq!(0., u.determinant());
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(8., u.determinant());
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(-174., u.determinant());
        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(1032., u.determinant());
    }
}
