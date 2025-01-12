use crate::{ex04::Modulus, matrix::Matrix};

impl<
        K: Copy
            + std::fmt::Debug
            + Default
            + std::cmp::PartialOrd
            + From<f32>
            + Modulus
            + std::ops::Div<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::SubAssign,
    > Matrix<K>
{
    /// Converts the matrix to its reduced row echelon form.
    ///
    /// # Returns
    ///
    /// A new matrix that is the row echelon form of the original matrix.
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut data = self.data.clone();
        let mut offset_n: usize = 0;
        let mut offset_m: usize = 0;
        while offset_m < self.shape()[1] && offset_n < self.shape()[0] {
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
        offset_m = self.shape()[1] - 1;
        offset_n = self.shape()[0] - 1;
        while offset_m > 0 && offset_n > 0 {
            match find_next_pivot(&mut data, &mut offset_n, &mut offset_m) {
                Ok(_) => {
                    remove_last_entries(&mut data, offset_n, offset_m);
                    if offset_n != 0 && offset_m != 0 {
                        offset_m -= 1;
                        offset_n -= 1;
                    }
                }
                Err(_) => break,
            }
        }
        Matrix { data }
    }

    /// Converts the matrix to its row echelon form and counts the number of row switches.
    ///
    /// # Arguments
    ///
    /// * `switch_counter` - A mutable reference to a counter that tracks the number of row switches.
    ///
    /// # Returns
    ///
    /// A new matrix that is the row echelon form of the original matrix.
    pub fn row_echelon_count(&self, switch_counter: &mut usize) -> Matrix<K> {
        let mut data = self.data.clone();
        let mut offset_n: usize = 0;
        let mut offset_m: usize = 0;
        while offset_m < self.shape()[1] && offset_n < self.shape()[0] {
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

/// Switches the rows of a matrix to move the row with the largest element to the top.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - A mutable reference to the column offset.
/// * `offset_m` - A mutable reference to the row offset.
/// * `switch_counter` - A mutable reference to the row switch counter.
///
/// # Returns
///
/// A Result containing `()` if the operation was successful, or a ZeroedColumnError if the column is all zeroes.
fn switch_rows<K: Copy + Default + From<f32> + std::cmp::PartialOrd + Modulus>(
    data: &mut Vec<Vec<K>>,
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

/// Finds the row with the largest element in a given column.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - A mutable reference to the column offset.
/// * `offset_m` - A mutable reference to the row offset.
///
/// # Returns
///
/// A Result containing the index of the row with the largest element in the column, or a ZeroedColumnError if the column is all zeroes.
fn find_max_row<K: Copy + Default + From<f32> + std::cmp::PartialOrd + Modulus>(
    data: &mut Vec<Vec<K>>,
    offset_n: &mut usize,
    offset_m: &mut usize,
) -> Result<usize, ZeroedColumnError> {
    let mut max_row = *offset_m;
    for i in *offset_m..data.len() {
        if data[i][*offset_n].modulus() > data[max_row][*offset_n].modulus() {
            max_row = i;
        }
    }
    if max_row == *offset_m && data[max_row][*offset_n] == K::from(0.) {
        *offset_n += 1;
        return Err(ZeroedColumnError);
    }
    Ok(max_row)
}

/// Normalizes the current pivot modifying the whole row.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - The column offset.
/// * `offset_m` - The row offset.
fn normalize_row<
    K: Copy + Default + std::cmp::PartialEq + From<f32> + std::ops::Div<Output = K>,
>(
    data: &mut Vec<Vec<K>>,
    offset_n: usize,
    offset_m: usize,
) {
    if data[offset_m][offset_n] != K::from(1.) {
        let factor = data[offset_m][offset_n];
        for i in offset_n..data[0].len() {
            data[offset_m][i] = data[offset_m][i] / factor;
        }
    }
}

/// Removes the first entries in the rows below the pivot.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - The column offset.
/// * `offset_m` - The row offset.
fn remove_first_entries<
    K: Copy
        + Default
        + std::cmp::PartialEq
        + From<f32>
        + std::ops::Mul<Output = K>
        + std::ops::Div<Output = K>
        + std::ops::SubAssign,
>(
    data: &mut Vec<Vec<K>>,
    offset_n: usize,
    offset_m: usize,
) {
    for i in offset_m + 1..data.len() {
        if data[i][offset_n] == K::from(0.) {
            continue;
        }
        let factor = data[offset_m][offset_n];
        let row: Vec<K> = data[offset_m]
            .clone()
            .into_iter()
            .map(|x| x * (data[i][offset_n] / factor))
            .collect();
        for j in offset_n..data[0].len() {
            data[i][j] -= row[j];
        }
    }
}

/// Finds the next pivot in the matrix.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - A mutable reference to the column offset.
/// * `offset_m` - A mutable reference to the row offset.
fn find_next_pivot<K: Copy + Default + std::cmp::PartialEq + From<f32>>(
    data: &mut Vec<Vec<K>>,
    offset_n: &mut usize,
    offset_m: &mut usize,
) -> Result<(), ZeroedMatrixError> {
    if let Some(new_offset_n) = data[*offset_m].iter().position(|x| *x == K::from(1.)) {
        *offset_n = new_offset_n;
        Ok(())
    } else {
        if *offset_m == 0 {
            return Err(ZeroedMatrixError);
        }
        *offset_m -= 1;
        find_next_pivot(data, offset_n, offset_m)
    }
}

/// Removes the last entries in the rows above the pivot.
///
/// # Arguments
///
/// * `data` - A mutable reference to the matrix data.
/// * `offset_n` - The column offset.
/// * `offset_m` - The row offset.
fn remove_last_entries<
    K: Copy
        + Default
        + std::cmp::PartialEq
        + From<f32>
        + std::ops::Mul<Output = K>
        + std::ops::SubAssign,
>(
    data: &mut Vec<Vec<K>>,
    offset_n: usize,
    offset_m: usize,
) {
    for i in (0..offset_m).rev() {
        if data[i][offset_n] == K::from(0.) {
            continue;
        }
        let row: Vec<K> = data[offset_m]
            .clone()
            .into_iter()
            .map(|x| x * data[i][offset_n])
            .collect();
        for j in offset_n..data[0].len() {
            data[i][j] -= row[j];
        }
    }
}

/// An error type for when all elements in a matrix column are zeroes.
#[derive(Debug, Clone)]
struct ZeroedColumnError;

impl std::fmt::Display for ZeroedColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "All elements in matrix column are zeroes")
    }
}

/// An error type for when all elements in a matrix are zeroes.
#[derive(Debug, Clone)]
struct ZeroedMatrixError;

impl std::fmt::Display for ZeroedMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "All elements in matrix are zeroes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_echelon() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 0.], [0., 1.]]));
        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 2.], [0., 0.]]));
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 0.625, 0., 0., -12.1666667],
                [0., 0., 1., 0., -3.6666667],
                [0., 0., 0., 1., 29.5]
            ])
        );
    }
}
