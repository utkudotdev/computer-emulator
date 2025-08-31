use crate::computer::register::Register;
use crate::computer::WORKING_BITS;
use common::un::U;
use std::borrow::BorrowMut;

pub struct ArithmeticLogicUnit {
    accumulator: Register<WORKING_BITS>,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        ArithmeticLogicUnit {
            accumulator: Register::new(),
        }
    }

    pub fn accumulator_mut(&mut self) -> &mut Register<WORKING_BITS> {
        self.accumulator.borrow_mut()
    }

    pub fn add(&mut self, value: U<WORKING_BITS>) {
        let current = self.accumulator.load();
        self.accumulator.store(current + value)
    }

    pub fn sub(&mut self, value: U<WORKING_BITS>) {
        let current = self.accumulator.load();
        self.accumulator.store(current - value)
    }

    pub fn bor(&mut self, value: U<WORKING_BITS>) {
        let current = self.accumulator.load();
        self.accumulator.store(current | value)
    }

    pub fn and(&mut self, value: U<WORKING_BITS>) {
        let current = self.accumulator.load();
        self.accumulator.store(current & value)
    }

    pub fn cmp(&self, value: U<WORKING_BITS>) -> bool {
        let current = self.accumulator.load();
        current == value
    }

    pub fn grt(&self, value: U<WORKING_BITS>) -> bool {
        let current = self.accumulator.load();
        current > value
    }

    pub fn les(&self, value: U<WORKING_BITS>) -> bool {
        let current = self.accumulator.load();
        current < value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    fn initialize_alu() -> ArithmeticLogicUnit {
        let mut alu = ArithmeticLogicUnit::new();
        alu.accumulator.borrow_mut().store(5u8.into());
        alu
    }

    #[test]
    fn add() {
        let mut alu = initialize_alu();
        alu.add(4u8.into());
        assert_eq!(alu.accumulator.borrow().load(), 9u8.into())
    }

    #[test]
    fn sub() {
        let mut alu = initialize_alu();
        alu.sub(4u8.into());
        assert_eq!(alu.accumulator.borrow().load(), 1u8.into())
    }

    #[test]
    fn bor() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.bor(0b011u8.into());
        assert_eq!(alu.accumulator.borrow().load(), 0b111u8.into())
    }

    #[test]
    fn and() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.and(0b011u8.into());
        assert_eq!(alu.accumulator.borrow().load(), 0b001u8.into())
    }

    #[test]
    fn cmp_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(5u8.into());
        assert_eq!(result, true)
    }

    #[test]
    fn cmp_not_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(4u8.into());
        assert_eq!(result, false)
    }

    #[test]
    fn grt_equal() {
        let alu = initialize_alu();
        let result = alu.les(5u8.into());
        assert_eq!(result, false)
    }

    #[test]
    fn grt_less() {
        let alu = initialize_alu();
        let result = alu.les(4u8.into());
        assert_eq!(result, false)
    }

    #[test]
    fn grt_greater() {
        let alu = initialize_alu();
        let result = alu.les(6u8.into());
        assert_eq!(result, true)
    }

    #[test]
    fn les_equal() {
        let alu = initialize_alu();
        let result = alu.les(5u8.into());
        assert_eq!(result, false)
    }

    #[test]
    fn les_less() {
        let alu = initialize_alu();
        let result = alu.les(6u8.into());
        assert_eq!(result, true)
    }

    #[test]
    fn les_greater() {
        let alu = initialize_alu();
        let result = alu.les(4u8.into());
        assert_eq!(result, false)
    }
}
