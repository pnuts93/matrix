use crate::Equals;

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    fn from(value: [K; N]) -> Self {
        Vector {
            data: value.to_vec(),
        }
    }
}

impl<K: Clone> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        Vector { data: value }
    }
}

impl<K: Equals> PartialEq for Vector<K> {
    fn eq(&self, v: &Self) -> bool {
        v.data.iter().zip(v.data.iter()).all(|(a, b)| a.equals(b))
    }
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_same_size(&self, v: &Vector<K>) -> bool {
        self.size() == v.size()
    }
}

impl<K: std::fmt::Debug> std::fmt::Display for Vector<K> {
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
    > num_traits::ops::mul_add::MulAdd<f32, Self> for Vector<K>
{
    type Output = Self;
    fn mul_add(self, a: f32, b: Self) -> Self::Output {
        let mut res = Vector::from(vec![K::default(); self.size()]);
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
    > std::ops::Sub for Vector<K>
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
    > std::ops::Mul<K> for Vector<K>
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
