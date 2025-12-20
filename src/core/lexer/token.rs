use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    String,
    Number,
    Whitespace,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
}
