use crate::matrix::Matrix;

/// Creates a perspective projection matrix.
///
/// # Arguments
///
/// * `fov` - The field of view in radians.
/// * `ratio` - The aspect ratio of the screen.
/// * `near` - The near clipping plane.
/// * `far` - The far clipping plane.
///
/// # Returns
///
/// A new matrix representing the perspective projection.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let mut data = vec![vec![0.; 4]; 4];
    let tan_half_fov = (fov / 2.0).tan();
    data[0][0] = 1.0 / (tan_half_fov * ratio);
    data[1][1] = 1.0 / tan_half_fov;
    data[2][2] = -(far + near) / (far - near);
    data[2][3] = -(2.0 * far * near) / (far - near);
    data[3][2] = -1.0;

    Matrix::from(data)
}
