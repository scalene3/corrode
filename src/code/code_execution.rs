use std::fmt::{Debug, Display};

use colored::{ColoredString, Colorize};
use num::{Integer, ToPrimitive, traits::NumOps};

use crate::{
    code::parse::parse_code,
    stack::{Stack, stack_error::StackError},
};

impl<T: ToPrimitive + NumOps + Display + TryInto<u8> + From<u8> + Integer + Clone> Stack<T> {
    pub fn execute(&mut self, code: &[u8]) -> Result<T, StackError> {
        while let Some(&op) = code.get(self.idx) {
            self.op = op;
            match op {
                0x00 => {
                    self.idx += 1;
                }
                0x01 => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    self.push(lhs + rhs)?;
                    self.idx += 1;
                }
                0x02 => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    self.push(lhs - rhs)?;
                    self.idx += 1;
                }
                0x03 => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    self.push(lhs * rhs)?;
                    self.idx += 1;
                }
                0x04 => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    self.push(lhs / rhs)?;
                    self.idx += 1;
                }
                0x05 => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    self.push(lhs % rhs)?;
                    self.idx += 1;
                }
                0x10 => {
                    println!("{}", self.peek().unwrap());

                    self.idx += 1;
                }
                0x11 => {
                    let mut string_data: Vec<u8> = Vec::new();

                    while let Ok(character) = self.pop() {
                        if character == 0.into() {
                            break;
                        } else if let Ok(to_push) = character.try_into() {
                            string_data.push(to_push)
                        }
                    }
                    string_data.reverse();
                    if let Ok(out_string) = String::from_utf8(string_data.clone()) {
                        println!("{}", out_string)
                    } else {
                        println!(
                            "{}",
                            ColoredString::from("Could not parse stack to string.").red()
                        )
                    }
                    for c in string_data {
                        self.push(c.into())?;
                    }
                    self.idx += 1
                }
                0x12 => {
                    return self.peek().cloned().ok_or(StackError::EmptyStack {
                        idx: self.idx,
                        op: self.op,
                    });
                }
                0x20 => {
                    self.idx += 1;

                    if let Some(number) = code.get(self.idx) {
                        self.push((*number).into())?;
                        self.idx += 1;
                    }
                }
                0x21 => {
                    self.idx += 1;
                    let first = self.pop()?;
                    let second = self.pop()?;
                    self.push(first)?;
                    self.push(second)?
                }
                0x22 => {
                    self.idx += 1;
                    let _ = self.pop();
                }
                0x23 => {
                    self.idx += 1;
                    let top = self.pop()?;
                    self.push(top.clone())?;
                    self.push(top)?;
                }
                0x30 => self.idx = code[self.idx + 1] as usize,
                0x31 => match self.peek() {
                    Some(top) if top != &0.into() => self.idx = code[self.idx + 1] as usize,
                    _ => self.idx += 2,
                },

                0xFF => return Ok(0xFF.into()),

                _ => {
                    return Err(StackError::UnknownOp {
                        idx: self.idx,
                        byte: self.op,
                    });
                }
            }
        }
        Ok(0.into())
    }
}

///Compile and execute .cor file returning any output to the caller
pub fn run<T>(input_file: &str) -> anyhow::Result<T>
where
    T: Debug + ToPrimitive + NumOps + Display + TryInto<u8> + From<u8> + Integer + Clone,
{
    let mut stack: Stack<T> = Stack::new();
    let code_stack: Stack<u8> = Stack::from(&parse_code(input_file)?);

    let result = stack.execute(&code_stack.state);

    if let Err(error) = result.clone() {
        let mut error_location = None;
        if let StackError::UnknownOp { idx, .. } = error {
            error_location = Some(idx);
        } else if let StackError::EmptyStack { idx, .. } = error {
            error_location = Some(idx);
        }
        eprintln!("{}", format!("{:?}", result).red());
        println!("Call Stack");

        code_stack.trace(error_location);
        println!("current execution stack state");
        println!("{:?}", stack.state);
    }
    Ok(result?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x20, 0x06, 0x01, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 11);

        assert_eq!(stack.state, [11]);
    }
    #[test]
    fn sub() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x20, 0x06, 0x02, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, -1);
        assert_eq!(stack.state, [-1]);
    }
    #[test]
    fn mul() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x20, 0x06, 0x03, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 30);
        assert_eq!(stack.state, [30]);
    }
    #[test]
    fn div() {
        let code: Vec<u8> = vec![0x20, 0x06, 0x20, 0x03, 0x04, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 2);
        assert_eq!(stack.state, [2]);
    }
    #[test]
    fn modulus() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x20, 0x03, 0x05, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 2);
        assert_eq!(stack.state, [2]);
    }
    #[test]
    fn jmp() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x30, 0x06, 0x20, 0x03, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 5);
        assert_eq!(stack.state, [5]);
    }
    #[test]
    fn jnz() {
        let code: Vec<u8> = vec![0x20, 0x05, 0x20, 0x01, 0x02, 0x31, 0x02, 0x20, 0x03, 0x12];
        let mut stack = Stack::<i64>::new();
        let retval = stack.execute(&code).unwrap();
        assert_eq!(retval, 3);
        assert_eq!(stack.state, [0, 3]);
    }
}
