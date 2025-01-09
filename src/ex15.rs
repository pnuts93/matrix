#[cfg(test)]
mod tests {
    use num::Complex;

    use crate::{ex01::linear_combination, ex02::lerp, matrix::Matrix, vector::Vector};

    #[test]
    fn test_vector_add_complex() {
        let u = Vector::from([Complex::from(2.), Complex::from(3.)]);
        let v = Vector::from([Complex::from(5.), Complex::from(7.)]);
        let w = u._add(&v);
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([Complex::from(7.), Complex::from(10.)]));
    }

    #[test]
    fn test_vector_sub_complex() {
        let u = Vector::from([Complex::from(2.), Complex::from(3.)]);
        let v = Vector::from([Complex::from(5.), Complex::from(7.)]);
        let w = u._sub(&v);
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([Complex::from(-3.), Complex::from(-4.)]));
    }

    #[test]
    fn test_vector_scl_complex() {
        let u = Vector::from([Complex::from(2.), Complex::from(3.)]);
        let w = u._scl(Complex::from(2.));
        assert_eq!(w.size(), 2);
        assert_eq!(w, Vector::from([Complex::from(4.), Complex::from(6.)]));
    }

    #[test]
    fn test_matrix_add_complex() {
        let u = Matrix::from([[Complex::from(1.), Complex::from(2.)], [Complex::from(3.), Complex::from(4.)]]);
        let v = Matrix::from([[Complex::from(7.), Complex::from(4.)], [-Complex::from(2.), Complex::from(2.)]]);
        let w = u._add(&v);
        assert_eq!(w, Matrix::from([[Complex::from(8.), Complex::from(6.)], [Complex::from(1.), Complex::from(6.)]]));
    }

    #[test]
    fn test_matrix_sub_complex() {
        let u = Matrix::from([[Complex::from(1.), Complex::from(2.)], [Complex::from(3.), Complex::from(4.)]]);
        let v = Matrix::from([[Complex::from(7.), Complex::from(4.)], [-Complex::from(2.), Complex::from(2.)]]);
        let w = u._sub(&v);
        assert_eq!(w, Matrix::from([[-Complex::from(6.), -Complex::from(2.)], [Complex::from(5.), Complex::from(2.)]]));
    }

    #[test]
    fn test_matrix_scl_complex() {
        let u = Matrix::from([[Complex::from(1.), Complex::from(2.)], [Complex::from(3.), Complex::from(4.)]]);
        let w = u._scl(Complex::from(2.));
        assert_eq!(w, Matrix::from([[Complex::from(2.), Complex::from(4.)], [Complex::from(6.), Complex::from(8.)]]));
    }

    #[test]
    fn test_linear_combination_complex() {
        let e1 = Vector::from([Complex::from(1.), Complex::from(0.), Complex::from(0.)]);
        let e2 = Vector::from([Complex::from(0.), Complex::from(1.), Complex::from(0.)]);
        let e3 = Vector::from([Complex::from(0.), Complex::from(0.), Complex::from(1.)]);
        assert_eq!(
            Vector::from([Complex::from(10.), Complex::from(-2.), Complex::from(0.5)]),
            linear_combination(&[&e1, &e2, &e3], &[Complex::from(10.), Complex::from(-2.), Complex::from(0.5)])
        );

        let v1 = Vector::from([Complex::from(1.), Complex::from(2.), Complex::from(3.)]);
        let v2 = Vector::from([Complex::from(0.), Complex::from(10.), Complex::from(-100.)]);
        assert_eq!(
            Vector::from([Complex::from(10.), Complex::from(0.), Complex::from(230.)]),
            linear_combination(&[&v1, &v2], &[Complex::from(10.), Complex::from(-2.)])
        );
    }

    #[test]
    fn test_lerp_complex() {
        assert_eq!(lerp(Complex::from(0.), Complex::from(1.), 0.), Complex::from(0.));
        assert_eq!(lerp(Complex::from(0.), Complex::from(1.), 1.), Complex::from(1.));
        assert_eq!(lerp(Complex::from(0.), Complex::from(1.), 0.5), Complex::from(0.5));
        assert_eq!(lerp(Complex::from(21.), Complex::from(42.), 0.3), Complex::from(27.3));
        assert_eq!(
            lerp(Vector::from([Complex::from(2.), Complex::from(1.)]), Vector::from([Complex::from(4.), Complex::from(2.)]), 0.3),
            Vector::from([Complex::from(2.6), Complex::from(1.3)])
        );
        assert_eq!(
            lerp(
                Matrix::from([[Complex::from(2.), Complex::from(1.)], [Complex::from(3.), Complex::from(4.)]]),
                Matrix::from([[Complex::from(20.), Complex::from(10.)], [Complex::from(30.), Complex::from(40.)]]),
                0.5
            ),
            Matrix::from([[Complex::from(11.), Complex::from(5.5)], [Complex::from(16.5), Complex::from(22.)]])
        );
    }
}