use crate::operators::operator::Operator;


pub struct Sub;

impl Operator for Sub {
    fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        *result = operands[0] - operands[1];
    }
}