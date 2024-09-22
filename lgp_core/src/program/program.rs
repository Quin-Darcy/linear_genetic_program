use rand::Rng;
use crate::program::registers::Registers;
use crate::constants::{
    NUM_OPS,
    NUM_VAR_REG, 
    NUM_CON_REG,
    Instruction,
    OPCODE_ADD,
    OPCODE_SUB,
    OPCODE_MUL,
    OPCODE_DIV
};

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub registers: Registers
}

impl Program {
    pub fn new_random(min_length: usize, max_length: usize) -> Self {
        if max_length < min_length {
            panic!("Err: max_len < min_length");
        }

        // Generate a random length
        let mut rng = rand::thread_rng();
        let num_instructions: usize = rng.gen_range(min_length..=max_length) as usize;

        // Generate the instructions
        let mut instructions: Vec<Instruction> = vec![0; num_instructions];
        
        for i in 0..num_instructions {
            instructions[i] = Program::generate_random_instruction();
        }

        Program {
            instructions,
            registers: Registers::new()
        }
    }

    fn generate_random_instruction() -> Instruction {
        // Start by generating the opcode
        let mut rng = rand::thread_rng();
        let mut opcode: u8 = rng.gen_range(0..NUM_OPS) as u8;

        // Select the index of the destination register
        let dest_reg_indx: u8 = rng.gen_range(0..NUM_VAR_REG) as u8;

        // Declare the remaining operands
        let operand_one: u8;
        let operand_two: u8;

        match opcode {
            OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV => {
                operand_one = rng.gen_range(0..NUM_VAR_REG);

                // Determine if second operand is constant or variable
                if rng.gen_bool(0.5) { // This branch is the 'constant operand' branch
                    // Set the high bit on the opcode to indicate constant
                    opcode |= 0x80; // 10000000
                    // Set the operand to the selected index
                    operand_two = rng.gen_range(0..NUM_CON_REG);
                } else {
                    operand_two = rng.gen_range(0..NUM_VAR_REG);
                }
            },
            // Any more operation types would go here
            _ => panic!("Invalid opcode: {}", opcode),
        }

        // Encode the instruction as u32 and return
        let instruction_bytes: [u8; 4] = [opcode, dest_reg_indx, operand_one, operand_two];
        Program::encode(instruction_bytes)
    }

    fn encode(instruction_bytes: [u8; 4]) -> Instruction {
        // Combines the bytes which make up an instruction into single u32
        ((instruction_bytes[0] as u32) << 24)
            | ((instruction_bytes[1] as u32) << 16)
            | ((instruction_bytes[2] as u32) << 8)
            | (instruction_bytes[3] as u32)
    }
}