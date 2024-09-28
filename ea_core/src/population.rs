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


    pub fn evaluate(&mut self) -> Vec<f32> {
        // Run the training data through each genotype
        for i in 0..self.population_size {
            self.genotypes[i].compute_total_fitness(&self.training_set);
        }

        // Collect the resulting fitnesses and return them
        let mut fitnesses: Vec<f32> = Vec::with_capacity(self.population_size);
        for i in 0..self.population_size {
            fitnesses.push(self.genotypes[i].fitness.unwrap());
        }
        fitnesses
    }

    // TODO: The fitness value should be the averaged fitness across all training cases
}