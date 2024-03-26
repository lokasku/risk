use logos::{Logos  , Lexer as LLexer };

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
// #[logos(error = Error)]
#[logos(skip r"/[*]([^*]|([*][^/]))*[*]/")] /* ... */
#[logos(skip r"\/\/.*")] //
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    
    // Keywords

    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("match")]
    Match,

    #[token("with")]
    With,

    #[token("type")]
    Type,

    // Primitives

    #[regex(r"[-+]?\d+", |lex| lex.slice().parse().ok(), priority = 3)]
    Integer(i64),

    #[regex(r"[-+]?\d+(\.\d*)?", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex("\"[^\"]+\"", |lex| lex.slice()[1..lex.slice().len() - 1].to_owned())]
    String(&'a str),

    #[regex("'[^']'", |lex| &lex.slice()[1..lex.slice().len() - 1])]
    Char(&'a str),

    #[token("True")]
    True,

    #[token("False")]
    False,

    #[regex(r"[A-Z][a-zA-Z0-9']*")]
    PCIdentifier(&'a str),

    #[regex(r"[a-z][a-zA-Z0-9']*")]
    Identifier(&'a str),

    // Syntactic operators

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("->")]
    Arrow,

    #[token(",")]
    Comma,

    #[token("::")]
    DoubleCollon,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token(">=")]
    Gte,

    #[token("<=")]
    Lte,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token(":")]
    Colon,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("^")]
    Exp,

    #[token("%")]
    Mod,
    
    #[token("=")]
    Assign
}
pub struct Lexer<'a> {
    pub lexer: LLexer<'a, Token<'a>>,
    // pub errors: Vec<Error>
}

pub fn lexer<'a>(input: &'a str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(token.unwrap());
    }
    tokens
}
