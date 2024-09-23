use crate::operators::operator::Operator;
use crate::constants::DIV_UNDEF;


pub struct Div;

impl Operator for Div {
    fn apply(&self, operands: Vec<f32>, result: &mut f32) {
        if operands[1] == 0.0 {
            *result = DIV_UNDEF;
        } else {
            *result = operands[0] / operands[1];
        }
    }
}