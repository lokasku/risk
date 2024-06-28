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

use crate::ast::{App, BinOp, Bool, Literal, LiteralKind, TypeDecl};
use crate::parser::lexer::{lexer, Token};
use crate::{ast, token};

mod error;
pub mod lexer;

type ParserResult<T> = Result<T, error::Error>;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    pub content: &'a str,
    pub tokens: Vec<lexer::Token<'a>>,
    pub current_span: ast::Span,
    pub current: usize,
    pub oneline: bool,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Parser<'a> {
        Parser {
            content,
            tokens: lexer(content),
            current_span: ast::Span::new(0, 0, "".to_string()),
            current: 0,
            oneline: false,
        }
    }

    fn start_recording(&mut self) -> usize {
        self.current
    }

    fn end_recording(&mut self, start: usize) -> ast::Span {
        let mut end = self.current;
        while self.tokens[end - 1].kind.is_whitespace() {
            end -= 1;
        }
        let input = self.tokens[start..end]
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.span.input);

        ast::Span::new(start, end, input)
    }

    fn expect_new_line(&mut self) -> ParserResult<()> {
        if self.peek().kind == lexer::TokenKind::Newline {
            self.advance()?;
            Ok(())
        } else {
            Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "new line".to_string(),
                    found: self.peek().span,
                },
                self.peek().span,
            ))
        }
    }

    fn advance(&mut self) -> ParserResult<lexer::Token<'a>> {
        if self.is_eof() {
            return Err(error::Error::new(
                error::ErrorKind::UnexpectedEndOfInput,
                self.peek().span,
            ));
        }
        if self.oneline && self.peek().kind == lexer::TokenKind::Newline {
            return Err(error::Error::new(
                error::ErrorKind::ExpectedNewline {
                    found: self.peek().span.clone(),
                },
                self.peek().span,
            ));
        }
        self.current += 1;
        if self.is_eof() {
            return Err(error::Error::new(
                error::ErrorKind::UnexpectedEndOfInput,
                self.peek().span,
            ));
        }
        let token = self.tokens[self.current].clone();

        if token.kind.is_whitespace()
            && (self.peek().kind != lexer::TokenKind::Newline || !self.oneline)
        {
            return self.advance();
        }
        Ok(token)
    }

    fn expect(&mut self, token: Token<'a>) -> ParserResult<Token<'a>> {
        let tok = self.advance()?;
        if self.peek().kind == token.kind {
            Ok(tok)
        } else {
            Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: format!("{:?}", token.kind),
                    found: self.peek().span,
                },
                self.peek().span,
            ))
        }
    }

    fn expect_current(&mut self, token: Token<'a>) -> ParserResult<Token<'a>> {
        if self.peek().kind == token.kind {
            self.advance()
        } else {
            Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: format!("{:?}", token.kind),
                    found: self.peek().span,
                },
                self.peek().span,
            ))
        }
    }

    fn match_token(&mut self, token: lexer::TokenKind) -> ParserResult<bool> {
        if self.peek().kind == token {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn future(&self) -> Token<'a> {
        self.tokens[self.current + 1].clone()
    }

    fn past(&self) -> lexer::Token<'a> {
        let mut n = 1;
        while self.tokens[self.current - n].kind.is_whitespace() {
            n += 1;
        }
        self.tokens[self.current - n].clone()
    }

    fn expect_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::Identifier(id) => {
                let index = self.start_recording();
                self.advance()?;
                let span = self.end_recording(index);
                Ok(ast::Identifier::new(id.to_string(), span))
            }
            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: peek.span.clone(),
                },
                peek.span,
            )),
        }
    }

    fn expect_pc_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::PCIdentifier(id) => {
                let index = self.start_recording();
                self.advance()?;
                let span = self.end_recording(index);
                Ok(ast::Identifier::new(id.to_string(), span))
            }
            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "pascal case identifier".to_string(),
                    found: peek.span.clone(),
                },
                peek.span,
            )),
        }
    }

    fn expect_any_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let index = self.start_recording();
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::Identifier(id) => {
                self.advance()?;
                let span = self.end_recording(index);
                Ok(ast::Identifier::new(id.to_string(), span))
            }
            lexer::TokenKind::PCIdentifier(id) => {
                self.advance()?;
                let span = self.end_recording(index);
                Ok(ast::Identifier::new(id.to_string(), span))
            }
            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: peek.span.clone(),
                },
                peek.span,
            )),
        }
    }

    fn peek(&self) -> lexer::Token<'a> {
        self.tokens[self.current].clone()
    }

    fn is_operator(&self) -> bool {
        match self.peek().kind {
            lexer::TokenKind::Add
            | lexer::TokenKind::Sub
            | lexer::TokenKind::Mul
            | lexer::TokenKind::Div
            | lexer::TokenKind::Exp
            | lexer::TokenKind::Mod => true,
            _ => false,
        }
    }

    fn back_up(&mut self) -> Option<Token> {
        self.current -= 1;
        if self.is_eof() {
            return None;
        }
        let token = self.tokens[self.current].clone();

        if token.kind.is_whitespace() {
            return self.back_up();
        }
        Some(token)
    }

    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn parse(&mut self) -> ParserResult<ast::Program> {
        let mut statements = Vec::new();
        while !self.is_eof() && self.peek().kind != lexer::TokenKind::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(ast::Program::new(statements))
    }

    fn parse_statement(&mut self) -> ParserResult<ast::Statement> {
        self.oneline = false;
        if self.peek().kind == lexer::TokenKind::Newline {
            self.advance()?;
        }
        let res = match self.peek().kind {
            lexer::TokenKind::Identifier(_) => self.parse_stmt_identifier(),
            lexer::TokenKind::Type => self.parse_type_decl(),
            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "statement".to_string(),
                    found: self.peek().span.clone(),
                },
                self.peek().span.clone(),
            )),
        }?;
        if self.peek().kind != lexer::TokenKind::Eof {
            if self.peek().kind != lexer::TokenKind::Newline {
                return Err(error::Error::new(
                    error::ErrorKind::TooMuchExpr {
                        found: self.peek().span.clone(),
                    },
                    self.peek().span.clone(),
                ));
            }
        }
        Ok(res)
    }

    fn parse_stmt_identifier(&mut self) -> ParserResult<ast::Statement> {
        self.oneline = true;
        let index = self.start_recording();
        let id = self.expect_identifier()?;
        if self.peek().kind == lexer::TokenKind::DoubleCollon {
            self.advance()?;
            let ty = self.parse_type()?;
            Ok(ast::Statement::TypeAssign(ast::TypeAssign::new(
                id,
                ty,
                self.end_recording(index),
            )))
        } else {
            self.back_up();
            Ok(ast::Statement::Bind(self.parse_bind()?))
        }
    }

    fn parse_type_decl(&mut self) -> ParserResult<ast::Statement> {
        let index = self.start_recording();
        self.advance()?;
        let id = self.expect_pc_identifier()?;
        let mut idents = Vec::new();
        while !self.match_token(lexer::TokenKind::Assign)? {
            let id = self.expect_identifier()?;
            idents.push(id);
        }

        let mut variants = Vec::new();
        let variant = self.parse_variant()?;
        variants.push(variant);
        while self.match_token(lexer::TokenKind::Pipe)? {
            let variant = self.parse_variant()?;
            variants.push(variant);
        }

        Ok(ast::Statement::TypeDecl(TypeDecl::new(
            id,
            idents,
            variants,
            self.end_recording(index),
        )))
    }

    fn parse_variant(&mut self) -> ParserResult<ast::Variant> {
        let index = self.start_recording();
        let id = self.expect_pc_identifier()?;
        let mut ty = Vec::new();
        let mut current_ty;
        loop {
            current_ty = self.parse_type();
            if current_ty.is_ok() {
                ty.push(current_ty.unwrap());
            } else {
                break;
            }
        }
        Ok(ast::Variant::new(id, ty, self.end_recording(index)))
    }

    fn parse_bind(&mut self) -> ParserResult<ast::Bind> {
        let index = self.start_recording();
        let id = self.expect_identifier()?;
        let mut args = Vec::new();
        while !self.match_token(lexer::TokenKind::Assign)? {
            let arg = self.parse_pattern()?;
            args.push(arg);
        }
        let expr = self.parse_expr()?;
        Ok(ast::Bind::new(id, args, expr, self.end_recording(index)))
    }

    fn parse_expr(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_factor()?;
        while self.match_token(lexer::TokenKind::Add)? || self.match_token(lexer::TokenKind::Sub)? {
            let op = match self.past().kind {
                lexer::TokenKind::Add => ast::BinOp::Add,
                lexer::TokenKind::Sub => ast::BinOp::Sub,
                e => panic!("Unexpected token: {:?}", e),
            };
            let rhs = self.parse_factor()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_or_and()?;

        while self.match_token(lexer::TokenKind::Mul)?
            || self.match_token(lexer::TokenKind::Div)?
            || self.match_token(lexer::TokenKind::Mod)?
        {
            let op = match self.past().kind {
                lexer::TokenKind::Mul => BinOp::Mul,
                lexer::TokenKind::Div => BinOp::Div,
                lexer::TokenKind::Mod => BinOp::Mod,
                _ => unreachable!(),
            };
            let rhs = self.parse_or_and()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_or_and(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_cmp()?;

        while self.match_token(lexer::TokenKind::Or)? || self.match_token(lexer::TokenKind::And)? {
            let op = match self.past().kind {
                lexer::TokenKind::Or => ast::BinOp::Or,
                lexer::TokenKind::And => ast::BinOp::And,
                _ => unreachable!(),
            };
            let rhs = self.parse_cmp()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_cmp(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_exp()?;

        while self.match_token(lexer::TokenKind::Eq)?
            || self.match_token(lexer::TokenKind::Neq)?
            || self.match_token(lexer::TokenKind::Lt)?
            || self.match_token(lexer::TokenKind::Gt)?
            || self.match_token(lexer::TokenKind::Lte)?
            || self.match_token(lexer::TokenKind::Gte)?
        {
            let op = match self.past().kind {
                lexer::TokenKind::Eq => BinOp::Eq,
                lexer::TokenKind::Neq => BinOp::Ineq,
                lexer::TokenKind::Lt => BinOp::LessThan,
                lexer::TokenKind::Gt => BinOp::GreaterThan,
                lexer::TokenKind::Lte => BinOp::LessThanOrEq,
                lexer::TokenKind::Gte => BinOp::GreaterThanOrEq,
                _ => unreachable!(),
            };
            let rhs = self.parse_exp()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_exp(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_list_cons()?;

        while self.match_token(lexer::TokenKind::Exp)? {
            let rhs = self.parse_list_cons()?;
            lhs = ast::Expr::BinOp(
                BinOp::Exp,
                Box::new(lhs),
                Box::new(rhs),
                self.end_recording(index),
            );
        }

        Ok(lhs)
    }

    fn parse_list_cons(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_annotation()?;
        while self.match_token(lexer::TokenKind::DoubleCollon)? {
            let rhs = self.parse_annotation()?;
            lhs = ast::Expr::BinOp(
                BinOp::ListCons,
                Box::new(lhs),
                Box::new(rhs),
                self.end_recording(index),
            );
        }

        Ok(lhs)
    }

    fn parse_annotation(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        let mut lhs = self.parse_primary()?;
        if self.match_token(lexer::TokenKind::DoubleCollon)? {
            let ty = self.parse_type()?;
            lhs = ast::Expr::Ann(Box::new(lhs), ty, self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_literal(&mut self) -> ParserResult<Literal> {
        let index = self.start_recording();
        match self.peek().kind {
            lexer::TokenKind::Integer(i) => {
                self.advance()?;
                Ok(Literal::new(
                    LiteralKind::Integer(i),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::Float(f) => {
                self.advance()?;
                Ok(Literal::new(
                    LiteralKind::Float(f),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::String(s) => {
                self.advance()?;
                Ok(Literal::new(
                    LiteralKind::String(s.to_string()),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::True => {
                self.advance()?;
                Ok(Literal::new(
                    LiteralKind::Bool(Bool::True),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::False => {
                self.advance()?;
                Ok(Literal::new(
                    LiteralKind::Bool(Bool::False),
                    self.end_recording(index),
                ))
            }
            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "literal".to_string(),
                    found: self.peek().span.clone(),
                },
                self.peek().span.clone(),
            )),
        }
    }

    fn parse_primary(&mut self) -> ParserResult<ast::Expr> {
        let index = self.start_recording();
        match self.peek().kind {
            lexer::TokenKind::Integer(i) => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::Integer(i),
                    self.end_recording(index),
                )))
            }
            lexer::TokenKind::Float(f) => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::Float(f),
                    self.end_recording(index),
                )))
            }
            lexer::TokenKind::String(s) => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::String(s.to_string()),
                    self.end_recording(index),
                )))
            }
            lexer::TokenKind::True => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::Bool(Bool::True),
                    self.end_recording(index),
                )))
            }
            lexer::TokenKind::False => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::Bool(Bool::False),
                    self.end_recording(index),
                )))
            }
            lexer::TokenKind::Char(c) => {
                self.advance()?;
                Ok(ast::Expr::Literal(Literal::new(
                    LiteralKind::Char(c.chars().nth(0).unwrap()),
                    self.end_recording(index),
                )))
            }
            n @ (lexer::TokenKind::Identifier(_) | lexer::TokenKind::PCIdentifier(_)) => {
                let id = self.expect_any_identifier()?;
                let cloned = self.clone();
                let mut expr = self.parse_expr();

                if expr.is_ok() {
                    let mut exprs = vec![expr.clone()?];
                    while expr.is_ok() {
                        exprs.push(expr.unwrap());
                        expr = self.parse_expr();
                    }

                    return Ok(ast::Expr::App(App::new(
                        id,
                        exprs,
                        self.end_recording(index),
                    )));
                } else {
                    *self = cloned;
                }

                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Expr::Identifier(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Expr::PCIdentifier(id)),
                    _ => unreachable!(),
                }
            }
            lexer::TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect_current(token![rparen])?;
                Ok(expr)
            }
            lexer::TokenKind::Let => {
                self.advance()?;
                let mut binds = Vec::new();
                while !self.match_token(lexer::TokenKind::In)? {
                    let bind = self.parse_bind()?;
                    self.expect_current(token![;])?;
                    binds.push(bind);
                }

                let expr = self.parse_expr()?;
                Ok(ast::Expr::Let(
                    binds,
                    Box::new(expr),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::If => {
                self.advance().unwrap();
                let cond = self.parse_expr()?;
                self.expect_current(token![then])?;
                let then = self.parse_expr()?;
                self.expect_current(token![else])?;
                let else_ = self.parse_expr()?;
                Ok(ast::Expr::Condition(
                    Box::new(cond),
                    Box::new(then),
                    Box::new(else_),
                    self.end_recording(index),
                ))
            }
            lexer::TokenKind::Match => {
                self.oneline = false;
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect_current(token![with])?;
                let mut arms = Vec::new();
                self.expect_current(token![|])?;
                let arm_pat = Box::new(self.parse_pattern()?);
                self.expect_current(token![->])?;
                let arm_expr = Box::new(self.parse_expr()?);
                arms.push((arm_pat, arm_expr));
                while self.match_token(lexer::TokenKind::Pipe)? {
                    let pat = Box::new(self.parse_pattern()?);
                    self.expect_current(token![->])?;
                    let expr = Box::new(self.parse_expr()?);
                    arms.push((pat, expr));
                }
                self.oneline = true;
                Ok(ast::Expr::Match(
                    Box::new(expr),
                    arms,
                    self.end_recording(index),
                ))
            }

            lexer::TokenKind::InversedSlash => {
                self.advance()?;
                let mut pats = Vec::new();

                while !self.match_token(lexer::TokenKind::Arrow)? {
                    let pat = self.parse_pattern()?;
                    pats.push(pat);
                }

                let expr = self.parse_expr()?;
                Ok(ast::Expr::Lambda(
                    pats,
                    Box::new(expr),
                    self.end_recording(index),
                ))
            }

            lexer::TokenKind::LBracket => {
                self.advance()?;
                let mut exprs = Vec::new();
                while !self.match_token(lexer::TokenKind::RBracket)? {
                    let expr = self.parse_expr()?;
                    exprs.push(expr);
                    self.expect(token![,])?;
                }
                Ok(ast::Expr::List(exprs, self.end_recording(index)))
            }

            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "primary expression".to_string(),
                    found: self.peek().span.clone(),
                },
                self.peek().span.clone(),
            )),
        }
    }

    fn parse_pattern(&mut self) -> ParserResult<ast::Pattern> {
        let index = self.start_recording();
        let pat = self.parse_pattern_primary()?;
        if self.match_token(lexer::TokenKind::Colon)? {
            let pat2 = self.parse_pattern()?;
            Ok(ast::Pattern::ListCons(
                Box::new(pat),
                Box::new(pat2),
                self.end_recording(index),
            ))
        } else {
            Ok(pat)
        }
    }

    fn parse_pattern_primary(&mut self) -> ParserResult<ast::Pattern> {
        let index = self.start_recording();
        match self.peek().kind {
            n if n.is_literal() => {
                let lit = self.parse_literal()?;
                Ok(ast::Pattern::Literal(lit))
            }
            n if n.is_identifier() => {
                let id = self.expect_any_identifier()?;
                let cloned = self.clone();
                let mut ty = self.parse_type();
                if ty.is_ok() {
                    let mut types = vec![ty.clone().unwrap()];
                    while ty.is_ok() {
                        types.push(ty.unwrap());
                        ty = self.parse_type();
                    }

                    return Ok(ast::Pattern::App(id, types, self.end_recording(index)));
                } else {
                    *self = cloned;
                }

                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Pattern::Variable(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Pattern::Id(id)),
                    _ => unreachable!(),
                }
            }

            lexer::TokenKind::Underscore => {
                self.advance()?;
                Ok(ast::Pattern::Wildcard(self.end_recording(index)))
            }

            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "pattern".to_string(),
                    found: self.peek().span.clone(),
                },
                self.peek().span.clone(),
            )),
        }
    }

    fn parse_type(&mut self) -> ParserResult<ast::Type> {
        let index = self.start_recording();
        let mut lhs = self.parse_type_primary()?;
        let mut rhs = Vec::new();
        while self.match_token(lexer::TokenKind::Arrow)? {
            rhs.push(self.parse_type()?);
            lhs = ast::Type::Func(Box::new(lhs), rhs.clone(), self.end_recording(index));
        }

        Ok(lhs)
    }

    fn parse_type_primary(&mut self) -> ParserResult<ast::Type> {
        let index = self.start_recording();
        match self.peek().kind {
            n if n.is_identifier() => {
                let id = self.expect_any_identifier()?;
                let mut cloned = self.clone();
                let mut ty = self.parse_type();
                if ty.is_ok() {
                    let mut types = vec![];
                    while ty.is_ok() {
                        types.push(ty.unwrap());
                        ty = self.parse_type();
                        cloned = self.clone();
                    }
                    *self = cloned;
                    return Ok(ast::Type::App(id, types, self.end_recording(index)));
                } else {
                    *self = cloned;
                }

                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Type::Generic(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Type::Id(id)),
                    _ => unreachable!(),
                }
            }

            lexer::TokenKind::LBracket => {
                self.advance()?;
                let ty = self.parse_type()?;
                self.expect_current(token![rbracket])?;

                Ok(ty)
            }

            lexer::TokenKind::LParen => {
                self.advance()?;
                let mut tys = Vec::new();
                let ty = self.parse_type()?;
                tys.push(ty);
                while !self.match_token(lexer::TokenKind::RParen)? {
                    self.expect_current(token![,])?;
                    let ty = self.parse_type()?;
                    tys.push(ty);
                }

                Ok(ast::Type::Tuple(tys, self.end_recording(index)))
            }

            _ => Err(error::Error::new(
                error::ErrorKind::UnexpectedToken {
                    expected: "type".to_string(),
                    found: self.peek().span.clone(),
                },
                self.peek().span.clone(),
            )),
        }
    }
}
