use crate::{matrix::Matrix, vector::Vector};

impl<
        K: Copy
            + Default
            + std::ops::Mul<Output = K>
            + std::ops::AddAssign
            + num_traits::MulAdd<Output = K>,
    > Matrix<K>
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mut data = vec![K::default(); vec.size()];
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
                data[i] = vec.data[j].mul_add(self.data[i][j], data[i]);
            }
        }
        Vector { data }
    }
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let n = self.shape()[0];
        let m = self.shape()[1];
        let p = mat.shape()[0];
        let mut data = vec![vec![K::default(); p]; m];
        for i in 0..m {
            for j in 0..p {
                for k in 0..n {
                    data[i][j] = mat.data[k][j].mul_add(self.data[i][k], data[i][j]);
                }
            }
        }
        Matrix { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_vec() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.]));
    }

    #[test]
    fn test_mul_mat() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    }
}
