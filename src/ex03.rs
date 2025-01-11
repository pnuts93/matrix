use num_traits::MulAdd;

use crate::vector::Vector;

impl<
        K: Copy
            + Default
            + From<f32>
            + PartialEq
            + std::ops::Mul<Output = K>
            + std::ops::Add<Output = K>
            + MulAdd<Output = K>,
    > Vector<K>
{
    /// Computes the dot product of two vectors.
    /// 
    /// # Arguments
    /// 
    /// * `v` - Another vector to compute the dot product with.
    /// 
    /// # Returns
    /// 
    /// The dot product of the two vectors.
    pub fn dot(&self, v: &Vector<K>) -> K {
        let mut res = K::default();
        for i in 0..self.size() {
            res = v.data[i].mul_add(self.data[i], res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(0., u.dot(&v));

        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(2., u.dot(&v));

        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        assert_eq!(9., u.dot(&v));
    }
}
