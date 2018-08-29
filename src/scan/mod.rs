mod tokens;
use self::tokens::Token;
use self::tokens::Token::*;
use self::State::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process;

enum State {
    Start,
    InID,
    InNum,
    InSlash,
    InComment,
    InStar,
    InEqual,
    InLess,
    InGreat,
    InNot,
    Done(Token),
}

#[derive(Debug)]
pub struct Scanner {
    source: BufReader<File>,
    buffer: [u8; 1],
    reserve: bool,
    id: String,
}

impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = Start;
        loop {
            if self.load_char() == false {
                break;
            }
            state = self.transition(state);
            if let Done(token) = state {
                return Some(token);
            }
        }
        None
    }
}

impl Scanner {
    pub fn new(path: &String) -> Result<Self> {
        let file = File::open(path)?;
        let source = BufReader::new(file);
        let buffer = [0u8; 1];
        let reserve = false;

        Ok(Scanner {
            source,
            buffer,
            reserve,
            id: String::from(""),
        })
    }

    fn transition(&mut self, state: State) -> State {
        let next_char = self.get_char();
        return match state {
            Start => match next_char {
                '+' => Done(PLUS),
                '-' => Done(MINUS),
                '*' => Done(MULTIPLY),
                '/' => InSlash,
                '!' => InNot,
                '=' => InEqual,
                '<' => InLess,
                '>' => InGreat,
                ';' => Done(SEMICOLON),
                ',' => Done(COMMA),
                '(' => Done(L_PAREN),
                ')' => Done(R_PAREN),
                '[' => Done(L_BRACK),
                ']' => Done(R_BRACK),
                '{' => Done(L_BRACE),
                '}' => Done(R_BRACE),
                '0'...'9' => {
                    self.build_identifier(next_char);
                    InNum
                }
                'A'...'Z' | 'a'...'z' => {
                    self.build_identifier(next_char);
                    InID
                }
                _ => Start,
            },
            InSlash => match next_char {
                '*' => InComment,
                _ => Done(DIVIDE),
            },
            InComment => match next_char {
                '*' => InStar,
                _ => InComment,
            },
            InStar => match next_char {
                '/' => Start,
                _ => InComment,
            },
            InNot => match next_char {
                '=' => Done(NOT_EQ),
                _ => Done(ERROR("Unexpected token: !")),
            },
            InEqual => match next_char {
                '=' => Done(EQUAL),
                _ => {
                    self.reserve_char();
                    Done(ASSIGN)
                }
            },
            InLess => match next_char {
                '=' => Done(GT_EQ),
                _ => {
                    self.reserve_char();
                    Done(LESS_THAN)
                }
            },
            InGreat => match next_char {
                '=' => Done(LT_EQ),
                _ => {
                    self.reserve_char();
                    Done(GREATER_THAN)
                }
            },
            InID => match next_char {
                '0'...'9' | 'A'...'Z' | 'a'...'z' => {
                    self.build_identifier(next_char);
                    InID
                }
                _ => {
                    self.reserve_char();
                    let id = self.get_identifier();
                    let state = match id.as_str() {
                        "if" => Done(IF),
                        "else" => Done(ELSE),
                        "while" => Done(WHILE),
                        "void" => Done(VOID),
                        "int" => Done(INT),
                        "return" => Done(RETURN),
                        _ => Done(IDENTIFIER(id)),
                    };
                    self.clear_identifier();
                    state
                }
            },
            InNum => match next_char {
                '0'...'9' => {
                    self.build_identifier(next_char);
                    InNum
                }
                _ => {
                    self.reserve_char();
                    let id = self.get_identifier().parse::<u32>().unwrap();
                    let state = Done(NUMBER(id));
                    self.clear_identifier();
                    state
                }
            },
            Done(x) => Done(x),
        };
    }

    fn get_identifier(&self) -> String {
        self.id.clone()
    }

    fn build_identifier(&mut self, c: char) {
        self.id.push(c);
    }

    fn clear_identifier(&mut self) {
        self.id.clear();
    }

    fn load_char(&mut self) -> bool {
        if self.reserve {
            self.unreserve_char();
            return true;
        }
        match self.source.read(&mut self.buffer) {
            Ok(size) => {
                if size == 1 {
                    true
                } else {
                    false
                }
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1)
            }
        }
    }

    fn get_char(&self) -> char {
        self.buffer[0] as char
    }

    fn reserve_char(&mut self) {
        self.reserve = true;
    }

    fn unreserve_char(&mut self) {
        self.reserve = false;
    }
}
