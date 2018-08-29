use self::Token::*;
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
            INT => String::from("int"),
            WHILE => String::from("while"),
            IF => String::from("if"),
            ELSE => String::from("else"),
            RETURN => String::from("return"),
            VOID => String::from("void"),
            PLUS => String::from("plus"),
            MINUS => String::from("minus"),
            MULTIPLY => String::from("multiply"),
            DIVIDE => String::from("divide"),
            LESS_THAN => String::from("less than"),
            LT_EQ => String::from("less than or equal to"),
            GREATER_THAN => String::from("greater than"),
            GT_EQ => String::from("greater than or equal to"),
            EQUAL => String::from("equal"),
            NOT_EQ => String::from("not equal"),
            ASSIGN => String::from("assign"),
            SEMICOLON => String::from("semicolon"),
            COMMA => String::from(","),
            R_PAREN => String::from(")"),
            L_PAREN => String::from("("),
            R_BRACK => String::from("]"),
            L_BRACK => String::from("["),
            R_BRACE => String::from("}"),
            L_BRACE => String::from("{"),
            IDENTIFIER(ref x) => x.clone(),
            NUMBER(ref x) => x.to_string(),
            ERROR(x) => String::from(x),
        };
        write!(f, "{}", printable)
    }
}
