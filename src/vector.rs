use crate::Equals;

/// A generic Vector struct that holds a 1D vector of data.
#[derive(Clone, Debug)]
pub struct Vector<K> {
    /// The data of the Vector.
    pub data: Vec<K>,
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    /// Converts a fixed-size array into a Vector.
    ///
    /// # Arguments
    ///
    /// * `value` - An array of elements.
    ///
    /// # Returns
    ///
    /// A Vector containing the elements of the array.
    fn from(value: [K; N]) -> Self {
        Vector {
            data: value.to_vec(),
        }
    }
}

impl<K: Clone> From<Vec<K>> for Vector<K> {
    /// Converts a vector into a Vector.
    ///
    /// # Arguments
    ///
    /// * `value` - A vector of elements.
    ///
    /// # Returns
    ///
    /// A Vector containing the elements of the vector.
    fn from(value: Vec<K>) -> Self {
        Vector { data: value }
    }
}

impl<K: Equals> PartialEq for Vector<K> {
    /// Checks if two Vectors are equal by comparing their elements.
    ///
    /// # Arguments
    ///
    /// * `v` - Another Vector to compare with.
    ///
    /// # Returns
    ///
    /// `true` if all elements are equal, `false` otherwise.
    fn eq(&self, v: &Self) -> bool {
        v.data.iter().zip(v.data.iter()).all(|(a, b)| a.equals(b))
    }
}

impl<K> Vector<K> {
    /// Returns the size of the Vector.
    ///
    /// # Returns
    ///
    /// The number of elements in the Vector.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Checks if another Vector has the same size as this one.
    ///
    /// # Arguments
    ///
    /// * `v` - Another Vector to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the sizes are the same, `false` otherwise.
    pub fn is_same_size(&self, v: &Vector<K>) -> bool {
        self.size() == v.size()
    }
}

impl<K: std::fmt::Debug> std::fmt::Display for Vector<K> {
    /// Formats the Vector for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the Vector.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for i in &self.data {
            res.push_str(&format!("[ {:?} ]\n", i));
        }
        write!(f, "{}", res)
    }
}

impl<K: Copy + Default + num_traits::ops::mul_add::MulAdd<f32, K, Output = K>>
    num_traits::ops::mul_add::MulAdd<f32, Self> for Vector<K>
{
    type Output = Self;

    /// Multiplies each element of the Vector by a scalar and adds another Vector.
    ///
    /// # Arguments
    ///
    /// * `a` - The scalar to multiply each element by.
    /// * `b` - The Vector to add.
    ///
    /// # Returns
    ///
    /// A new Vector with the result of the operation.
    fn mul_add(self, a: f32, b: Self) -> Self::Output {
        let mut res = Vector::from(vec![K::default(); self.size()]);
        for i in 0..self.size() {
            res.data[i] = self.data[i].mul_add(a, b.data[i]);
        }
        res
    }
}

impl<K: Copy + Default + std::ops::Sub<Output = K>> std::ops::Sub for Vector<K> {
    /// Subtracts two Vectors.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The Vector to subtract.
    ///
    /// # Returns
    ///
    /// A new Vector with the result of the operation.
    fn sub(self, rhs: Self) -> Self::Output {
        self._sub(&rhs)
    }

    type Output = Self;
}

impl<K: Copy + Default + std::ops::Mul<F, Output = K>, F: Copy> std::ops::Mul<F> for Vector<K> {
    /// Multiplies a Vectors by a scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The scalar to multiply the vector by.
    ///
    /// # Returns
    ///
    /// A new Vector with the result of the operation.
    fn mul(self, rhs: F) -> Self::Output {
        self._scl(rhs)
    }

    type Output = Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([5.]);
        assert_eq!(v1.size(), 3);
        assert_eq!(v2.size(), 1);
    }
}
