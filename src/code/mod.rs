/*!Module controlling parsing, and executing .cor files
 *
 * #1 byte Instructions.
 * NOP => Does Nothing.
 * ADD => Adds top two elements on the stack, leaving result on stack.
 */

pub mod code_execution;
pub mod labels;
pub mod parse;
