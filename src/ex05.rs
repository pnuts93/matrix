use crate::{ex04::Modulus, vector::Vector};

/// Computes the cosine of the angle between two vectors.
///
/// # Arguments
///
/// * `u` - The first vector.
/// * `v` - The second vector.
///
/// # Returns
///
/// The cosine of the angle between the two vectors.
pub fn angle_cos<
    K: Copy
        + Default
        + Modulus
        + PartialEq
        + From<f32>
        + std::ops::Add<Output = K>
        + std::ops::Mul<Output = K>
        + std::ops::Div<Output = K>
        + num_traits::Pow<f32, Output = K>
        + num_traits::ops::mul_add::MulAdd<Output = K>,
>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> K {
    u.dot(v) / (u.dot(u) * v.dot(v)).pow(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_cos() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.);
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.);
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert_eq!(angle_cos(&u, &v), -1.);
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.);
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(angle_cos(&u, &v), 0.9746318461970762);
    }
}
