use crate::constants::{NUM_VAR_REG, NUM_CON_REG};

pub struct Registers {
    pub variable_registers: [f32; NUM_VAR_REG as usize],
    pub constants: [f32; NUM_CON_REG as usize]
}

impl Registers {
    pub fn new() -> Self {
        // Initialize variable registers to all 0.0
        let variable_registers: [f32; NUM_VAR_REG as usize] = [0.0; NUM_VAR_REG as usize];

        // Initialize constants with a range from -NUM_CON_REG/2 to NUM_CON_REG/2
        let mut constants: [f32; NUM_CON_REG as usize] = [0.0; NUM_CON_REG as usize];

        let starting_point: f32 = -(NUM_CON_REG as f32) / 2.0;
        for i in 0..NUM_CON_REG {
            constants[i as usize] = starting_point + i as f32;
        }

        Registers {
            variable_registers,
            constants,
        }
    }
}