use lgp_core::program::Program;


pub struct Genotype {
    pub program: Program,
    pub fitness: Option<f32>
}

impl Genotype {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        let mut prog = Program::new_random(min_length, max_length);

        Genotype {
            program: prog,
            fitness: None
        }
    }
}