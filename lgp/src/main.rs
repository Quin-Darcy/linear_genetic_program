use lgp_core::program::Program;
use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};
use ea_core::population::Population;

fn main() {
    let min_length: usize = 8;
    let max_length: usize = 16;
    let population_size: usize = 10;

    let mut pop = Population::new(population_size, min_length, max_length);
}
