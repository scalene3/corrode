/*!Module Controlling logic of the principal Stack
 */

use std::fmt::Display;

use num::{Integer, ToPrimitive, traits::NumOps};

pub mod stack_error;
pub mod stack_operations;
pub mod stack_trace;

#[derive(Debug)]
pub struct Stack<T: ToPrimitive + Integer + NumOps + Display + From<u8> + PartialEq + PartialOrd> {
    pub state: Vec<T>,
    pub idx: usize,
    pub op: u8,
}
