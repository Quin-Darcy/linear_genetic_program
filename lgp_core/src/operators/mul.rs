use crate::program::operators::Operator;


pub struct Mul;

impl Operator for Mul {
    pub fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        *result = operands[0] * operands[1];
    }
}