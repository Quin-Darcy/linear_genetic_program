use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};
use ea_core::population::Population;

fn main() {
    let min_length: usize = 16;
    let max_length: usize = 64;
    let population_size: usize = 10;
    let training_set: Vec<([f32; NUM_INPUTS], [f32; NUM_OUTPUTS])> = vec![([1.0], [1.0]), ([2.0], [4.0]), ([3.0], [9.0]), ([4.0], [16.0]), ([5.0], [25.0])];

    let mut pop = Population::new(population_size, min_length, max_length, training_set);

    let fitnesses: Vec<f32> = pop.evaluate(); 

    println!("{:?}", fitnesses);
}
