use crate::{ex04::Modulus, matrix::Matrix};

impl<
        K: Copy
            + std::fmt::Debug
            + Default
            + std::cmp::PartialOrd
            + From<f32>
            + Modulus
            + std::ops::Div<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::SubAssign,
    > Matrix<K>
{
    /// Calculates the inverse of the matrix.
    ///
    /// # Returns
    ///
    /// The inverse of the matrix.
    pub fn inverse(&self) -> Matrix<K> {
        let mut data = vec![];
        let n = self.shape()[0];
        for i in 0..n {
            data.push(
                [
                    self.data[i].clone(),
                    (0..n)
                        .map(|j| if i == j { K::from(1.) } else { K::from(0.) })
                        .collect(),
                ]
                .concat(),
            );
        }
        println!("{:?}", data);
        let m = Matrix::from(data).row_echelon();
        println!("{}", m);
        let mut inverse_data = vec![vec![K::default(); n]; n];
        for i in 0..n {
            inverse_data[i] = m.data[i][n..].to_vec();
        }
        Matrix::from(inverse_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.inverse(),
            Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(
            u.inverse(),
            Matrix::from([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]])
        );
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(
            u.inverse(),
            Matrix::from([
                [0.649425287, 0.097701149, -0.655172414],
                [-0.781609195, -0.126436782, 0.965517241],
                [0.143678161, 0.074712644, -0.206896552]
            ])
        );
    }
}
