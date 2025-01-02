#[derive(Clone, Copy)]
pub struct Vector<K, const N: usize> {
    pub data: [K; N],
}

impl<K, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(value: [K; N]) -> Self {
        Vector { data: value }
    }
}

impl<
        K: Copy
            + From<f32>
            + std::cmp::PartialOrd
            + std::ops::Sub<Output = K>
            + num::Signed
            + std::cmp::PartialEq,
        const N: usize,
    > Vector<K, N>
{
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_same_size(&self, v: &Vector<K, N>) -> bool {
        self.size() == v.size()
    }

    pub fn equals(&self, v: &Vector<K, N>) -> bool {
        self.data
            .iter()
            .zip(v.data.iter())
            .all(|(a, b)| (*a - *b).abs() < K::from(1e-6))
    }
}

impl<K: std::fmt::Debug, const N: usize> std::fmt::Display for Vector<K, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for i in &self.data {
            res.push_str(&format!("[ {:?} ]\n", i));
        }
        write!(f, "{}", res)
    }
}

impl<
        K: Copy
            + Default
            + From<f32>
            + std::cmp::PartialOrd
            + std::cmp::PartialEq
            + num::Signed
            + num_traits::ops::mul_add::MulAdd<f32, K, Output = K>
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
        const N: usize,
    > num_traits::ops::mul_add::MulAdd<f32, Self> for Vector<K, N>
{
    type Output = Self;
    fn mul_add(self, a: f32, b: Self) -> Self::Output {
        let mut res = Vector::from([K::default(); N]);
        for i in 0..self.size() {
            res.data[i] = self.data[i].mul_add(a, b.data[i]);
        }
        res
    }
}

impl<
        K: Copy
            + Default
            + From<f32>
            + std::cmp::PartialOrd
            + num::Signed
            + std::cmp::PartialEq
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
        const N: usize,
    > std::ops::Sub for Vector<K, N>
{
    fn sub(self, rhs: Self) -> Self::Output {
        self._sub(&rhs)
    }

    type Output = Self;
}

impl<
        K: Copy
            + Default
            + From<f32>
            + std::cmp::PartialOrd
            + num::Signed
            + std::cmp::PartialEq
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
        const N: usize,
    > std::ops::Mul<K> for Vector<K, N>
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
    fn test_size() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([5.]);
        assert_eq!(v1.size(), 3);
        assert_eq!(v2.size(), 1);
    }
}
