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

    fn express(&mut self, input: &[f32; NUM_INPUTS]) {
        self.output = Some(self.program.run(input));
    }

    fn compute_fitness(&mut self, input: &[f32; NUM_INPUTS], expected: &[f32; NUM_OUTPUTS]) -> f32 {
        // Run the program with the given input allowing the genotype to capture its output
        self.express(input);

        // Use the stored output to compare against the expected value(s) to compute fitness
        let fitness: f32 = expected.iter()
            .zip(self.output.unwrap().iter())
            .map(|(a, b)| (a - b).powf(2.0))
            .sum::<f32>() / (expected.len() as f32);
        
        fitness
    }

    pub fn compute_total_fitness(&mut self, training_set: &Vec<([f32; NUM_INPUTS], [f32; NUM_OUTPUTS])>) {
        // Computes the total fitness over the entire training set
        let mut total_fitness: f32 = 0.0;

        for i in 0..training_set.len() {
            total_fitness += self.compute_fitness(&training_set[i].0, &training_set[i].1);
        }
        self.fitness = Some( 1.0 / (total_fitness / (training_set.len() as f32)));
    }

    // TODO: Add mutation method    pub
}