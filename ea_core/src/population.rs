use rand::thread_rng;
use rand::Rng;

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

    pub fn tournament_selection(&mut self, tournament_size: usize) {
        let mut rng = thread_rng();
        let mut new_genotypes: Vec<Genotype> = Vec::with_capacity(self.population_size);

        // Repeat tournament selection for each new genotype
        for _ in 0..self.population_size {
            // Step 1: Create a vector of indices from 0 to population_size - 1
            let mut indices: Vec<usize> = (0..self.population_size).collect();

            // Step 2: Create an empty vector to hold the randomly selected indices
            let mut selected_indices: Vec<usize> = Vec::with_capacity(tournament_size);

            // Step 3: Randomly select `tournament_size` indices, removing them from the original vector
            for _ in 0..tournament_size {
                let random_index = rng.gen_range(0..indices.len());
                selected_indices.push(indices.remove(random_index));
            }

            // Step 4: Use the selected indices to get the competitors from the population
            let competitors: Vec<&Genotype> = selected_indices.iter().map(|&i| &self.genotypes[i]).collect();

            // Step 5: Find the genotype with the highest fitness among the competitors
            let best_genotype = competitors
                .into_iter()
                .max_by(|a, b| {
                    let fitness_a = a.fitness.unwrap_or(f32::MIN);
                    let fitness_b = b.fitness.unwrap_or(f32::MIN);
                    fitness_a.partial_cmp(&fitness_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();

            // Step 6: Clone and add the best genotype to the new population
            new_genotypes.push(best_genotype.clone());
        }

        // Replace the old population with the new one
        self.genotypes = new_genotypes;
    }
}