use nalgebra::Complex;

pub fn main() {
    let n = 8;
    let x = calc_samples(n);
    let test = (n as f64).log2().trunc() as usize;
    let y = fft(x, test);

    println!("FFT Results");
    for (k, val) in y.iter().enumerate() {
        println!("a{} = {}", k, val);
    }
}

fn calc_samples(n: usize) -> Vec<Complex<f64>> {
    let delta = 1.0 / n as f64;
    (0..n)
        .map(|i| {
            let x = i as f64 * delta;
            if x < 0.25 {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            }
        })
        .collect()
}

fn fft(x: Vec<Complex<f64>>, k: usize) -> Vec<Complex<f64>> {
    if k > 0 {
        let n = x.len();
        let m = n / 2;
        let angle = std::f64::consts::PI / m as f64;

        let mut d = Vec::new();
        for l in 0..m {
            let theta = angle * l as f64;
            d.push(Complex::new(theta.cos(), theta.sin()));
        }

        let mut u1_to_j_minus_1 = Vec::with_capacity(m);
        let mut uj_to_l = Vec::with_capacity(m);

        for i in 0..m {
            u1_to_j_minus_1.push(x[i] + x[i + m]);
            uj_to_l.push(x[i] - x[i + m]);
        }

        for i in 0..m {
            uj_to_l[i] *= d[i];
        }
        let w = fft(u1_to_j_minus_1, k - 1);
        let v = fft(uj_to_l, k - 1);

        let mut y = vec![Complex::new(0.0, 0.0); n];
        for i in 0..m {
            y[2 * i] = w[i];
            y[2 * i + 1] = v[i];
        }

        y
    } else {
        x
    }
}
