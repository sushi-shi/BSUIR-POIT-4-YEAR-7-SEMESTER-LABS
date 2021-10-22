use ndarray::prelude::*;
use ndarray_linalg::Solve;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut equations = Array2::zeros((6, 6));
    let mut solutions = Array::zeros(6);
    let mu = 0.5;
    let p = 0.4;
    let lam = 0.45;
    equations[[0, 0]] = lam;
    equations[[0, 1]] = -mu;
    equations[[0, 2]] = -mu;

    equations[[1, 1]] = mu + lam;
    equations[[1, 0]] = -lam * p;
    equations[[1, 3]] = -mu;

    equations[[2, 2]] = mu + lam;
    equations[[2, 0]] = -lam * (1. - p);
    equations[[2, 4]] = -mu;
    equations[[2, 5]] = -mu;

    equations[[3, 3]] = mu;
    equations[[3, 1]] = -lam * p;
    equations[[3, 4]] = -lam * p;

    equations[[4, 5]] = lam * p + mu;
    equations[[4, 2]] = -lam * (1. - p);

    for i in 0..=5 {
        equations[[5, i]] = 1.;
    }
    solutions[5] = 1.;

    let chances = equations.solve_into(solutions).unwrap();
    for (i, c) in chances.into_iter().enumerate() {
        println!("P{} = {}", i, c);
    }
    Ok(())
}
