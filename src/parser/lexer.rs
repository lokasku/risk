/*
    Risk is a purely functional, strongly typed language.
    Copyright (C) 2024, Lokasku & NightProg

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ast::Span;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"/[*]([^*]|([*][^/]))*[*]/")] /* ... */
#[logos(skip r"\/\/.*")] //
pub enum TokenKind<'a> {
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

    #[regex("\"[^\"]+\"", |lex| &lex.slice()[1..lex.slice().len() - 1])]
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
    #[token("|")]
    Pipe,

    #[token("_")]
    Underscore,

    #[token("\\")]
    InversedSlash,

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

    #[token(";")]
    Semicolon,

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
    Assign,

    #[token(" ")]
    Space,

    #[token("\t")]
    Tab,

    #[token("\n")]
    Newline,

    Eof,
}

impl<'a> TokenKind<'a> {
    pub fn is_literal(&self) -> bool {
        match self {
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::Char(_) => true,
            TokenKind::True | TokenKind::False => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            TokenKind::Identifier(_) | TokenKind::PCIdentifier(_) => true,
            _ => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            TokenKind::Space | TokenKind::Tab | TokenKind::Newline => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'token> {
    pub kind: TokenKind<'token>,
    pub span: Span,
}

impl<'token> Token<'token> {
    pub fn new(kind: TokenKind<'token>, span: Span) -> Self {
        Token { kind, span }
    }
}

pub fn lexer<'a>(input: &'a str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(Token {
            kind: token.unwrap(),
            span: Span::new(
                lexer.span().start,
                lexer.span().end,
                lexer.slice().to_string(),
            ),
        });
    }

    tokens.push(Token::new(
        TokenKind::Eof,
        Span::new(
            lexer.span().start,
            lexer.span().end,
            lexer.slice().to_string(),
        ),
    ));
    tokens
}

#[macro_export]
macro_rules! token {
    (if) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::If,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (then) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Then,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (else) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Else,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (let) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Let,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (in) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::In,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (match) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Match,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (with) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::With,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (type) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Type,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (true) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::True,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (false) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::False,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (pc_identifier, $id: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::PCIdentifier($id),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (identifier, $id: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Identifier($id),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };

    (integer, $int: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Integer($int),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (float, $float: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Float($float),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (string, $str: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::String($str),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (char, $char: expr) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Char($char),
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (lparen) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::LParen,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (rparen) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::RParen,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (lbracket) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::LBracket,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (rbracket) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::RBracket,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (inversed_slash) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::InversedSlash,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (_) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Underscore,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (|) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Pipe,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (->) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Arrow,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (,) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Comma,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (;) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Semicolon,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (::) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::DoubleCollon,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (<) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Lt,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (>) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Gt,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (>=) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Gte,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (<=) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Lte,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (&&) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::And,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (||) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Or,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (==) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Eq,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (!=) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Neq,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (:) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Colon,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (+) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Add,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (-) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Sub,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (*) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Mul,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (/) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Div,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (^) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Exp,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (%) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Mod,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
    (=) => {
        $crate::parser::lexer::Token::new(
            $crate::parser::lexer::TokenKind::Assign,
            $crate::ast::Span::new(0, 0, "".to_string()),
        )
    };
}
