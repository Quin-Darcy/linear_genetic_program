use crate::program::operators::Operator;


pub struct Sub;

impl Operator for Sub {
    pub fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        *result = operands[0] - operands[1];
    }
}