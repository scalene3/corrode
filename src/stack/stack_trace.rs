use std::fmt::Display;

use colored::{ColoredString, Colorize};
use num::{Integer, ToPrimitive, traits::NumOps};

use crate::stack::Stack;

impl<T: ToPrimitive + NumOps + Display + From<u8> + Integer> Stack<T> {
    pub fn trace(&self, location: Option<usize>) {
        let mut stack = self.state.iter().enumerate();
        println!(" idx\u{2502}BCode \u{2500}\u{2500}\u{2500}\u{2500}\u{2500} Command");
        while let Some((idx, element)) = stack.next() {
            if let Some(item) = element.to_u8() {
                let trace_string: ColoredString = match item {
                    0x00 => {
                        format!("{idx:>4}\u{2502}(0x00) \u{2500}\u{2500}\u{2500}  Nop     ").into()
                    }
                    0x01 => {
                        format!("{idx:>4}\u{2502}(0x01) \u{2500}\u{2500}\u{2500}  Add     ").into()
                    }
                    0x02 => {
                        format!("{idx:>4}\u{2502}(0x02) \u{2500}\u{2500}\u{2500}  Sub     ").into()
                    }
                    0x03 => {
                        format!("{idx:>4}\u{2502}(0x03) \u{2500}\u{2500}\u{2500}  Mul     ").into()
                    }
                    0x04 => {
                        format!("{idx:>4}\u{2502}(0x04) \u{2500}\u{2500}\u{2500}  Div     ").into()
                    }
                    0x05 => {
                        format!("{idx:>4}\u{2502}(0x05) \u{2500}\u{2500}\u{2500}  Mod     ").into()
                    }
                    0x10 => {
                        format!("{idx:>4}\u{2502}(0x10) \u{2500}\u{2500}\u{2500}  Print   ").into()
                    }
                    0x11 => {
                        format!("{idx:>4}\u{2502}(0x11) \u{2500}\u{2500}\u{2500}  PChar   ").into()
                    }
                    0x12 => {
                        format!("{idx:>4}\u{2502}(0x12) \u{2500}\u{2500}\u{2500}  Rer     ").into()
                    }
                    0x20 => {
                        let val: u8;
                        let first_idx = idx;

                        if let Some((idx, number)) = stack.next() {
                            println!(
                                "{first_idx:>4}\u{2502}(0x20) \u{2500}\u{252C}\u{2500}  ValueU8  "
                            );
                            val = number.to_u8().unwrap();
                            format!("{idx:>4}\u{2502}({:#04x})  \u{2514}\u{2500}  {}", val, val)
                                .into()
                        } else {
                            format!("Expected byte at address {:#04x}", idx + 1).into()
                        }
                    }
                    0x30 => {
                        let val: u8;
                        let first_idx = idx;

                        if let Some((idx, number)) = stack.next() {
                            println!(
                                "{first_idx:>4}\u{2502}(0x30) \u{2500}\u{252C}\u{2500}  JMP     "
                            );
                            val = number.to_u8().unwrap();
                            format!("{idx:>4}\u{2502}({:#04x})  \u{2514}\u{2500}  {}", val, val)
                                .into()
                        } else {
                            format!("Expected byte at address {:#04x}", idx + 1).into()
                        }
                    }
                    0x31 => {
                        let val: u8;
                        let first_idx = idx;

                        if let Some((idx, number)) = stack.next() {
                            println!(
                                "{first_idx:>4}\u{2502}(0x31) \u{2500}\u{252C}\u{2500}  JNZ     "
                            );
                            val = number.to_u8().unwrap();
                            format!("{idx:>4}\u{2502}({:#04x})  \u{2514}\u{2500}  {}", val, val)
                                .into()
                        } else {
                            format!("Expected byte at address {:#04x}", idx + 1).into()
                        }
                    }

                    0xFF => {
                        format!("{idx:>4}\u{2502}(0xFF) \u{2500}\u{2500}\u{2500}  Exit    ").into()
                    }
                    _ => format!(
                        "{idx:>4}\u{2502}({:#04x}) \u{2500}\u{2500}\u{2500}  Unknown ",
                        item
                    )
                    .red(),
                };
                if location.is_some() && idx == location.unwrap() {
                    println!("{}", trace_string.red());
                } else {
                    println!("{}", trace_string);
                }
            }
        }
    }
}
