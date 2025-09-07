use std::fmt::Display;

use num::{Integer, ToPrimitive, traits::NumOps};

use crate::stack::{Stack, stack_error::StackError};

impl<T: Clone + ToPrimitive + NumOps + Display + From<u8> + Integer> Stack<T> {
    pub fn new() -> Self {
        Stack {
            state: Vec::new(),
            idx: 0,
            op: 0,
        }
    }
    pub fn from(slice: &[T]) -> Self {
        Stack {
            state: slice.to_vec(),
            idx: 0,
            op: 0,
        }
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        match self.state.pop() {
            Some(item) => Ok(item),
            None => Err(StackError::EmptyStack {
                idx: self.idx,
                op: self.op,
            }),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), StackError> {
        self.state
            .try_reserve(1)
            .map_err(|e| StackError::ReserveError { source: e })?;
        self.state.push(item);
        Ok(())
    }

    pub fn peek(&self) -> Option<&T> {
        self.state.last()
    }
}

impl<T: Clone + ToPrimitive + NumOps + Display + From<u8> + Integer> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_empty_stack() {
        let stack = Stack::<u8>::new();
        assert_eq!(stack.state, [])
    }
    #[test]
    fn test_new_stack_with_values() {
        let stack = Stack::from(&[1, 5, 78]);
        assert_eq!(stack.state, [1, 5, 78])
    }
    #[test]
    fn test_push() {
        let mut stack = Stack::from(&[1]);
        stack.push(5).unwrap();
        assert_eq!(stack.state, [1, 5])
    }
    #[test]
    fn test_pop() {
        let mut stack = Stack::from(&[1, 5]);
        let popped = stack.pop().unwrap();
        assert_eq!(popped, 5);
        assert_eq!(stack.state, [1])
    }
    #[test]
    fn test_peek() {
        let stack = Stack::from(&[1, 5]);
        let popped = stack.peek().unwrap();
        assert_eq!(popped, &5);
        assert_eq!(stack.state, [1, 5])
    }
}
