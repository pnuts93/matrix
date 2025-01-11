use crate::vector::Vector;

/// Computes the cross product of two vectors.
/// 
/// # Arguments
/// 
/// * `u` - The first vector.
/// * `v` - The second vector.
/// 
/// # Returns
/// 
/// The cross product of the two vectors.
pub fn cross_product<K: Copy + Default + std::ops::Mul<Output = K> + std::ops::Sub<Output = K>>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> Vector<K> {
    let mut data = [K::default(); 3];
    data[0] = u.data[1] * v.data[2] - u.data[2] * v.data[1];
    data[1] = u.data[2] * v.data[0] - u.data[0] * v.data[2];
    data[2] = u.data[0] * v.data[1] - u.data[1] * v.data[0];
    Vector::from(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));

        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
    }
}
