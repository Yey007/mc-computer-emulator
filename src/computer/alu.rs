use std::borrow::Borrow;
use ux::u4;
use crate::computer::register::Register;

pub struct ArithmeticLogicUnit {
   accumulator: Register<u4>
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
       ArithmeticLogicUnit { accumulator: Register::new() }
    }

    pub fn accumulator(&self) -> &Register<u4> {
       self.accumulator.borrow()
    }
}
