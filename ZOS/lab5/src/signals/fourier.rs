pub fn correlate(signal_f: &[f64], signal_g: &[f64]) -> Vec<f64> {
    let mut res = Vec::new();
    for tau in 0..signal_f.len() {
        let mut rho = 0.;
        for t in 0..signal_g.len() {
            rho += signal_g[t]
                * if t + tau < signal_f.len() {
                    signal_f[t + tau]
                } else {
                    0.
                };
        }
        res.push(rho);
    }
    res
}
