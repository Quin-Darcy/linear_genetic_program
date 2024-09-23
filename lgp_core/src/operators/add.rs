use crate::operators::operator::Operator;


pub struct Add;

impl Operator for Add {
    fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        *result = operands[0] + operands[1];
    }
}