use crate::genotype::Genotype;


pub struct Population {
    pub genotypes: Vec<Genotype>,
}

impl Population {
    pub fn new(population_size: usize, min_length: usize, max_length: usize) -> Self {
        let mut genotypes: Vec<Genotype> = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            let genotype = Genotype::new(min_length, max_length);
            genotypes.push(genotype);
        }        

        Population {
            genotypes
        }
    }
}