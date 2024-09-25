
// Each of the operators in the programming language will implement the Operator trait
pub trait Operator {
    fn apply(&self, operands: &Vec<f32>, result: &mut f32);
}