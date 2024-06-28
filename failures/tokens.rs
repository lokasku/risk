use chumsky::prelude::*;

#[derive(Debug, Clone)]
enum Token {
    // Literal
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    True,
    False,

    // Ident
    PCIdentifier(&'a str),
    Identifier(&'a str),

    // Keywords
    Def,
    Type,
    Let, In,
    Match, With,
    If, Then, Else,

    // Delimiters
    LParen, RParen,
    LBracket, RBracket,

    // Symbols
    BinOp(&'a str),
    Symbol(&'a str) // : , -> , :: , ',' , | , =
}