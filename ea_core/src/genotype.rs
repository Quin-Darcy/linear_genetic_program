use lgp_core::program::Program;
use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};


pub struct Genotype {
    pub program: Program,
    pub fitness: Option<f32>
}

impl Genotype {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        let prog = Program::new_random(min_length, max_length);

        Genotype {
            program: prog,
            fitness: None
        }
    }

    pub fn express(&mut self, input: &[f32; NUM_INPUTS]) -> [f32; NUM_OUTPUTS] {
        self.program.run(input)
    }
}