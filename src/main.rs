use std::io::Read;
use std::process;

fn main() {
    enum BFToken {
        Increment,
        Decrement,
        LoopOpen,
        LoopClose,
        Input,
        Output,
        Next,
        Prev,
    }

    struct BF {
        code: String,
    }

    impl BF {
        fn new(code: String) -> BF {
            BF { code: code }
        }

        fn tokenize(&self) -> Vec<BFToken> {
            let mut tokens: Vec<BFToken> = vec![];
            for c in self.code.chars() {
                match c {
                    '.' => tokens.push(BFToken::Output),
                    ',' => tokens.push(BFToken::Input),
                    '+' => tokens.push(BFToken::Increment),
                    '-' => tokens.push(BFToken::Decrement),
                    '[' => tokens.push(BFToken::LoopOpen),
                    ']' => tokens.push(BFToken::LoopClose),
                    '>' => tokens.push(BFToken::Next),
                    '<' => tokens.push(BFToken::Prev),
                    _ => (),
                }
            }
            tokens
        }

        fn run(&self) {
            let tokens = self.tokenize();
            let mut pointer: usize = 0;
            let mut memory: Vec<u8> = vec![0];
            let mut code_pos: usize = 0;
            let mut loop_entries: Vec<usize> = vec![];

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
                        let input: Option<u8> = std::io::stdin()
                            .bytes()
                            .next()
                            .and_then(|result| result.ok())
                            .map(|byte| byte as u8);
                        memory[pointer] = input.unwrap_or(0);
                    }
                    BFToken::Output => {
                        print!("{}", memory[pointer] as char);
                    }
                    BFToken::LoopOpen => {
                        if memory[pointer] == 0 {
                            let mut num_opens: u32 = 1;
                            while code_pos < tokens.len() - 1 {
                                code_pos += 1;
                                match tokens[code_pos] {
                                    BFToken::LoopOpen => num_opens += 1,
                                    BFToken::LoopClose => {
                                        num_opens -= 1;
                                        if num_opens == 0 {
                                            break;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            if num_opens != 0 {
                                eprintln!("Loops (`[]`) must come in groups of 2.");
                                process::exit(1);
                            }
                        } else {
                            loop_entries.push(code_pos - 1);
                        }
                    }
                    BFToken::LoopClose => {
                        if memory[pointer] != 0 {
                            code_pos = loop_entries.pop().unwrap_or_else(|| {
                                eprintln!("Loops (`[]`) must come in groups of 2.");
                                process::exit(1);
                            });
                        } else {
                            loop_entries.pop();
                        }
                    }
                }
                code_pos += 1;
            }
        }
    }

    // hello world
    BF::new(">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++.------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++.".to_string()).run();
    // hello world
    BF::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string()).run();
    // fibinachi
    //BF::new(">++++++++++>+>+[[+++++[>++++++++<-]>.<++++++[>--------<-]+<<<]>.>>[[-]<[>+<-]>>[<<+>+>-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>[-]>+>+<<<-[>+<-]]]]]]]]]]]+>>>]<<<]".to_string()).run();
    //BF::new("".to_string()).run();
}
