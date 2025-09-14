/*!Module controlling parsing, and executing .cor files
 *
 * #1 byte Instructions.
 * NOP => ()
 * ADD => ( a b -- a + b )
 * SUB => ( a b -- a - b )
 * MUL => ( a b -- a * b )
 * DIV => ( a b -- a / b )
 * MOD => ( a b -- a % b )
 *
 * SWP => ( a b -- b a )
 * POP => ( a -- )
 * DUP => ( a -- a a )
 *
 * PRINT => ( a --> println! ) \\ println! top of stack
 * PCHAR => ( ... a -> println! ) \\ println! stack as UTF-8 until 0
 * RET => ( -- ) \\ return top of stack
 *
 * EXIT => () \\ stop execution
 *
 * #2 byte Instructions.
 * PUSH A => ( -- A )
 * JMP => () \\ go to address (%int) or label ($string)
 * JNZ => ( -- ) \\ go to address (%int) or label ($string) IF stack top is NOT == 0
 */

pub mod code_execution;
pub mod labels;
pub mod parse;
