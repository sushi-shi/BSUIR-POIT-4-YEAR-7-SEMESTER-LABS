use gtk::{
    Box,
};
use gtk::prelude::*;
use crate::widgets::input::Input;
use crate::seq::Random;
use crate::picture::GraphInfo;

const DEFAULT_WIDTH: i32 = 6;
const DEFAULT_LENGTH: usize = 2_000_000;

pub fn set_uniform(anchor: &Box) {
    anchor.append(&Input::new("a", DEFAULT_WIDTH));
    anchor.append(&Input::new("b", DEFAULT_WIDTH));
}

pub fn set_gaussian(anchor: &Box) {
    anchor.append(&Input::new("N", DEFAULT_WIDTH));
    anchor.append(&Input::new("mean", DEFAULT_WIDTH));
    anchor.append(&Input::new("devi", DEFAULT_WIDTH));
}

pub fn set_exponential(anchor: &Box) {
    anchor.append(&Input::new("lambda", DEFAULT_WIDTH));
}

pub fn set_gamma(anchor: &Box) {
    anchor.append(&Input::new("eta", DEFAULT_WIDTH));
    anchor.append(&Input::new("lambda", DEFAULT_WIDTH));
}

pub fn set_triangle(anchor: &Box) {
    anchor.append(&Input::new("a", DEFAULT_WIDTH));
    anchor.append(&Input::new("b", DEFAULT_WIDTH));
}

pub fn set_simpson(anchor: &Box) {
    anchor.append(&Input::new("a", DEFAULT_WIDTH));
    anchor.append(&Input::new("b", DEFAULT_WIDTH));
}

pub fn calc_uniform(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let a_input = anchor.first_child()?.downcast::<Input>().ok()?;
    let b_input = a_input.next_sibling()?.downcast::<Input>().ok()?; 
    
    let a = a_input.text().parse::<f64>().ok()?;
    let b = b_input.text().parse::<f64>().ok()?;

    if a >= b { None? }

    Some(calc_stats(calc_uniform_pure(a, b)))
}

pub fn calc_uniform_pure(a: f64, b: f64) -> Vec<f64> {
    let mut vec = Vec::new();
    Random::default()
        .take(DEFAULT_LENGTH)
        .for_each(|i| vec.push((a + (b - a) * i) as f64));

    vec
}

pub fn calc_gaussian(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let n_input = anchor.first_child()?.downcast::<Input>().ok()?;
    let mean_input = n_input.next_sibling()?.downcast::<Input>().ok()?; 
    let devi_input = mean_input.next_sibling()?.downcast::<Input>().ok()?; 
    
    let n = n_input.text().parse::<usize>().ok()?;
    let mean = mean_input.text().parse::<f64>().ok()?;
    let devi = devi_input.text().parse::<f64>().ok()?;

    if !(6..=12).contains(&n) { None? }

    let mut vec = Vec::new();
    Random::default()
        .take(DEFAULT_LENGTH)
        .collect::<Vec<f64>>()
        .chunks_exact(n)
        .for_each(|chunk| {
            let sum = chunk.iter().sum::<f64>() - n as f64 / 2.0;
            let x = mean + devi * (12.0/ n as f64).powf(0.5) * sum;
            vec.push(x);
        });

    Some(calc_stats(vec))
}

pub fn calc_exponential(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let lambda_input = anchor.first_child()?.downcast::<Input>().ok()?;

    let lambda = lambda_input.text().parse::<f64>().ok()?;

    if lambda == 0.0 { None? }

    let vec = Random::default()
        .take(DEFAULT_LENGTH)
        .map(|v| 0.0 - (1.0 / lambda) * v.ln())
        .collect::<Vec<f64>>();

    Some(calc_stats(vec))
}

pub fn calc_gamma(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let eta_input = anchor.first_child()?.downcast::<Input>().ok()?;
    let lambda_input = eta_input.next_sibling()?.downcast::<Input>().ok()?; 

    let eta = eta_input.text().parse::<usize>().ok()?;
    let lambda = lambda_input.text().parse::<f64>().ok()?;

    if lambda <= 0.0 { None? }
    if eta == 0 { None? }

    let mut vec = Vec::new();
    Random::default()
        .take(DEFAULT_LENGTH)
        .collect::<Vec<f64>>()
        .chunks_exact(eta)
        .for_each(|chunk| {
            let sum = chunk.iter().map(|x| (*x).ln()).sum::<f64>();
            let x = 0.0 - sum / lambda;
            vec.push(x);
        });

    Some(calc_stats(vec))
}

pub fn calc_triangle(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let a_input = anchor.first_child()?.downcast::<Input>().ok()?;
    let b_input = a_input.next_sibling()?.downcast::<Input>().ok()?; 
    
    let a = a_input.text().parse::<f64>().ok()?;
    let b = b_input.text().parse::<f64>().ok()?;

    if a >= b { None? }

    let mut vec = Vec::new();
    let pairs = {
        let (even, odd): (Vec<_>, Vec<_>) = Random::default()
            .take(DEFAULT_LENGTH)
            .zip(0..)
            .partition(|(_, n)| n % 2 == 0);
        even.into_iter().map(|(x, _)| x).zip(odd.into_iter().map(|(x, _)| x))
    };
    pairs.for_each(|(x, y)| vec.push(a + (b - a) * f64::min(x, y)));


    Some(calc_stats(vec))
}

pub fn calc_simpson(anchor: &Box) -> Option<(f64, f64, f64, GraphInfo)> {
    let a_input = anchor.first_child()?.downcast::<Input>().ok()?;
    let b_input = a_input.next_sibling()?.downcast::<Input>().ok()?; 
    
    let a = a_input.text().parse::<f64>().ok()?;
    let b = b_input.text().parse::<f64>().ok()?;

    if a >= b { None? }

    let uniform = calc_uniform_pure(a / 2.0, b / 2.0);
    let vec = uniform.iter().zip(uniform.iter().skip(1)).map(|(x, y)| x + y).collect(); // whoosh-whoosh, optimize it, smarty pants compiler

    Some(calc_stats(vec))
}

fn calc_stats(sequence: Vec<f64>) -> (f64, f64, f64, GraphInfo) {
    let max = sequence.iter().fold(f64::NAN, |a, b| f64::max(a, *b));
    let min = sequence.iter().fold(f64::NAN, |a, b| f64::min(a, *b));
    let delta = max - min;

    let mean: f64 = sequence.iter().sum::<f64>() / sequence.len() as f64;

    let dispersion = sequence
            .iter()
            .map(|x| (x - mean).powf(2.0))
            .sum::<f64>() 
            / sequence.len() as f64;
    let deviation = dispersion.powf(0.5);

    let cols: Vec<f64> = {
        let mut cols: Vec<usize> = vec![0; crate::seq::PARTS + 1];
        sequence.iter().for_each(|x| {
            let x = x - min;
            let i = (x * crate::seq::PARTS as f64 / delta) as usize;
            cols[i] += 1;
        });
        cols[crate::seq::PARTS - 1] += cols[crate::seq::PARTS];
        let _ = cols.pop();
        cols.into_iter().map(|v| v as f64 / sequence.len() as f64).collect()
    };
    let max_y = cols.iter().copied().fold(f64::NAN, f64::max);

    let ginfo = GraphInfo {
        max_y,
        cols,
        min_x: min,
        max_x: max,
    };

    (mean, dispersion, deviation, ginfo)
}
