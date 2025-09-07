/*!A stack based language loosely inspired by Java Bytecode and Forth
!*/

use corrode::code::code_execution::run;

use anyhow::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run::<i64>("hello_world.cor")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test() {
        let retval: u8 = run("./testfiles/testfile.cor").unwrap();
        assert_eq!(retval, 30)
    }
}
