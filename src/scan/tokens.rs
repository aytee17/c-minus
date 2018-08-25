use std::fmt;
use std::str;

#[allow(non_camel_case_types)]
pub enum Token {
    INT,
    WHILE,
    IF,
    ELSE,
    RETURN,
    VOID,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LESS_THAN,
    LT_EQ,
    GREATER_THAN,
    GT_EQ,
    EQUAL,
    NOT_EQ,
    ASSIGN,
    SEMICOLON,
    COMMA,
    R_PAREN,
    L_PAREN,
    R_BRACK,
    L_BRACK,
    R_BRACE,
    L_BRACE,
    IDENTIFIER(String),
    NUMBER(u32),
    ERROR(&'static str),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Token::INT => String::from("int"),
            Token::WHILE => String::from("while"),
            Token::IF => String::from("if"),
            Token::ELSE => String::from("else"),
            Token::RETURN => String::from("return"),
            Token::VOID => String::from("void"),
            Token::PLUS => String::from("plus"),
            Token::MINUS => String::from("minus"),
            Token::MULTIPLY => String::from("multiply"),
            Token::DIVIDE => String::from("divide"),
            Token::LESS_THAN => String::from("less than"),
            Token::LT_EQ => String::from("less than or equal to"),
            Token::GREATER_THAN => String::from("greater than"),
            Token::GT_EQ => String::from("greater than or equal to"),
            Token::EQUAL => String::from("equal"),
            Token::NOT_EQ => String::from("not equal"),
            Token::ASSIGN => String::from("assign"),
            Token::SEMICOLON => String::from("semicolon"),
            Token::COMMA => String::from(","),
            Token::R_PAREN => String::from(")"),
            Token::L_PAREN => String::from("("),
            Token::R_BRACK => String::from("]"),
            Token::L_BRACK => String::from("["),
            Token::R_BRACE => String::from("}"),
            Token::L_BRACE => String::from("{"),
            Token::IDENTIFIER(ref x) => x.clone(),
            Token::NUMBER(ref x) => x.to_string(),
            Token::ERROR(x) => String::from(x),
        };
        write!(f, "{}", printable)
    }
}
