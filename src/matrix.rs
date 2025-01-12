use crate::Equals;

/// A generic Matrix struct that holds a 2D vector of data.
#[derive(Clone, Debug)]
pub struct Matrix<K> {
    /// The data of the Matrix.
    pub data: Vec<Vec<K>>,
}

impl<K: Clone, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K> {
    /// Converts a fixed-size 2D array into a Matrix.
    ///
    /// # Arguments
    ///
    /// * `value` - A 2D array of elements.
    ///
    /// # Returns
    ///
    /// A Matrix containing the elements of the 2D array.
    fn from(value: [[K; N]; M]) -> Self {
        Matrix {
            data: value.map(|arr| arr.to_vec()).to_vec(),
        }
    }
}

impl<K: Clone> From<Vec<Vec<K>>> for Matrix<K> {
    /// Converts a 2D vector into a Matrix.
    ///
    /// # Arguments
    ///
    /// * `value` - A 2D vector of elements.
    ///
    /// # Returns
    ///
    /// A Matrix containing the elements of the 2D vector.
    fn from(value: Vec<Vec<K>>) -> Self {
        Matrix { data: value }
    }
}

impl<K> Matrix<K> {
    /// Returns the shape of the Matrix as a 2-element array.
    ///
    /// # Returns
    ///
    /// An array containing the number of columns and rows in the Matrix.
    pub fn shape(&self) -> [usize; 2] {
        [self.data[0].len(), self.data.len()]
    }

    /// Checks if another Matrix has the same shape as this one.
    ///
    /// # Arguments
    ///
    /// * `m` - Another Matrix to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the shapes are the same, `false` otherwise.
    pub fn is_same_shape(&self, m: &Matrix<K>) -> bool {
        self.shape()
            .iter()
            .zip(m.shape().iter())
            .all(|(a, b)| a == b)
    }
}

impl<K: Equals> PartialEq for Matrix<K> {
    /// Checks if two Matrices are equal by comparing their elements.
    ///
    /// # Arguments
    ///
    /// * `v` - Another Matrix to compare with.
    ///
    /// # Returns
    ///
    /// `true` if all elements are equal, `false` otherwise.
    fn eq(&self, v: &Self) -> bool {
        self.data
            .iter()
            .zip(v.data.iter())
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(ax, by)| ax.equals(by)))
    }
}

impl<K: std::fmt::Debug> std::fmt::Display for Matrix<K> {
    /// Formats the Matrix for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the Matrix.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for i in &self.data {
            res.push_str("[ ");
            for (count, j) in i.iter().enumerate() {
                if count > 0 {
                    res.push_str(", ");
                }
                res.push_str(&format!("{:?}", j));
            }
            res.push_str(" ]\n");
        }
        write!(f, "{}", res)
    }
}

impl<K: Copy + Default + num_traits::ops::mul_add::MulAdd<f32, K, Output = K>>
    num_traits::ops::mul_add::MulAdd<f32, Self> for Matrix<K>
{
    type Output = Self;

    /// Multiplies each element of the Matrix by a scalar and adds another Matrix.
    ///
    /// # Arguments
    ///
    /// * `a` - The scalar to multiply each element by.
    /// * `b` - The Matrix to add.
    ///
    /// # Returns
    ///
    /// A new Matrix with the result of the operation.
    fn mul_add(self, a: f32, b: Self) -> Self::Output {
        let mut res = Matrix::from(vec![vec![K::default(); self.shape()[0]]; self.shape()[1]]);
        for i in 0..self.shape()[0] {
            for j in 0..self.shape()[1] {
                res.data[i][j] = self.data[i][j].mul_add(a, b.data[i][j]);
            }
        }
        res
    }
}

impl<K: Copy + Default + std::ops::Sub<Output = K>> std::ops::Sub for Matrix<K> {
    /// Subtracts two Matrices.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The Matrix to subtract.
    ///
    /// # Returns
    ///
    /// A new Matrix with the result of the operation.
    fn sub(self, rhs: Self) -> Self::Output {
        self._sub(&rhs)
    }

    type Output = Self;
}

impl<K: Copy + Default + std::ops::Mul<F, Output = K>, F: Copy> std::ops::Mul<F> for Matrix<K> {
    /// Multiplies a Matrix by a scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The scalar to multiply by.
    ///
    /// # Returns
    ///
    /// A new Matrix with the result of the operation.
    fn mul(self, rhs: F) -> Self::Output {
        self._scl(rhs)
    }

    type Output = Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let m1 = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        let m2 = Matrix::from([[5.]]);
        assert_eq!(m1.shape(), [3, 2]);
        assert_eq!(m2.shape(), [1, 1]);
    }
}
