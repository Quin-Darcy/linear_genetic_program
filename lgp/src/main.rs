use lgp_core::program::Program;
use lgp_core::constants::{NUM_INPUTS, NUM_OUTPUTS};

fn main() {
    let min_length: usize = 8;
    let max_length: usize = 16;
    let mut prog: Program = Program::new_random(min_length, max_length);

    println!("Program: {:?}", prog.instructions);

    let inputs: [f32; NUM_INPUTS] = [1.0];
    
    let outputs: [f32; NUM_OUTPUTS] = prog.run(&inputs);

    println!("OUTPUTS: {:?}", outputs);
}
