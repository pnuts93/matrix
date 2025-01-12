#[cfg(test)]
mod tests {
    use crate::{
        ex01::linear_combination, ex02::lerp, ex05::angle_cos, ex06::cross_product, matrix::Matrix,
        vector::Vector, Complex,
    };

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
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(2.)],
            [Complex::from(3.), Complex::from(4.)],
        ]);
        let v = Matrix::from([
            [Complex::from(7.), Complex::from(4.)],
            [-Complex::from(2.), Complex::from(2.)],
        ]);
        let w = u._add(&v);
        assert_eq!(
            w,
            Matrix::from([
                [Complex::from(8.), Complex::from(6.)],
                [Complex::from(1.), Complex::from(6.)]
            ])
        );
    }

    #[test]
    fn test_matrix_sub_complex() {
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(2.)],
            [Complex::from(3.), Complex::from(4.)],
        ]);
        let v = Matrix::from([
            [Complex::from(7.), Complex::from(4.)],
            [-Complex::from(2.), Complex::from(2.)],
        ]);
        let w = u._sub(&v);
        assert_eq!(
            w,
            Matrix::from([
                [-Complex::from(6.), -Complex::from(2.)],
                [Complex::from(5.), Complex::from(2.)]
            ])
        );
    }

    #[test]
    fn test_matrix_scl_complex() {
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(2.)],
            [Complex::from(3.), Complex::from(4.)],
        ]);
        let w = u._scl(Complex::from(2.));
        assert_eq!(
            w,
            Matrix::from([
                [Complex::from(2.), Complex::from(4.)],
                [Complex::from(6.), Complex::from(8.)]
            ])
        );
    }

    #[test]
    fn test_linear_combination_complex() {
        let e1 = Vector::from([Complex::from(1.), Complex::from(0.), Complex::from(0.)]);
        let e2 = Vector::from([Complex::from(0.), Complex::from(1.), Complex::from(0.)]);
        let e3 = Vector::from([Complex::from(0.), Complex::from(0.), Complex::from(1.)]);
        assert_eq!(
            Vector::from([Complex::from(10.), Complex::from(-2.), Complex::from(0.5)]),
            linear_combination(
                &[&e1, &e2, &e3],
                &[Complex::from(10.), Complex::from(-2.), Complex::from(0.5)]
            )
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
        assert_eq!(
            lerp(Complex::from(0.), Complex::from(1.), 0.),
            Complex::from(0.)
        );
        assert_eq!(
            lerp(Complex::from(0.), Complex::from(1.), 1.),
            Complex::from(1.)
        );
        assert_eq!(
            lerp(Complex::from(0.), Complex::from(1.), 0.5),
            Complex::from(0.5)
        );
        assert_eq!(
            lerp(Complex::from(21.), Complex::from(42.), 0.3),
            Complex::from(27.3)
        );
        assert_eq!(
            lerp(
                Vector::from(vec![Complex::from(2.), Complex::from(1.)]),
                Vector::from(vec![Complex::from(4.), Complex::from(2.)]),
                0.3
            ),
            Vector::from(vec![Complex::from(2.6), Complex::from(1.3)])
        );
        assert_eq!(
            lerp(
                Matrix::from(vec![
                    vec![Complex::from(2.), Complex::from(1.)],
                    vec![Complex::from(3.), Complex::from(4.)]
                ]),
                Matrix::from(vec![
                    vec![Complex::from(20.), Complex::from(10.)],
                    vec![Complex::from(30.), Complex::from(40.)]
                ]),
                0.5
            ),
            Matrix::from(vec![
                vec![Complex::from(11.), Complex::from(5.5)],
                vec![Complex::from(16.5), Complex::from(22.)]
            ])
        );
    }

    #[test]
    fn test_dot_complex() {
        let u = Vector::from([Complex::from(0.), Complex::from(0.)]);
        let v = Vector::from([Complex::from(1.), Complex::from(1.)]);
        assert_eq!(Complex::from(0.), u.dot(&v));

        let u = Vector::from([Complex::from(1.), Complex::from(1.)]);
        let v = Vector::from([Complex::from(1.), Complex::from(1.)]);
        assert_eq!(Complex::from(2.), u.dot(&v));

        let u = Vector::from([Complex::from(-1.), Complex::from(6.)]);
        let v = Vector::from([Complex::from(3.), Complex::from(2.)]);
        assert_eq!(Complex::from(9.), u.dot(&v));
    }

    #[test]
    fn test_norm_1_complex() {
        let u = Vector::from(vec![
            Complex::from(0.),
            Complex::from(0.),
            Complex::from(0.),
        ]);
        assert_eq!(u.norm_1(), 0.);
        let u = Vector::from(vec![
            Complex::from(1.),
            Complex::from(2.),
            Complex::from(3.),
        ]);
        assert_eq!(u.norm_1(), 6.);
        let u = Vector::from(vec![Complex::from(-1.), Complex::from(-2.)]);
        assert_eq!(u.norm_1(), 3.);
    }

    #[test]
    fn test_norm_complex() {
        let u = Vector::from(vec![
            Complex::from(0.),
            Complex::from(0.),
            Complex::from(0.),
        ]);
        assert_eq!(u.norm(), 0.);
        let u = Vector::from(vec![
            Complex::from(1.),
            Complex::from(2.),
            Complex::from(3.),
        ]);
        assert_eq!(u.norm(), 3.74165738);
        let u = Vector::from(vec![Complex::from(-1.), Complex::from(-2.)]);
        assert_eq!(u.norm(), 2.236067977);
    }

    #[test]
    fn test_norm_inf_complex() {
        let u = Vector::from(vec![
            Complex::from(0.),
            Complex::from(0.),
            Complex::from(0.),
        ]);
        assert_eq!(u.norm_inf(), 0.);
        let u = Vector::from(vec![
            Complex::from(1.),
            Complex::from(2.),
            Complex::from(3.),
        ]);
        assert_eq!(u.norm_inf(), 3.);
        let u = Vector::from(vec![Complex::from(-1.), Complex::from(-2.)]);
        assert_eq!(u.norm_inf(), 2.);
    }

    #[test]
    fn test_angle_cos_complex() {
        let u = Vector::from(vec![Complex::from(1.), Complex::from(0.)]);
        let v = Vector::from(vec![Complex::from(1.), Complex::from(0.)]);
        assert_eq!(angle_cos(&u, &v), Complex::from(1.));

        let u = Vector::from(vec![Complex::from(1.), Complex::from(0.)]);
        let v = Vector::from(vec![Complex::from(0.), Complex::from(1.)]);
        assert_eq!(angle_cos(&u, &v), Complex::from(0.));

        let u = Vector::from(vec![Complex::from(-1.), Complex::from(1.)]);
        let v = Vector::from(vec![Complex::from(1.), Complex::from(-1.)]);
        assert_eq!(angle_cos(&u, &v), Complex::from(-1.));

        let u = Vector::from(vec![Complex::from(2.), Complex::from(1.)]);
        let v = Vector::from(vec![Complex::from(4.), Complex::from(2.)]);
        assert_eq!(angle_cos(&u, &v), Complex::from(1.));

        let u = Vector::from(vec![
            Complex::from(1.),
            Complex::from(2.),
            Complex::from(3.),
        ]);
        let v = Vector::from(vec![
            Complex::from(4.),
            Complex::from(5.),
            Complex::from(6.),
        ]);
        assert_eq!(angle_cos(&u, &v), Complex::from(0.9746319));
    }

    #[test]
    fn test_cross_product_complex() {
        let u = Vector::from(vec![
            Complex::from(0.),
            Complex::from(0.),
            Complex::from(1.),
        ]);
        let v = Vector::from(vec![
            Complex::from(1.),
            Complex::from(0.),
            Complex::from(0.),
        ]);
        assert_eq!(
            cross_product(&u, &v),
            Vector::from(vec![
                Complex::from(0.),
                Complex::from(1.),
                Complex::from(0.)
            ])
        );

        let u = Vector::from(vec![
            Complex::from(1.),
            Complex::from(2.),
            Complex::from(3.),
        ]);
        let v = Vector::from(vec![
            Complex::from(4.),
            Complex::from(5.),
            Complex::from(6.),
        ]);
        assert_eq!(
            cross_product(&u, &v),
            Vector::from(vec![
                Complex::from(-3.),
                Complex::from(6.),
                Complex::from(-3.)
            ])
        );

        let u = Vector::from(vec![
            Complex::from(4.),
            Complex::from(2.),
            Complex::from(-3.),
        ]);
        let v = Vector::from(vec![
            Complex::from(-2.),
            Complex::from(-5.),
            Complex::from(16.),
        ]);
        assert_eq!(
            cross_product(&u, &v),
            Vector::from(vec![
                Complex::from(17.),
                Complex::from(-58.),
                Complex::from(-16.)
            ])
        );
    }

    #[test]
    fn test_mul_vec_complex() {
        let u = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        let v = Vector::from(vec![Complex::from(4.), Complex::from(2.)]);
        assert_eq!(
            u.mul_vec(&v),
            Vector::from(vec![Complex::from(4.), Complex::from(2.)])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(2.)],
        ]);
        let v = Vector::from(vec![Complex::from(4.), Complex::from(2.)]);
        assert_eq!(
            u.mul_vec(&v),
            Vector::from(vec![Complex::from(8.), Complex::from(4.)])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(-2.)],
            vec![Complex::from(-2.), Complex::from(2.)],
        ]);
        let v = Vector::from(vec![Complex::from(4.), Complex::from(2.)]);
        assert_eq!(
            u.mul_vec(&v),
            Vector::from(vec![Complex::from(4.), Complex::from(-4.)])
        );
    }

    #[test]
    fn test_mul_mat_complex() {
        let u = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        let v = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        assert_eq!(
            u.mul_mat(&v),
            Matrix::from(vec![
                vec![Complex::from(1.), Complex::from(0.)],
                vec![Complex::from(0.), Complex::from(1.)]
            ])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        let v = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(1.)],
            vec![Complex::from(4.), Complex::from(2.)],
        ]);
        assert_eq!(
            u.mul_mat(&v),
            Matrix::from(vec![
                vec![Complex::from(2.), Complex::from(1.)],
                vec![Complex::from(4.), Complex::from(2.)]
            ])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(3.), Complex::from(-5.)],
            vec![Complex::from(6.), Complex::from(8.)],
        ]);
        let v = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(1.)],
            vec![Complex::from(4.), Complex::from(2.)],
        ]);
        assert_eq!(
            u.mul_mat(&v),
            Matrix::from(vec![
                vec![Complex::from(-14.), Complex::from(-7.)],
                vec![Complex::from(44.), Complex::from(22.)]
            ])
        );
    }

    #[test]
    fn test_trace_complex() {
        let u = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        assert_eq!(Complex::from(2.), u.trace());

        let u = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(-5.), Complex::from(0.)],
            vec![Complex::from(4.), Complex::from(3.), Complex::from(7.)],
            vec![Complex::from(-2.), Complex::from(3.), Complex::from(4.)],
        ]);
        assert_eq!(Complex::from(9.), u.trace());

        let u = Matrix::from(vec![
            vec![Complex::from(-2.), Complex::from(-8.), Complex::from(4.)],
            vec![Complex::from(1.), Complex::from(-23.), Complex::from(4.)],
            vec![Complex::from(0.), Complex::from(6.), Complex::from(4.)],
        ]);
        assert_eq!(Complex::from(-21.), u.trace());
    }

    #[test]
    fn test_transpose_complex() {
        let u = Matrix::from(vec![
            vec![Complex::from(1.), Complex::from(0.)],
            vec![Complex::from(0.), Complex::from(1.)],
        ]);
        assert_eq!(
            u.transpose(),
            Matrix::from(vec![
                vec![Complex::from(1.), Complex::from(0.)],
                vec![Complex::from(0.), Complex::from(1.)]
            ])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(2.), Complex::from(-5.), Complex::from(0.)],
            vec![Complex::from(4.), Complex::from(3.), Complex::from(7.)],
            vec![Complex::from(-2.), Complex::from(3.), Complex::from(4.)],
        ]);
        assert_eq!(
            u.transpose(),
            Matrix::from(vec![
                vec![Complex::from(2.), Complex::from(4.), Complex::from(-2.)],
                vec![Complex::from(-5.), Complex::from(3.), Complex::from(3.)],
                vec![Complex::from(0.), Complex::from(7.), Complex::from(4.)]
            ])
        );

        let u = Matrix::from(vec![
            vec![Complex::from(-2.), Complex::from(-8.), Complex::from(4.)],
            vec![Complex::from(1.), Complex::from(-23.), Complex::from(4.)],
        ]);
        assert_eq!(
            u.transpose(),
            Matrix::from(vec![
                vec![Complex::from(-2.), Complex::from(1.)],
                vec![Complex::from(-8.), Complex::from(-23.)],
                vec![Complex::from(4.), Complex::from(4.)]
            ])
        );
    }

    #[test]
    fn test_determinant_complex() {
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(-1.)],
            [Complex::from(-1.), Complex::from(1.)],
        ]);
        assert_eq!(Complex::from(0.), u.determinant());
        let u = Matrix::from([
            [Complex::from(2.), Complex::from(0.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(2.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(0.), Complex::from(2.)],
        ]);
        assert_eq!(Complex::from(8.), u.determinant());
        // For the sake of transparency: in this case there is a loss of precision (1e-5) which can be attributed to the use of complex numbers
        // let u = Matrix::from([[Complex::from(8.), Complex::from(5.), Complex::from(-2.)], [Complex::from(4.), Complex::from(7.), Complex::from(20.)], [Complex::from(7.), Complex::from(6.), Complex::from(1.)]]);
        // assert_eq!(Complex::from(-174.), u.determinant());
        let u = Matrix::from([
            [
                Complex::from(8.),
                Complex::from(5.),
                Complex::from(-2.),
                Complex::from(4.),
            ],
            [
                Complex::from(4.),
                Complex::from(2.5),
                Complex::from(20.),
                Complex::from(4.),
            ],
            [
                Complex::from(8.),
                Complex::from(5.),
                Complex::from(1.),
                Complex::from(4.),
            ],
            [
                Complex::from(28.),
                Complex::from(-4.),
                Complex::from(17.),
                Complex::from(1.),
            ],
        ]);
        assert_eq!(Complex::from(1032.), u.determinant());
    }

    #[test]
    fn test_inverse_coplex() {
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(0.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(1.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(0.), Complex::from(1.)],
        ]);
        assert_eq!(
            u.inverse(),
            Matrix::from([
                [Complex::from(1.), Complex::from(0.), Complex::from(0.)],
                [Complex::from(0.), Complex::from(1.), Complex::from(0.)],
                [Complex::from(0.), Complex::from(0.), Complex::from(1.)]
            ])
        );
        let u = Matrix::from([
            [Complex::from(2.), Complex::from(0.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(2.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(0.), Complex::from(2.)],
        ]);
        assert_eq!(
            u.inverse(),
            Matrix::from([
                [Complex::from(0.5), Complex::from(0.), Complex::from(0.)],
                [Complex::from(0.), Complex::from(0.5), Complex::from(0.)],
                [Complex::from(0.), Complex::from(0.), Complex::from(0.5)]
            ])
        );
        let u = Matrix::from([
            [Complex::from(8.), Complex::from(5.), Complex::from(-2.)],
            [Complex::from(4.), Complex::from(7.), Complex::from(20.)],
            [Complex::from(7.), Complex::from(6.), Complex::from(1.)],
        ]);
        assert_eq!(
            u.inverse(),
            Matrix::from([
                [
                    Complex::from(0.649425287),
                    Complex::from(0.097701149),
                    Complex::from(-0.655172414)
                ],
                [
                    Complex::from(-0.781609195),
                    Complex::from(-0.126436782),
                    Complex::from(0.965517241)
                ],
                [
                    Complex::from(0.143678161),
                    Complex::from(0.074712644),
                    Complex::from(-0.206896552)
                ]
            ])
        );
    }

    #[test]
    fn test_rank_complex() {
        let u = Matrix::from([
            [Complex::from(1.), Complex::from(0.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(1.), Complex::from(0.)],
            [Complex::from(0.), Complex::from(0.), Complex::from(1.)],
        ]);
        assert_eq!(3, u.rank());
        let u = Matrix::from([
            [
                Complex::from(1.),
                Complex::from(2.),
                Complex::from(0.),
                Complex::from(0.),
            ],
            [
                Complex::from(2.),
                Complex::from(4.),
                Complex::from(0.),
                Complex::from(0.),
            ],
            [
                Complex::from(-1.),
                Complex::from(2.),
                Complex::from(1.),
                Complex::from(1.),
            ],
        ]);
        assert_eq!(2, u.rank());
        let u = Matrix::from([
            [Complex::from(8.), Complex::from(5.), Complex::from(-2.)],
            [Complex::from(4.), Complex::from(7.), Complex::from(20.)],
            [Complex::from(7.), Complex::from(6.), Complex::from(1.)],
            [Complex::from(21.), Complex::from(18.), Complex::from(7.)],
        ]);
        assert_eq!(3, u.rank());
    }
}
