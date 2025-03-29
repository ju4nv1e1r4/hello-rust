extern crate rand;
use rand::Rng;

fn main() {
    let random_values1 = rand::rng().random_range(1.5..10.);
    println!("{random_values1}");

    let random_values2 = rand::rng().random_bool(0.3);
    println!("{random_values2}")
}
