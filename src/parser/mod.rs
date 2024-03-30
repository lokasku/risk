use crate::{ast, token};
use crate::ast::{App, Bool, Literal, LiteralKind, TypeDecl};
use crate::parser::lexer::{Lexer, lexer};

pub mod lexer;
mod error;


type ParserResult<T> = Result<T, error::Error>;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    pub content: &'a str,
    pub tokens: Vec<lexer::Token<'a>>,
    pub current_span: ast::Span,
    pub current: usize
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Parser<'a> {
        Parser {
            content,
            tokens: lexer(content),
            current_span: ast::Span::new(0, 0, "".to_string()),
            current: 0
        }
    }

    fn advance(&mut self) -> Option<lexer::Token<'a>> {
        if self.is_eof() {
            return None;
        }
        self.current += 1;
        if self.is_eof() {
            return None;
        }
        let token = self.tokens[self.current].clone();
        self.current_span.end = token.span.end;
        self.current_span.input += &token.span.input;
        Some(token)
    }

    fn expect(&mut self, token: lexer::Token<'a>) -> ParserResult<Option<lexer::Token<'a>>> {
        let tok = self.advance();
        if self.peek().kind == token.kind {
            Ok(tok)
        } else {
            Err(
                error::Error::new(error::ErrorKind::UnexpectedToken {
                    expected: format!("{:?}", token.kind),
                    found: self.current_span.clone()
                }, self.current_span.clone())
            )
        }
    }

    fn match_token(&mut self, token: lexer::TokenKind) -> bool {
        if self.peek().kind == token {
            self.advance();
            true
        } else {
            false
        }
    }

    fn future(&self) -> lexer::Token<'a> {
        self.tokens[self.current + 1].clone()
    }
    fn past(&self) -> lexer::Token<'a> {
        self.tokens[self.current - 1].clone()
    }

    fn cut(&mut self) -> ast::Span {
        let span = self.current_span.clone();
        self.current_span = ast::Span::new(self.current_span.end, self.current_span.end, "".to_string());
        span
    }

    fn expect_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::Identifier(id) => {
                self.advance();
                let span = self.cut();
                Ok(ast::Identifier::new(id.to_string(), span))
            },
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "identifier".to_string(),
                found: peek.span.clone()
            }, peek.span))
        }
    }

    fn expect_pc_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::PCIdentifier(id) => {
                self.advance();
                let span = self.cut();
                Ok(ast::Identifier::new(id.to_string(), span))
            },
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "pascal case identifier".to_string(),
                found: peek.span.clone()
            }, peek.span))
        }
    }

    fn expect_any_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let peek = self.peek();
        match peek.kind {
            lexer::TokenKind::Identifier(id) => {
                self.advance();
                let span = self.cut();
                Ok(ast::Identifier::new(id.to_string(), span))
            },
            lexer::TokenKind::PCIdentifier(id) => {
                self.advance();
                let span = self.cut();
                Ok(ast::Identifier::new(id.to_string(), span))
            },
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "identifier".to_string(),
                found: peek.span.clone()
            }, peek.span))
        }
    }

    fn peek(&self) -> lexer::Token<'a> {
        self.tokens[self.current].clone()
    }

    fn is_operator(&self) -> bool {
        match self.peek().kind {
            lexer::TokenKind::Add | lexer::TokenKind::Sub | lexer::TokenKind::Mul | lexer::TokenKind::Div | lexer::TokenKind::Exp | lexer::TokenKind::Mod => true,
            _ => false
        }
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

    fn parse_statement(&mut self) -> ParserResult<ast::Statment> {
        match self.peek().kind {
            lexer::TokenKind::Identifier(_) => self.parse_stmt_identifier(),
            lexer::TokenKind::Type => self.parse_type_decl(),
            _ => panic!("Unexpected token: {:?}", self.peek())
        }
    }
    
    fn parse_stmt_identifier(&mut self) -> ParserResult<ast::Statment> {
        let id = self.expect_identifier()?;
        if self.peek().kind == lexer::TokenKind::Assign {
            self.advance();
            let expr = self.parse_expr()?;
            Ok(ast::Statment::Bind(ast::Bind::new(id, vec![], expr, self.cut())))
        } else if self.peek().kind == lexer::TokenKind::DoubleCollon {
            self.advance();
            let ty = self.parse_type()?;
            Ok(ast::Statment::TypeAssign(ast::TypeAssign::new(id, ty, self.cut())))
        } else {
            Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "assignment or type annotation".to_string(),
                found: self.peek().span.clone()
            }, self.peek().span.clone()))
        }
    }
    fn parse_type_decl(&mut self) -> ParserResult<ast::Statment> {
        self.advance();
        let id = self.expect_pc_identifier()?;
        let mut idents = Vec::new();
        while !self.match_token(lexer::TokenKind::Assign) {
            let id = self.expect_identifier()?;
            idents.push(id);
        }
        let mut variants = Vec::new();
        while self.match_token(lexer::TokenKind::Pipe) {
            let variant = self.parse_variant()?;
            variants.push(variant);
        }
        
        Ok(ast::Statment::TypeDecl(TypeDecl::new(id, idents, variants, self.cut())))
    }
    
    fn parse_variant(&mut self) -> ParserResult<ast::Variant> {
        let id = self.expect_pc_identifier()?;
        let mut ty = Vec::new();
        while !self.match_token(lexer::TokenKind::Semicolon) {
            let t = self.parse_type()?;
            ty.push(t);
        }
        Ok(ast::Variant::new(id, ty))
    }

    fn parse_bind(&mut self) -> ParserResult<ast::Bind> {
        let id = self.expect_identifier()?;
        let mut args = Vec::new();
        while !self.match_token(lexer::TokenKind::Assign) {
            let arg = self.parse_pattern()?;
            args.push(arg);
        }
        let expr = self.parse_expr()?;
        Ok(ast::Bind::new(id, args, expr, self.cut()))
    }

    fn parse_expr(&mut self) -> ParserResult<ast::Expr> {
        let mut lhs = self.parse_factor()?;
        while self.match_token(lexer::TokenKind::Add) || self.match_token(lexer::TokenKind::Sub) {
            let op = match self.past().kind {
                lexer::TokenKind::Add => ast::BinOp::Add,
                lexer::TokenKind::Sub => ast::BinOp::Sub,
                _ => unreachable!()
            };
            let rhs = self.parse_factor()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.cut());
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> ParserResult<ast::Expr> {
        let mut lhs = self.parse_cmp()?;

        while self.match_token(lexer::TokenKind::Mul) || self.match_token(lexer::TokenKind::Div) || self.match_token(lexer::TokenKind::Mod) {
            let op = match self.past().kind {
                lexer::TokenKind::Mul => ast::BinOp::Mul,
                lexer::TokenKind::Div => ast::BinOp::Div,
                lexer::TokenKind::Mod => ast::BinOp::Mod,
                _ => unreachable!()
            };
            let rhs = self.parse_cmp()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.cut());
        }

        Ok(lhs)

    }

    fn parse_cmp(&mut self) -> ParserResult<ast::Expr> {
        let mut lhs = self.parse_exp()?;

        while self.match_token(lexer::TokenKind::Eq) || self.match_token(lexer::TokenKind::Neq) {
            let op = match self.past().kind {
                lexer::TokenKind::Eq => ast::BinOp::Eq,
                lexer::TokenKind::Neq => ast::BinOp::Ineq,
                _ => unreachable!()
            };
            let rhs = self.parse_exp()?;
            lhs = ast::Expr::BinOp(op, Box::new(lhs), Box::new(rhs), self.cut());
        }

        Ok(lhs)
    }

    fn parse_exp(&mut self) -> ParserResult<ast::Expr> {
        let mut lhs = self.parse_annotation()?;

        while self.match_token(lexer::TokenKind::Exp) {
            let rhs = self.parse_annotation()?;
            lhs = ast::Expr::BinOp(ast::BinOp::Exp, Box::new(lhs), Box::new(rhs), self.cut());
        }

        Ok(lhs)
    }

    fn parse_annotation(&mut self) -> ParserResult<ast::Expr> {
        let mut lhs = self.parse_primary()?;
        if self.match_token(lexer::TokenKind::DoubleCollon) {
            let ty = self.parse_type()?;
            lhs = ast::Expr::Ann(Box::new(lhs), ty, self.cut());
        }

        Ok(lhs)
    }

    fn parse_literal(&mut self) -> ParserResult<Literal> {
        match self.peek().kind {
            lexer::TokenKind::Integer(i) => {
                self.advance();
                Ok(Literal::new(LiteralKind::Integer(i), self.cut()))
            },
            lexer::TokenKind::Float(f) => {
                self.advance();
                Ok(Literal::new(LiteralKind::Float(f), self.cut()))
            },
            lexer::TokenKind::String(s) => {
                self.advance();
                Ok(Literal::new(LiteralKind::String(s.to_string()), self.cut()))
            },
            lexer::TokenKind::True => {
                self.advance();
                Ok(Literal::new(LiteralKind::Bool(Bool::True), self.cut()))
            },
            lexer::TokenKind::False => {
                self.advance();
                Ok(Literal::new(LiteralKind::Bool(Bool::False), self.cut()))
            },
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "literal".to_string(),
                found: self.peek().span.clone()
            }, self.peek().span.clone()))
        }
    }



    fn parse_primary(&mut self) -> ParserResult<ast::Expr> {
        match self.peek().kind {
            lexer::TokenKind::Integer(i) => {
                self.advance();
                Ok(ast::Expr::Literal(Literal::new(LiteralKind::Integer(i), self.cut())))
            },
            lexer::TokenKind::Float(f) => {
                self.advance();
                Ok(ast::Expr::Literal(Literal::new(LiteralKind::Float(f), self.cut())))
            },
            lexer::TokenKind::String(s) => {
                self.advance();
                Ok(ast::Expr::Literal(Literal::new(LiteralKind::String(s.to_string()), self.cut())))
            },
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

                    return Ok(ast::Expr::App(App::new(id, exprs, self.cut())));
                } else {
                    *self = cloned;
                }


                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Expr::Identifier(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Expr::Id(id)),
                    _  => unreachable!()
                }
            },
            lexer::TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(token![rparen])?;
                Ok(expr)
            },
            lexer::TokenKind::Let => {
                self.advance();
                let mut binds = Vec::new();
                while !self.match_token(lexer::TokenKind::In) {
                    let bind = self.parse_bind()?;
                    self.expect(token![;])?;
                    binds.push(bind);
                }

                let expr = self.parse_expr()?;
                Ok(ast::Expr::Let(binds, Box::new(expr), self.cut()))
            },
            lexer::TokenKind::If => {
                self.advance();
                let cond = self.parse_expr()?;
                self.expect(token![then])?;
                let then = self.parse_expr()?;
                self.expect(token![else])?;
                let else_ = self.parse_expr()?;
                Ok(ast::Expr::Condition(Box::new(cond), Box::new(then), Box::new(else_), self.cut()))
            },
            lexer::TokenKind::Match => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(token![in])?;
                let mut arms = Vec::new();
                while self.match_token(lexer::TokenKind::Pipe) {
                    let pat = Box::new(self.parse_pattern()?);
                    self.expect(token![->])?;
                    let expr = Box::new(self.parse_expr()?);
                    arms.push((pat, expr));
                }
                Ok(ast::Expr::Match(Box::new(expr), arms, self.cut()))
            },

            lexer::TokenKind::DoubleSlash => {
                self.advance();
                let mut pats = Vec::new();

                while !self.match_token(lexer::TokenKind::Arrow) {
                    let pat = self.parse_pattern()?;
                    pats.push(pat);
                }

                let expr = self.parse_expr()?;
                Ok(ast::Expr::Lambda(pats, Box::new(expr), self.cut()))
            },


            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "primary expression".to_string(),
                found: self.peek().span.clone()
            }, self.peek().span.clone()))
        }
    }

    fn parse_pattern(&mut self) -> ParserResult<ast::Pattern> {
        let pat = self.parse_pattern_primary()?;
        if self.match_token(lexer::TokenKind::Colon) {
            let pat2 = self.parse_pattern()?;
            Ok(ast::Pattern::ListCons(Box::new(pat), Box::new(pat2), self.cut()))
        } else {
            Ok(pat)
        }
    }

    fn parse_pattern_primary(&mut self) -> ParserResult<ast::Pattern> {
        match self.peek().kind {
            n if n.is_literal() => {
                let lit = self.parse_literal()?;
                Ok(ast::Pattern::Literal(lit))
            },
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

                    return Ok(ast::Pattern::App(id, types, self.cut()));
                } else {
                    *self = cloned;
                }

                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Pattern::Variable(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Pattern::Id(id)),
                    _ => unreachable!()

                }
            },

            lexer::TokenKind::Underscore => {
                self.advance();
                Ok(ast::Pattern::Wildcard(self.cut()))
            },
            
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "pattern".to_string(),
                found: self.peek().span.clone()
            }, self.peek().span.clone()))


        }
    }

    fn parse_type(&mut self) -> ParserResult<ast::Type> {
        let mut lhs = self.parse_type_primary()?;
        let mut rhs = Vec::new();
        while self.match_token(lexer::TokenKind::Arrow) {
            rhs.push(self.parse_type_primary()?);
            lhs = ast::Type::Func(Box::new(lhs), rhs.clone(), self.cut());
        }
        
        Ok(lhs)
    }
    
    fn parse_type_primary(&mut self) -> ParserResult<ast::Type> {
        match self.peek().kind {
            n if n.is_identifier() => {
                let id = self.expect_any_identifier()?;
                let cloned = self.clone();
                let mut ty = self.parse_type();
                if ty.is_ok() {
                    let mut types = vec![];
                    while ty.is_ok() {
                        types.push(ty.unwrap());
                        ty = self.parse_type();
                    }

                    return Ok(ast::Type::App(id, types, self.cut()));
                } else {
                    *self = cloned;
                }

                match n {
                    lexer::TokenKind::Identifier(_) => Ok(ast::Type::Generic(id)),
                    lexer::TokenKind::PCIdentifier(_) => Ok(ast::Type::Id(id)),
                    _ => unreachable!()
                }
            },
            
            lexer::TokenKind::LBracket => {
                self.advance();
                let ty = self.parse_type()?;
                self.expect(token![rbracket])?;
                Ok(ty)
            },
            
            lexer::TokenKind::LParen => {
                self.advance();
                let mut tys = Vec::new();
                while !self.match_token(lexer::TokenKind::RParen) {
                    let ty = self.parse_type()?;
                    tys.push(ty);
                    self.expect(token![,])?;
                }
                
                Ok(ast::Type::Tuple(tys, self.cut()))
            },
            
            _ => Err(error::Error::new(error::ErrorKind::UnexpectedToken {
                expected: "type".to_string(),
                found: self.peek().span.clone()
            }, self.peek().span.clone()))
        }
    }


}