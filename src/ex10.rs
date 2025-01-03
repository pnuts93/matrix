use crate::matrix::Matrix;

impl<
        K: Copy
            + std::fmt::Debug
            + Default
            + std::cmp::PartialOrd
            + From<f32>
            + num::Signed
            + std::ops::Div<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::SubAssign,
        const N: usize,
        const M: usize,
    > Matrix<K, N, M>
{
    pub fn row_echelon(&self) -> Matrix<K, N, M> {
        let mut data = self.data.clone();
        let mut offset_n: usize = 0;
        let mut offset_m: usize = 0;
        while offset_m < M && offset_n < N {
            match switch_rows(&mut data, &mut offset_n, &mut offset_m, &mut 0) {
                Ok(_) => {
                    normalize_row(&mut data, offset_n, offset_m);
                    remove_first_entries(&mut data, offset_n, offset_m);
                    offset_m += 1;
                    offset_n += 1;
                }
                Err(_) => continue,
            };
        }
        offset_m = M - 1;
        offset_n = N - 1;
        while offset_m > 0 && offset_n > 0 {
            find_next_pivot(&mut data, &mut offset_n, &mut offset_m);
            remove_last_entries(&mut data, offset_n, offset_m);
            if offset_n != 0 && offset_m != 0 {
                offset_m -= 1;
                offset_n -= 1;
            }
        }
        Matrix { data }
    }

    pub fn row_echelon_count(&self, switch_counter: &mut usize) -> Matrix<K, N, M> {
        let mut data = self.data.clone();
        let mut offset_n: usize = 0;
        let mut offset_m: usize = 0;
        while offset_m < M && offset_n < N {
            match switch_rows(&mut data, &mut offset_n, &mut offset_m, switch_counter) {
                Ok(_) => {
                    remove_first_entries(&mut data, offset_n, offset_m);
                    offset_m += 1;
                    offset_n += 1;
                }
                Err(_) => continue,
            };
        }
        Matrix { data }
    }
}

fn switch_rows<
    K: Copy + Default + From<f32> + std::cmp::PartialOrd + num::Signed,
    const N: usize,
    const M: usize,
>(
    data: &mut [[K; N]; M],
    offset_n: &mut usize,
    offset_m: &mut usize,
    switch_counter: &mut usize,
) -> Result<(), ZeroedColumnError> {
    let max_row = find_max_row(data, offset_n, offset_m)?;
    if max_row != *offset_m {
        data.swap(*offset_m, max_row);
        *switch_counter += 1;
    }
    Ok(())
}

fn find_max_row<
    K: Copy + Default + From<f32> + std::cmp::PartialOrd + num::Signed,
    const N: usize,
    const M: usize,
>(
    data: &[[K; N]; M],
    offset_n: &mut usize,
    offset_m: &mut usize,
) -> Result<usize, ZeroedColumnError> {
    let mut max_row = *offset_m;
    for i in *offset_m..M {
        if data[i][*offset_n].abs() > data[max_row][*offset_n].abs() {
            max_row = i;
        }
    }
    if max_row == *offset_m && data[max_row][*offset_n] == K::from(0.) {
        *offset_n += 1;
        return Err(ZeroedColumnError);
    }
    Ok(max_row)
}

fn normalize_row<
    K: Copy
        + Default
        + std::cmp::PartialOrd
        + From<f32>
        + std::ops::Div<Output = K>
        + std::ops::Mul<Output = K>,
    const N: usize,
    const M: usize,
>(
    data: &mut [[K; N]; M],
    offset_n: usize,
    offset_m: usize,
) {
    if data[offset_m][offset_n] != K::from(1.) {
        let factor = K::from(1.) / data[offset_m][offset_n];
        for i in offset_n..N {
            data[offset_m][i] = data[offset_m][i] * factor;
        }
    }
}

fn remove_first_entries<
    K: Copy
        + Default
        + std::cmp::PartialOrd
        + From<f32>
        + std::ops::Mul<Output = K>
        + std::ops::Div<Output = K>
        + std::ops::SubAssign,
    const N: usize,
    const M: usize,
>(
    data: &mut [[K; N]; M],
    offset_n: usize,
    offset_m: usize,
) {
    for i in offset_m + 1..M {
        if data[i][offset_n] == K::from(0.) {
            continue;
        }
        let factor = data[offset_m][offset_n];
        let row = data[offset_m].map(|x| x * (data[i][offset_n] / factor));
        for j in offset_n..N {
            data[i][j] -= row[j];
        }
    }
}

fn find_next_pivot<
    K: Copy + Default + std::cmp::PartialOrd + From<f32>,
    const N: usize,
    const M: usize,
>(
    data: &mut [[K; N]; M],
    offset_n: &mut usize,
    offset_m: &mut usize,
) {
    if let Some(new_offset_n) = data[*offset_m].iter().position(|x| *x == K::from(1.)) {
        *offset_n = new_offset_n;
    } else {
        *offset_m -= 1;
        find_next_pivot(data, offset_n, offset_m);
    }
}

fn remove_last_entries<
    K: Copy
        + Default
        + std::cmp::PartialOrd
        + From<f32>
        + std::ops::Mul<Output = K>
        + std::ops::SubAssign,
    const N: usize,
    const M: usize,
>(
    data: &mut [[K; N]; M],
    offset_n: usize,
    offset_m: usize,
) {
    for i in (0..offset_m).rev() {
        if data[i][offset_n] == K::from(0.) {
            continue;
        }
        let row = data[offset_m].map(|x| x * data[i][offset_n]);
        for j in offset_n..N {
            data[i][j] -= row[j];
        }
    }
}

#[derive(Debug, Clone)]
struct ZeroedColumnError;

impl std::fmt::Display for ZeroedColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "All elements in matrix column are zeroes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_echelon() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert!(u
            .row_echelon()
            .equals(&Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])));
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert!(u.row_echelon().equals(&Matrix::from([[1., 0.], [0., 1.]])));
        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert!(u.row_echelon().equals(&Matrix::from([[1., 2.], [0., 0.]])));
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert!(u.row_echelon().equals(&Matrix::from([
            [1., 0.625, 0., 0., -12.1666667],
            [0., 0., 1., 0., -3.6666667],
            [0., 0., 0., 1., 29.5]
        ])));
    }
}
