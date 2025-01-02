use matrix::{
    ex01::linear_combination, ex02::lerp, ex05::angle_cos, ex06::cross_product, matrix::Matrix,
    vector::Vector,
};

fn main() {
    let mut exercise_map: Vec<(String, fn())> = vec![];
    exercise_map.push(("EX00: Add, Subtract and Scale".to_string(), ex00));
    exercise_map.push(("EX01: Linear combination".to_string(), ex01));
    exercise_map.push(("EX02: Linear interpolation".to_string(), ex02));
    exercise_map.push(("EX03: Dot product".to_string(), ex03));
    exercise_map.push(("EX04: Norm".to_string(), ex04));
    exercise_map.push(("EX05: Cosine".to_string(), ex05));
    exercise_map.push(("EX06: Cross product".to_string(), ex06));
    exercise_map.push(("EX07: Linear map, Matrix multiplication".to_string(), ex07));
    exercise_map.push(("EX08: Trace".to_string(), ex08));
    exercise_map.push(("EX09: Transpose".to_string(), ex09));
    exercise_map.push(("EX10: Row-echelon form".to_string(), ex10));

    for (key, function) in exercise_map.into_iter() {
        println!("\n\n\n##### {:?} #####\n", key);
        function();
    }
}

fn ex00() {
    let u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    let w = u._add(&v);
    println!("{}", w);
    let u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    let w = u._sub(&v);
    println!("{}", w);
    let u = Vector::from([2., 3.]);
    let w = u._scl(2.);
    println!("{}", w);
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    let w = u._add(&v);
    println!("{}", w);
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    let w = u._sub(&v);
    println!("{}", w);
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    let w = u._scl(2.);
    println!("{}", w);
}

fn ex01() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    println!("{}", linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5]));

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination(&[&v1, &v2], &[10., -2.]));
}

fn ex02() {
    println!("{}", lerp(0., 1., 0.));
    println!("{}", lerp(0., 1., 1.));
    println!("{}", lerp(0., 1., 0.5));
    println!("{}", lerp(21., 42., 0.3));
    println!(
        "{}",
        lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );
    println!(
        "{}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
}

fn ex03() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(&v));
}

fn ex04() {
    let u = Vector::from([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    let u = Vector::from([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    let u = Vector::from([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
}

fn ex05() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{}", angle_cos(&u, &v));
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{}", angle_cos(&u, &v));
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("{}", angle_cos(&u, &v));
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{}", angle_cos(&u, &v));
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
}

fn ex06() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
}

fn ex07() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.mul_mat(&v));
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(&v));
    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(&v));
}

fn ex08() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.trace());
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.trace());
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("{}", u.trace());
}

fn ex09() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.transpose());
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.transpose());
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);
    println!("{}", u.transpose());
}

fn ex10() {
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.row_echelon());
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{}", u.row_echelon());
    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("{}", u.row_echelon());
    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("{}", u.row_echelon());
}
