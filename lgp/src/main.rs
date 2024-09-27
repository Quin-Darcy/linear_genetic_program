use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};
use ea_core::population::Population;

fn main() {
    let min_length: usize = 8;
    let max_length: usize = 16;
    let population_size: usize = 10;
    let training_set: Vec<([f32; NUM_INPUTS], [f32; NUM_OUTPUTS])> = vec![([1.0], [1.0])];

    let mut pop = Population::new(population_size, min_length, max_length, training_set);
    let results: Vec<[f32; NUM_OUTPUTS]> = pop.evaluate();

    println!("{:?}", results);
}
