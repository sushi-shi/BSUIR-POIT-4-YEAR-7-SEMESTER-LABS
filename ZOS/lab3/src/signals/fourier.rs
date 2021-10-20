use num::complex::Complex;
use std::f64::consts::PI;

pub fn discrete_fourier(vec: &Vec<f64>, n_big: usize) -> Vec<Complex<f64>> {
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

pub fn inverse_discrete_fourier(fourier: &[Complex<f64>], n_big: usize) -> Vec<f64> {
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

pub struct Filter;
impl Filter {
    pub fn moving_average(signal: &Vec<f64>, length: usize) -> Vec<f64> {
        let mut res = Vec::new();
        let mut average = vec![0.; length];

        for (i, a) in signal.iter().enumerate() {
            average[i % length] = *a;
            res.push(if i < length {
                signal.iter().take(i + 1).sum::<f64>() / (i + 1) as f64
            } else {
                average.iter().sum::<f64>() / length as f64
            })
        }

        res
    }

    pub fn cut_min(signal: &Vec<Complex<f64>>, min: usize) -> Vec<Complex<f64>> {
        let len = signal.len();
        signal
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if i < len / 2 {
                    if i < min {
                        Complex::new(0., 0.)
                    } else {
                        *c
                    }
                } else {
                    /* symmetrical */
                    if i > len - min {
                        Complex::new(0., 0.)
                    } else {
                        *c
                    }
                }
            })
            .collect()
    }

    pub fn cut_max(signal: &Vec<Complex<f64>>, max: usize) -> Vec<Complex<f64>> {
        let len = signal.len();
        signal
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if i < len / 2 {
                    if i > max {
                        Complex::new(0., 0.)
                    } else {
                        *c
                    }
                } else {
                    /* symmetrical */
                    if i < len - max {
                        Complex::new(0., 0.)
                    } else {
                        *c
                    }
                }
            })
            .collect()
    }
}
