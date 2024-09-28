use lgp_core::program::Program;
use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};


pub struct Genotype {
    pub program: Program,
    pub fitness: Option<f32>,
    pub output: Option<[f32; NUM_OUTPUTS]>
}

impl Genotype {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        let prog = Program::new_random(min_length, max_length);

        Genotype {
            program: prog,
            fitness: None,
            output: None
        }
    }

    pub fn express(&mut self, input: &[f32; NUM_INPUTS]) {
        self.output = Some(self.program.run(input));
    }

    pub fn set_fitness(&mut self, expected: &[f32; NUM_OUTPUTS]) {
        self.fitness = Some(expected.iter()
            .zip(self.output.unwrap().iter())
            .map(|(a, b)| (a - b).powf(2.0))
            .sum::<f32>() / (expected.len() as f32));
    }

    // TODO: Add mutation method    pub
}