use std::io::Read;
use std::process;

fn main() {
    enum BFToken {
        Increment,
        Decrement,
        LoopOpen(usize),
        LoopClose(usize),
        Input,
        Output,
        Next,
        Prev,
    }

    struct BF {
        code: Vec<char>,
        input: Vec<char>,
    }

    impl BF {
        fn new(code: &str, input: &str) -> BF {
            let code = code.chars().collect();
            let input = input.chars().collect();
            BF { code, input }
        }

        fn tokenize(&self) -> Vec<BFToken> {
            let mut tokens: Vec<BFToken> = vec![];
            for (index, c) in self.code.iter().enumerate() {
                match c {
                    '.' => tokens.push(BFToken::Output),
                    ',' => tokens.push(BFToken::Input),
                    '+' => tokens.push(BFToken::Increment),
                    '-' => tokens.push(BFToken::Decrement),
                    '[' => tokens.push(BFToken::LoopOpen(index + 1)),
                    ']' => tokens.push(BFToken::LoopClose(index + 1)),
                    '>' => tokens.push(BFToken::Next),
                    '<' => tokens.push(BFToken::Prev),
                    _ => (),
                }
            }
            tokens
        }

        fn run(&self) -> String {
            let tokens = self.tokenize();
            let mut pointer: usize = 0;
            let mut input_count: usize = 0;
            let mut memory: Vec<u8> = vec![0];
            let mut code_pos: usize = 0;
            let mut loop_entries: Vec<usize> = vec![];
            let mut output = String::from("");

            while code_pos < tokens.len() {
                match tokens[code_pos] {
                    BFToken::Increment => {
                        if memory[pointer] < u8::MAX {
                            memory[pointer] += 1;
                        } else {
                            memory[pointer] = u8::MIN;
                        }
                    }
                    BFToken::Decrement => {
                        if memory[pointer] > u8::MIN {
                            memory[pointer] -= 1;
                        } else {
                            memory[pointer] = u8::MAX;
                        }
                    }
                    BFToken::Next => {
                        pointer += 1;
                        if pointer >= memory.len() {
                            memory.push(0);
                        }
                    }
                    BFToken::Prev => {
                        if pointer != 0 {
                            pointer -= 1;
                        }
                    }
                    BFToken::Input => {
                        let input: u8;
                        if input_count < self.input.len() {
                            input = self.input[input_count] as u8;
                            input_count += 1;
                        } else {
                            input = std::io::stdin()
                                .bytes()
                                .next()
                                .and_then(|result| result.ok())
                                .map(|byte| byte as u8)
                                .unwrap_or(0);
                        }
                        memory[pointer] = input
                    }
                    BFToken::Output => {
                        let c = memory[pointer] as char;
                        output.push(c);
                        print!("{c}");
                    }
                    BFToken::LoopOpen(index) => {
                        if memory[pointer] == 0 {
                            let mut num_opens: u32 = 1;
                            while code_pos < tokens.len() - 1 {
                                code_pos += 1;
                                match tokens[code_pos] {
                                    BFToken::LoopOpen(_) => num_opens += 1,
                                    BFToken::LoopClose(_) => {
                                        num_opens -= 1;
                                        if num_opens == 0 {
                                            break;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            if num_opens != 0 {
                                eprintln!(
                                    "Error: char {index}, loops `[]` must come in groups of 2."
                                );
                                process::exit(1);
                            }
                        } else {
                            loop_entries.push(code_pos - 1);
                        }
                    }
                    BFToken::LoopClose(index) => {
                        if loop_entries.len() < 1 {
                            eprintln!("Error: char {index}, loops `[]` must come in groups of 2.");
                            process::exit(1);
                        }
                        if memory[pointer] != 0 {
                            code_pos = loop_entries.pop().unwrap();
                        } else {
                            loop_entries.pop();
                        }
                    }
                }
                code_pos += 1;
            }
            output
        }
    }

    let hello_world1 = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++.------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++.";
    let hello_world2 = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let _fibonacci = ">++++++++++>+>+[[+++++[>++++++++<-]>.<++++++[>--------<-]+<<<]>.>>[[-]<[>+<-]>>[<<+>+>-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>[-]>+>+<<<-[>+<-]]]]]]]]]]]+>>>]<<<]";

    assert_eq!(
        BF::new(hello_world1, "").run(),
        "Hello world!\n".to_string()
    );
    assert_eq!(
        BF::new(hello_world2, "").run(),
        "Hello World!\n".to_string()
    );
    assert_eq!(BF::new(",>,>,.<.<.", "abc").run(), "cba".to_string());
}
