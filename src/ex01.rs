use crate::vector::Vector;
use num_traits::MulAdd;

pub fn linear_combination<K: std::marker::Copy + Default + MulAdd<Output = K> + std::fmt::Display, const N: usize>(
    u: &[&Vector<K, N>],
    coefs: &[K],
) -> Vector<K, N> {
    let mut res = [K::default(); N];
    for i in 0..N {
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
        assert!(Vector::from([10., -2., 0.5]).equals(&linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5])));

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert!(Vector::from([10., 0., 230.]).equals(&linear_combination(&[&v1, &v2], &[10., -2.])));
    }
}
