use rand::Rng;
use crate::operators::operator_set::OperatorSet;
use crate::operators::operator::Operator;
use crate::program::registers::Registers;
use crate::constants::{
    NUM_OPS,
    NUM_VAR_REG, 
    NUM_CON_REG,
    Instruction,
    OPCODE_ADD,
    OPCODE_SUB,
    OPCODE_MUL,
    OPCODE_DIV,
    NUM_INPUTS,
    NUM_OUTPUTS,
};

#[derive(Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub registers: Registers,
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

    fn decode(instruction: u32) -> [u8; 4] {
        let bytes: [u8; 4] = [
            (instruction >> 24) as u8,
            (instruction >> 16 & 0xFF) as u8,
            (instruction >> 8 & 0xFF) as u8,
            (instruction & 0xFF) as u8,
        ];
        bytes
    }    

    fn run_instruction(&mut self, instruction: u32, op_set: &OperatorSet) {
        let mut is_constant: bool = true;
        // Get the first bit which indicates constant or variable
        if instruction & 0x80000000 == 0 {
            // This means the bit is not set and the 3rd byte
            // is to be interpretted as the index of a variable register
            is_constant = false;
        }

        // Get the bytes of the instruction
        let instruction_bytes: [u8; 4] = Program::decode(instruction);
        // Zeroize the first bit to restore the actual opcode index
        let opcode: u8 = instruction_bytes[0] & 0x7F;
        let operator: &dyn Operator = op_set.get_operator(opcode);

        // Collect the operands in a vector
        let index_one: usize = instruction_bytes[1] as usize;
        let index_two: usize = instruction_bytes[2] as usize;
        let index_three: usize = instruction_bytes[3] as usize;

        // Collect the operands in single vector
        let operands: Vec<f32> = if is_constant {
            vec![self.registers.variable_registers[index_two], self.registers.constants[index_three]]
        } else {
            vec![self.registers.variable_registers[index_two], self.registers.variable_registers[index_three]]
        };

        // Apply the instruction by passing operands and mutable ref to 
        operator.apply(&operands, &mut self.registers.variable_registers[index_one]);
    }

    pub fn run(&mut self, input: &[f32; NUM_INPUTS]) -> [f32; NUM_OUTPUTS] {
        if NUM_VAR_REG < NUM_INPUTS as u8 {
            // TODO: Add better error handling
            panic!("Err: Too many inputs.");
        }

        // Initialize the variable registers with the input
        for i in 0..NUM_INPUTS {
            self.registers.variable_registers[i] = input[i];
        }

        // Create OperatorSet instance to dynamically call operators
        let op_set = OperatorSet::new();

        // Step through each instruction in the program
        let num_instructions: usize = self.instructions.len();
        for i in 0..num_instructions {
            self.run_instruction(self.instructions[i], &op_set);
        }

        // Collect the outputs
        let mut outputs = [0.0; NUM_OUTPUTS];
        for i in 0..NUM_OUTPUTS {
            outputs[i] = self.registers.variable_registers[i];
        }
        outputs
    }
}