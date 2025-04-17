use rayon::prelude::*;
use std::time::Instant;

pub fn sequential() {
    let numbers = vec![1, 2, 3, 4, 5];
    // to measure time of sequantial sum
    let start_seq = Instant::now();
    let sum_seq: u32 = numbers.iter().sum();
    let end_seq = start_seq.elapsed();
    println!("Sequential sum: {} in {:?}", sum_seq, end_seq);
}

pub fn parallel() {
    let numbers = vec![1, 2, 3, 4, 5];
    // to measure time of parallel sum
    let start_par = Instant::now();
    let sum_par: u32 = numbers.par_iter().sum();
    let end_par = start_par.elapsed();
    println!("Parallel sum: {} in {:?}", sum_par, end_par);
}

