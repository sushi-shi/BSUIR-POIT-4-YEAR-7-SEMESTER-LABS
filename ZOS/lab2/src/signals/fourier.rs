use num::complex::Complex;
use std::f64::consts::PI;

pub fn discrete_fourier(n_big: u64, vec: &Vec<f64>) -> Vec<Complex<f64>> {
    let mut fourier = Vec::new();
    for k in 0..n_big {
        let k = k as f64;
        let mut f = Complex::new(0., 0.);
        for n in 0..n_big {
            let alpha = 2.0 * PI * (n as f64) * k / n_big as f64;
            f += Complex::new(f64::cos(alpha), f64::sin(alpha)).scale(vec[n as usize]);
        }
        fourier.push(f.unscale(n_big as f64 / 2.));
    }
    fourier
}

pub fn inverse_discrete_fourier(n_big: usize, fourier: &[Complex<f64>]) -> Vec<f64> {
    let mut res = vec![0.; n_big];
    for n in 0..n_big {
        for k in 0..n_big {
            let theta = (2. * PI * k as f64 * n as f64) / n_big as f64;
            res[n] += fourier[k].re * f64::cos(theta) + fourier[k].im * f64::sin(theta)
        }
        res[n] /= 2 as f64;
    }
    res
}
