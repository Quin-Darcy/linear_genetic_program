use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};
use crate::genotype::Genotype;


pub struct Population {
    pub genotypes: Vec<Genotype>,
    pub population_size: usize,
    pub training_set: Vec<([f32; NUM_INPUTS], [f32; NUM_OUTPUTS])>
}

impl Population {
    pub fn new(
        population_size: usize, 
        min_length: usize, 
        max_length: usize,
        training_set: Vec<([f32; NUM_INPUTS], [f32; NUM_OUTPUTS])>
    ) -> Self {
        let mut genotypes: Vec<Genotype> = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            let genotype = Genotype::new(min_length, max_length);
            genotypes.push(genotype);
        }        

        Population {
            genotypes,
            population_size,
            training_set
        }
    }

    fn evaluate(&mut self) {
        for i in 0..self.population_size {
            for j in 0..self.training_set.len() {
                // This runs the input through each program and allows each to set the output
                self.genotypes[i].express(&mut self.training_set[j].0);
                // Pass in the expected result to be compared against each program's output
                self.genotypes[i].set_fitness(&self.training_set[j].1);
            }
        }
    }

    pub fn get_fitnesses(&mut self) -> Vec<f32> {
        self.evaluate();
        let mut fitnesses: Vec<f32> = Vec::with_capacity(self.population_size);
        for i in 0..self.population_size {
            fitnesses.push(self.genotypes[i].fitness.unwrap());
        }
        fitnesses
    }

    // TODO: The fitness value should be the averaged fitness across all training cases
}