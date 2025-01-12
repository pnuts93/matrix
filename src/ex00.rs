use crate::{matrix::Matrix, vector::Vector};

impl<K: std::ops::Add<Output = K> + Copy + Default> Vector<K> {
    /// Adds two vectors element-wise.
    ///
    /// # Arguments
    ///
    /// * `v` - Another vector to add.
    ///
    /// # Returns
    ///
    /// A new vector that is the element-wise sum of the two vectors.
    ///
    /// # Panics
    ///
    /// Panics if the vectors are not the same size.
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
}
impl<K: std::ops::Sub<Output = K> + Copy + Default> Vector<K> {
    /// Subtracts one vector from another element-wise.
    ///
    /// # Arguments
    ///
    /// * `v` - Another vector to subtract.
    ///
    /// # Returns
    ///
    /// A new vector that is the element-wise difference of the two vectors.
    ///
    /// # Panics
    ///
    /// Panics if the vectors are not the same size.
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
}

impl<K: Copy + Default> Vector<K> {
    /// Multiplies a vector by a scalar.
    ///
    /// # Arguments
    ///
    /// * `a` - The scalar to multiply by.
    ///
    /// # Returns
    ///
    /// A new vector that is the original vector scaled by the scalar.
    pub fn _scl<F>(&self, a: F) -> Self
    where
        K: std::ops::Mul<F, Output = K>,
        F: Copy,
    {
        let mut res = Vector::from(vec![K::default(); self.size()]);
        for i in 0..self.size() {
            res.data[i] = self.data[i] * a;
        }
        res
    }
}
impl<K: std::ops::Add<Output = K> + Copy + Default> Matrix<K> {
    /// Adds two matrices element-wise.
    ///
    /// # Arguments
    ///
    /// * `m` - Another matrix to add.
    ///
    /// # Returns
    ///
    /// A new matrix that is the element-wise sum of the two matrices.
    ///
    /// # Panics
    ///
    /// Panics if the matrices do not have the same shape.
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
}
impl<K: std::ops::Sub<Output = K> + Copy + Default> Matrix<K> {
    /// Subtracts one matrix from another element-wise.
    ///
    /// # Arguments
    ///
    /// * `m` - Another matrix to subtract.
    ///
    /// # Returns
    ///
    /// A new matrix that is the element-wise difference of the two matrices.
    ///
    /// # Panics
    ///
    /// Panics if the matrices do not have the same shape.
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
}
impl<K: Copy + Default> Matrix<K> {
    /// Multiplies a matrix by a scalar.
    ///
    /// # Arguments
    ///
    /// * `a` - The scalar to multiply by.
    ///
    /// # Returns
    ///
    /// A new matrix that is the original matrix scaled by the scalar.
    pub fn _scl<F>(&self, a: F) -> Self
    where
        K: std::ops::Mul<F, Output = K>,
        F: Copy,
    {
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
