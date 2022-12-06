# A Brain F\*ck Interpreter

This is just a simple BF interpreter written as my first project in rust.

## Usage

```rust
let hello_world = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++.------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++.";

// prints to console and returns the output
let output: String = BF::new(hello_world, "").run();

// run with input
BF::new(",>,>,.<.<.", "abc").run(); // cba

// or type in input
BF::new(",>,>,.<.<.", "").run(); // ???
```
