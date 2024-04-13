use std::f64::consts::PI;

use na::{DMatrix, DVector};

extern crate nalgebra as na;

pub fn main() {
    let support_points = vec![-PI, -PI / 4.0, 0.0, PI / 4.0, PI];
    let function_values: Vec<f64> = support_points.iter().map(|&x| x.cos()).collect();

    let n = support_points.len();

    let mut matrix = DMatrix::<f64>::zeros(n, n);

    for i in 0..n {
        let x = support_points[i];
        let mut x_pow = 1.0;

        for j in 0..n {
            matrix[(i, j)] = x_pow;
            x_pow *= x;
        }
    }

    let b = DVector::from_vec(function_values);

    if let Some(solution) = matrix.clone().lu().solve(&b) {
        println!("The solution is: {:?}", solution);
    } else {
        println!("No solution exists");
    }
}
