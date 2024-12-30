use num_traits::MulAdd;

pub fn lerp<V: MulAdd<f32, V, Output = V> + std::ops::Sub<Output = V> + Copy + std::ops::Mul<f32, Output = V>>(
    u: V,
    v: V,
    t: f32,
) -> V {
    (u).mul_add(1. - t, v * t)
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, vector::Vector};

    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.);
        assert_eq!(lerp(0., 1., 1.), 1.);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert!(lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3).equals(&Vector::from([2.6, 1.3])));
        assert!(lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20., 10.], [30., 40.]]), 0.5).equals(&Matrix::from([[11., 5.5], [16.5, 22.]])));
    }
}
