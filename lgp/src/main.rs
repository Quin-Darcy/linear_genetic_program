use lgp_core::program::Program;

fn main() {
    let min_length: usize = 8;
    let max_length: usize = 16;
    let mut prog: Program = Program::new_random(min_length, max_length);

    println!("Program: {:?}", prog.instructions);
    println!("Constants: {:?}", prog.registers.constants);
}
