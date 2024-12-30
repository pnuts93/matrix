use crate::matrix::Matrix;

impl<K: Copy + Default + std::ops::AddAssign, const N: usize> Matrix<K, N, N> {
    pub fn trace(&self) -> K {
        let mut res = K::default();
        for i in 0..N {
            res += self.data[i][i];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(2., u.trace());
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(9., u.trace());
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(-21., u.trace());
    }
}
