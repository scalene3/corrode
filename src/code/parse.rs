use std::{collections::HashMap, fs, io};

use pest::Parser;
use pest_derive::Parser;

use crate::code::labels::process_labels;

#[derive(Parser)]
#[grammar = "./code/grammar.pest"]
pub struct InputParser;

pub fn parse_code(input_file: &str) -> Result<Vec<u8>, io::Error> {
    let input = fs::read_to_string(input_file)?;
    let parsed = InputParser::parse(Rule::file, &input)
        .expect("Could not parse file")
        .next()
        .unwrap();
    let mut code: Vec<u8> = Vec::new();

    let mut labels: HashMap<String, usize> = HashMap::new();
    process_labels(&mut labels, parsed.clone());

    for line in parsed.into_inner() {
        match line.as_rule() {
            Rule::nop => code.push(0x00),
            Rule::add => code.push(0x01),
            Rule::sub => code.push(0x02),
            Rule::mul => code.push(0x03),
            Rule::div => code.push(0x04),
            Rule::modulus => code.push(0x05),
            Rule::print => code.push(0x10),
            Rule::pchar => code.push(0x11),
            Rule::ret => code.push(0x12),
            Rule::valu8 => {
                code.push(0x20);
                let value: u8 = line.into_inner().as_str().parse::<u8>().unwrap();
                code.push(value)
            }
            Rule::swp => code.push(0x21),
            Rule::pop => code.push(0x22),
            Rule::dup => code.push(0x23),
            Rule::jmp => {
                let inner_rule = line.into_inner().next().unwrap();
                code.push(0x30);

                let value: u8 = match inner_rule.as_rule() {
                    Rule::word => {
                        if let Some(address) = labels.get(inner_rule.as_str()) {
                            *address as u8
                        } else {
                            panic!()
                        }
                    }
                    Rule::address => inner_rule.as_str().parse::<u8>().unwrap(),
                    _ => unreachable!(),
                };
                code.push(value);
            }

            Rule::jnz => {
                let inner_rule = line.into_inner().next().unwrap();
                code.push(0x31);

                let value: u8 = match inner_rule.as_rule() {
                    Rule::word => {
                        if let Some(address) = labels.get(inner_rule.as_str()) {
                            *address as u8
                        } else {
                            panic!()
                        }
                    }
                    Rule::address => inner_rule.as_str().parse::<u8>().unwrap(),
                    _ => unreachable!(),
                };
                code.push(value);
            }

            Rule::label => (),

            Rule::EOI | Rule::exit => code.push(0xFF),

            _ => unreachable!(),
        }
    }
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_testfile() {
        std::fs::write("./testfiles/testfile.cor", "push 10\npush 20\nadd\nret\n").unwrap();

        let retval = parse_code("./testfiles/testfile.cor").unwrap();

        assert_eq!(retval, [0x20, 0x0a, 0x20, 0x14, 0x01, 0x12, 0xFF])
    }
}
