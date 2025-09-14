# CORRODE
This is a stack based language loosely inspired by Forth and Java Bytecode.

## Why?
I started this project mainly as a way to learn rust.

## Example
The below example prints 30 through 1.
```
PUSH 10
PUSH 20
ADD
loop: PRINT
PUSH 1
SUB
JNZ loop
EXIT
```
## TODO

- [ ] Comments
- [ ] Other Stack based commands
    - [x] dup
    - [x] swap
    - [x] pop
    - [ ] more?
- [ ] Improved string handling
- [ ] Improved array handling
- [ ] Type system
- [ ] Functions
