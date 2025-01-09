use crate::Equals;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub data: Vec<Vec<K>>,
}

impl<K: Clone, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K> {
    fn from(value: [[K; N]; M]) -> Self {
        Matrix {
            data: value.map(|arr| arr.to_vec()).to_vec(),
        }
    }
}

impl<K: Clone> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(value: Vec<Vec<K>>) -> Self {
        Matrix { data: value }
    }
}

impl<K> Matrix<K> {
    pub fn shape(&self) -> [usize; 2] {
        [self.data[0].len(), self.data.len()]
    }

    pub fn is_same_shape(&self, m: &Matrix<K>) -> bool {
        self.shape()
            .iter()
            .zip(m.shape().iter())
            .all(|(a, b)| a == b)
    }
}
impl<K: Equals> PartialEq for Matrix<K> {
    fn eq(&self, v: &Self) -> bool {
        self.data
            .iter()
            .zip(v.data.iter())
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(ax, by)| ax.equals(by)))
    }
}

impl<K: std::fmt::Debug> std::fmt::Display for Matrix<K> {
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

impl<
        K: Copy
            + Default
            + std::cmp::PartialEq
            + num_traits::ops::mul_add::MulAdd<f32, K, Output = K>
            + std::ops::Sub<Output = K>
            + num::Signed
            + std::cmp::PartialOrd
            + From<f32>,
    > num_traits::ops::mul_add::MulAdd<f32, Self> for Matrix<K>
{
    type Output = Self;
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

impl<
        K: Copy
            + Default
            + num::Signed
            + std::cmp::PartialOrd
            + From<f32>
            + std::cmp::PartialEq
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > std::ops::Sub for Matrix<K>
{
    fn sub(self, rhs: Self) -> Self::Output {
        self._sub(&rhs)
    }

    type Output = Self;
}

impl<
        K: Copy
            + Default
            + num::Signed
            + std::cmp::PartialOrd
            + From<f32>
            + std::cmp::PartialEq
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > std::ops::Mul<K> for Matrix<K>
{
    fn mul(self, rhs: K) -> Self::Output {
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
