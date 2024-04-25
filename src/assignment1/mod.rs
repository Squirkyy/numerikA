use std::f64::consts::PI;

use na::{DMatrix, DVector};

extern crate nalgebra as na;

pub fn main() {
    // Stützstellen
    let support_points = vec![-PI, -PI / 4.0, 0.0, PI / 4.0, PI];
    // Funktionswerte an den Stützstellen
    let function_values: Vec<f64> = support_points.iter().map(|&x| x.tan()).collect();

    //Anzahl der Stützstellen
    let n = support_points.len();
    // Vandemoond Matrix erstellen
    let mut matrix = DMatrix::<f64>::zeros(n, n);

    for i in 0..n {
        let x = support_points[i];
        let mut x_pow = 1.0;

        for j in 0..n {
            matrix[(i, j)] = x_pow;
            x_pow *= x;
        }
    }

    // Lösungsvektor
    let b = DVector::from_vec(function_values);

    //Lösung des Gleichungssystems mit LR-Zerlegung
    if let Some(solution) = matrix.clone().lu().solve(&b) {
        // String building stuff
        let mut terms = Vec::new();
        let mut zero_coeffs = Vec::new();

        for (index, &coeff) in solution.iter().enumerate() {
            let coeff_name = match index {
                0 => "constant".to_string(),
                1 => "linear".to_string(),
                2 => "quadratic".to_string(),
                3 => "cubic".to_string(),
                _ => format!("{}-th degree", index),
            };

            if coeff.abs() < 1e-10 {
                // checking if coefficient is necessarily close enough to zero
                zero_coeffs.push(coeff_name);
                continue;
            }

            // Term creation with sign
            let sign = if coeff >= 0.0 { "+" } else { "-" };
            let term = if index == 0 {
                format!("{} {}", sign, coeff.abs())
            } else {
                format!(
                    "{} {}x{}",
                    sign,
                    coeff.abs(),
                    if index == 1 {
                        "".to_string()
                    } else {
                        format!("^{}", index)
                    }
                )
            };
            terms.push(term);
        }

        // Reverse for highest coefficient first
        terms.reverse();

        let poly_str = terms.join(" ");
        let formatted_poly_str = if poly_str.starts_with("+ ") {
            poly_str[2..].to_string()
        } else {
            poly_str
        };

        // Print polynomial
        println!("The interpolating polynomial is: {}", formatted_poly_str);
        // Output zero coefficients, if there are any
        if !zero_coeffs.is_empty() {
            println!("Zero coefficients: {}", zero_coeffs.join(", "));
        }
    } else {
        println!("No solution exists");
    }
}
