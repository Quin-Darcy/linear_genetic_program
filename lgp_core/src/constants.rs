pub const NUM_OPS: u8 = 4;
pub const NUM_VAR_REG: u8 = 3;
pub const NUM_CON_REG: u8 = 255;

pub const OPCODE_ADD: u8 = 0;
pub const OPCODE_SUB: u8 = 1;
pub const OPCODE_MUL: u8 = 2;
pub const OPCODE_DIV: u8 = 3;

pub const DIV_UNDEF: f32 = f32::MAX;
pub const NUM_INPUTS: usize = 1;
pub const NUM_OUTPUTS: usize = 1;

pub type Instruction = u32;
