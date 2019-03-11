use scanner::tokens;

pub enum State {
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
    Done(tokens::Token),
}
