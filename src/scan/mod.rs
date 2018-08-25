use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process;
mod tokens;
use self::tokens::Token;

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

pub struct Scanner {
    source: BufReader<File>,
    buffer: Vec<u8>,
    reserve: bool,
    id: String,
}

impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut state = State::Start;
        loop {
            if self.load_char() == false {
                break;
            }
            state = self.transition(state);
            if let State::Done(token) = state {
                return Some(token);
            }
        }
        None
    }
}

impl Scanner {
    pub fn new(path: &String) -> Result<Scanner> {
        let file = File::open(path)?;
        let source = BufReader::new(file);
        let buffer: Vec<u8> = vec![0];
        let reserve = false;

        Ok(Scanner {
            source,
            buffer,
            reserve,
            id: String::from(""),
        })
    }

    fn transition(&mut self, state: State) -> State {
        let c = self.get_char();
        return match state {
            State::Start => match c {
                '+' => State::Done(Token::PLUS),
                '-' => State::Done(Token::MINUS),
                '*' => State::Done(Token::MULTIPLY),
                '/' => State::InSlash,
                '!' => State::InNot,
                '=' => State::InEqual,
                '<' => State::InLess,
                '>' => State::InGreat,
                ';' => State::Done(Token::SEMICOLON),
                ',' => State::Done(Token::COMMA),
                '(' => State::Done(Token::L_PAREN),
                ')' => State::Done(Token::R_PAREN),
                '[' => State::Done(Token::L_BRACK),
                ']' => State::Done(Token::R_BRACK),
                '{' => State::Done(Token::L_BRACE),
                '}' => State::Done(Token::R_BRACE),
                '0'...'9' => {
                    self.build_identifier(c);
                    State::InNum
                }
                'A'...'Z' | 'a'...'z' => {
                    self.build_identifier(c);
                    State::InID
                }
                _ => State::Start,
            },
            State::InSlash => match c {
                '*' => State::InComment,
                _ => State::Done(Token::DIVIDE),
            },
            State::InComment => match c {
                '*' => State::InStar,
                _ => State::InComment,
            },
            State::InStar => match c {
                '/' => State::Start,
                _ => State::InComment,
            },
            State::InNot => match c {
                '=' => State::Done(Token::NOT_EQ),
                _ => State::Done(Token::ERROR("Unexpected token: !")),
            },
            State::InEqual => match c {
                '=' => State::Done(Token::EQUAL),
                _ => {
                    self.reserve_char();
                    State::Done(Token::ASSIGN)
                }
            },
            State::InLess => match c {
                '=' => State::Done(Token::GT_EQ),
                _ => {
                    self.reserve_char();
                    State::Done(Token::LESS_THAN)
                }
            },
            State::InGreat => match c {
                '=' => State::Done(Token::LT_EQ),
                _ => {
                    self.reserve_char();
                    State::Done(Token::GREATER_THAN)
                }
            },
            State::InID => match c {
                '0'...'9' | 'A'...'Z' | 'a'...'z' => {
                    self.build_identifier(c);
                    State::InID
                }
                _ => {
                    self.reserve_char();
                    let id = self.get_identifier();
                    let state = match id.as_str() {
                        "if" => State::Done(Token::IF),
                        "else" => State::Done(Token::ELSE),
                        "while" => State::Done(Token::WHILE),
                        "void" => State::Done(Token::VOID),
                        "int" => State::Done(Token::INT),
                        "return" => State::Done(Token::RETURN),
                        _ => State::Done(Token::IDENTIFIER(id)),
                    };
                    self.clear_identifier();
                    state
                }
            },
            State::InNum => match c {
                '0'...'9' => {
                    self.build_identifier(c);
                    State::InNum
                }
                _ => {
                    self.reserve_char();
                    let id = self.get_identifier().parse::<u32>().unwrap();
                    let state = State::Done(Token::NUMBER(id));
                    self.clear_identifier();
                    state
                }
            },
            State::Done(x) => State::Done(x),
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
