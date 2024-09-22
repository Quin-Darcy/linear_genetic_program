use crate::program::operators::Operator;


pub struct Add;

impl Operator for Add {
    pub fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        *result = operands[0] + operands[1];
    }
}