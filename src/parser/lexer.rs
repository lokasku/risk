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

fn lexer<'a>() -> impl Parser<&'a str, Vec<Token>> {
    let ident = text::ident::<_, Simple<char>>().map(|kw: &'a str| match kw {
        "def" => Token::Def,
        "type" => Token::Type,
        "let" => Token::Let,
        "in" => Token::In,
        "match" => Token::Match,
        "with" => Token::With,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        x => Token::Identifier(x)
    });

    let token = ident;
    token
        .map(|t| t)
        .padded()
        .repeated()
        .collect()
}