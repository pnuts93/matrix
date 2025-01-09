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
            + std::ops::SubAssign,
    > Matrix<K>
{
    pub fn rank(&self) -> usize {
        let row_echelon_form = self.row_echelon();
        let mut rank = 0;
        for i in 0..self.shape()[1] {
            for j in 0..self.shape()[0] {
                if row_echelon_form.data[i][j] != K::default() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(3, u.rank());
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        assert_eq!(2, u.rank());
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        assert_eq!(3, u.rank());
    }
}
