use crate::{matrix::Matrix, vector::Vector};

impl<
        K: Copy
            + Default
            + std::ops::Mul<Output = K>
            + std::ops::AddAssign
            + num_traits::MulAdd<Output = K>,
        const N: usize,
        const M: usize,
    > Matrix<K, N, M>
{
    pub fn mul_vec(&self, vec: &Vector<K, N>) -> Vector<K, N> {
        let mut data = [K::default(); N];
        for i in 0..N {
            for j in 0..M {
                data[i] = vec.data[j].mul_add(self.data[i][j], data[i]);
            }
        }
        Vector { data }
    }
    pub fn mul_mat<const P: usize>(&self, mat: &Matrix<K, P, M>) -> Matrix<K, P, M> {
        let mut data = [[K::default(); P]; M];
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
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
        assert!(u.mul_vec(&v).equals(&Vector::from([4., 2.])));
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert!(u.mul_vec(&v).equals(&Vector::from([8., 4.])));
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert!(u.mul_vec(&v).equals(&Vector::from([4., -4.])));
    }

    #[test]
    fn test_mul_mat() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert!(u.mul_mat(&v).equals(&Matrix::from([[1., 0.], [0., 1.]])));
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert!(u.mul_mat(&v).equals(&Matrix::from([[2., 1.], [4., 2.]])));
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert!(u.mul_mat(&v).equals(&Matrix::from([[-14., -7.], [44., 22.]])));
    }
}
