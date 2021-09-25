
use std::f64::consts::PI;
use num::complex::Complex;

pub fn discrete_fourier(n_big: u64, vec: &Vec<f64>) -> Vec<Complex<f64>> {
    (0..=n_big / 2)
        .map(|k| discrete_fourier_k(n_big, vec, k))
        .map(|x| x.unscale(n_big as f64 / 2.))
        .collect()
}

fn discrete_fourier_k(n_big: u64, vec: &Vec<f64>, k: u64) -> Complex<f64> {
    let k = k as f64;
    (0..n_big).map(|n| {
        let alpha = 2.0 * PI * (n as f64) * k / n_big as f64;
        Complex::new(f64::cos(alpha), -f64::sin(alpha)).scale(vec[n as usize])
    }).sum::<Complex<f64>>()
}

