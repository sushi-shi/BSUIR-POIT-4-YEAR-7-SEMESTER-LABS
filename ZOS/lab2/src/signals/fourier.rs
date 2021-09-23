
use std::f64::consts::PI;
use num::complex::Complex;

// Integral is intinsically high to understand



pub fn discrete_fourier(n_big: u64, f: Box<dyn Fn(u64) -> f64>) -> Vec<Complex<f64>> {
    (0..=n_big).map(discrete_fourier_k(n_big, f)).map(|x| x.unscale(n_big as f64)).collect()
}

fn discrete_fourier_k(n_big: u64, f: Box<dyn Fn(u64) -> f64>) -> Box<dyn Fn(u64) -> Complex<f64>> {
    Box::new(move |k| {
        let k = k as f64;
        (0..n_big).map(|n| {
            let alpha = 2.0 * PI * (n as f64) * k / n_big as f64;
            Complex::new(f64::cos(alpha), -f64::sin(alpha)).scale(f(n))
        }).sum::<Complex<f64>>()
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::*;

    const N: u64 = 10;
    const EPS: f64 = 0.000001;

    fn cos3(phase: f64, n_big: u64) -> Box<dyn Fn(u64) -> f64> {
        Box::new(move |n| {
            let n = n as f64;
            let n_big = n_big as f64;
            f64::cos(2.0 * PI * 3.0 * n / n_big + phase)
        })
    }
    #[test]
    fn test() {
        let cos = cos3(0.0, N);
        let fouriers = discrete_fourier(N, cos);
        assert!(approx_eq!(f64, fouriers[3].re, 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[3].norm(), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[2].norm(), 0.0, epsilon = EPS));

        let cos = cos3(PI / 4.0, N);
        let fouriers = discrete_fourier(N, cos);
        assert!(approx_eq!(f64, fouriers[3].re, 0.353553, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[3].norm(), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[2].norm(), 0.0, epsilon = EPS));

        let cos = cos3(PI / 2.0, N);
        let fouriers = discrete_fourier(N, cos);
        assert!(approx_eq!(f64, fouriers[3].im, 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[3].norm(), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, fouriers[2].norm(), 0.0, epsilon = EPS));
    }
}
