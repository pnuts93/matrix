use crate::vector::Vector;
use num_traits::MulAdd;

/// Compute the linear combination of vectors.
/// 
/// # Arguments
/// 
/// * `u` - A slice of vectors.
/// * `coefs` - A slice of coefficients.
/// 
/// # Returns
/// 
/// A new vector that is the linear combination of the input vectors.
pub fn linear_combination<
    K: std::marker::Copy + Default + MulAdd<Output = K> + std::fmt::Display,
>(
    u: &[&Vector<K>],
    coefs: &[K],
) -> Vector<K> {
    let mut res = vec![K::default(); u[0].size()];
    for i in 0..u[0].size() {
        for j in 0..u.len() {
            res[i] = u[j].data[i].mul_add(coefs[j], res[i]);
        }
    }
    Vector::from(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        assert_eq!(
            Vector::from([10., -2., 0.5]),
            linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5])
        );

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(
            Vector::from([10., 0., 230.]),
            linear_combination(&[&v1, &v2], &[10., -2.])
        );
    }
}
