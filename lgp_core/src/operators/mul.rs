use crate::operators::operator::Operator;


pub struct Mul;

impl Operator for Mul {
    fn apply(&self, operands: &Vec<f32>, result: &mut f32) {
        *result = operands[0] * operands[1];
    }
}