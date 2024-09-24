use crate::operators::{
    add::Add, 
    sub::Sub, 
    mul::Mul, 
    div::Div, 
    operator::Operator
};


pub struct OperatorSet {
    // Note for me: This is essentially saying "Here is a vector of equal sized pointers
    // (the elements of a Vec must all be the same size) which will be pointing to items
    // that implement the Operator trait, but what exactly they are pointing to will 
    // only be known at runtime.
    operators: Vec<Box<dyn Operator>>
}

impl OperatorSet {
    pub fn new() -> Self {
        let operators: Vec<Box<dyn Operator>> = vec![
            Box::new(Add), // Opcode 0 corresponds to Add
            Box::new(Sub), // Opcode 1 corresponds to Sub
            Box::new(Mul), // Opcode 2 corresponds to Mul
            Box::new(Div), // Opcode 3 corresponds to Div
        ];
        OperatorSet { operators }
    }

    pub fn get_operator(&self, opcode: u8) -> &dyn Operator {
        &*self.operators[opcode as usize] // Retrieve the operator for the given opcode
    }
}