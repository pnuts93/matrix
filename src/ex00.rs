use crate::{matrix::Matrix, vector::Vector};

impl<
        K: std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + From<f32>
            + Copy
            + Default,
    > Vector<K>
{
    pub fn _add(&self, v: &Vector<K>) -> Self {
        if !self.is_same_size(v) {
            panic!()
        }
        let mut res = Vector::from(vec![K::default(); self.size()]);
        for i in 0..self.size() {
            res.data[i] = self.data[i] + v.data[i];
        }
        res
    }
    pub fn _sub(&self, v: &Vector<K>) -> Self {
        if !self.is_same_size(v) {
            panic!()
        }
        let mut res = Vector::from(vec![K::default(); self.size()]);
        for i in 0..self.size() {
            res.data[i] = self.data[i] - v.data[i];
        }
        res
    }
    pub fn _scl(&self, a: K) -> Self {
        let mut res = Vector::from(vec![K::default(); self.size()]);
        for i in 0..self.size() {
            res.data[i] = self.data[i] * a;
        }
        res
    }
}
impl<
        K: std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + Copy
            + Default
            + From<f32>,
    > Matrix<K>
{
    pub fn _add(&self, m: &Matrix<K>) -> Self {
        if !self.is_same_shape(m) {
            panic!()
        }
        let mut res = Matrix::from(vec![vec![K::default(); self.shape()[0]]; self.shape()[1]]);
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
                res.data[i][j] = self.data[i][j] + m.data[i][j];
            }
        }
        res
    }
    pub fn _sub(&self, m: &Matrix<K>) -> Self {
        if !self.is_same_shape(m) {
            panic!()
        }
        let mut res = Matrix::from(vec![vec![K::default(); self.shape()[0]]; self.shape()[1]]);
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
                res.data[i][j] = self.data[i][j] - m.data[i][j];
            }
        }
        res
    }
    pub fn _scl(&self, a: K) -> Self {
        let mut res = Matrix::from(vec![vec![K::default(); self.shape()[0]]; self.shape()[1]]);
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
                res.data[i][j] = self.data[i][j] * a;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u._add(&v);
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([7., 10.]));
    }

    #[test]
    fn test_vector_sub() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u._sub(&v);
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([-3., -4.]));
    }

    #[test]
    fn test_vector_scl() {
        let u = Vector::from([2., 3.]);
        let w = u._scl(2.);
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([4., 6.]));
    }

    #[test]
    fn test_matrix_add() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let w = u._add(&v);
        assert_eq!(w, Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_matrix_sub() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let w = u._sub(&v);
        assert_eq!(w, Matrix::from([[-6.0, -2.0], [5.0, 2.0]]));
    }

    #[test]
    fn test_matrix_scl() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let w = u._scl(2.);
        assert_eq!(w, Matrix::from([[2.0, 4.0], [6.0, 8.0]]));
    }
}
