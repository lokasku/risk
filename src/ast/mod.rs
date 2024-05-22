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

use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub input: String,
}

impl Span {
    pub fn new(start: usize, end: usize, input: String) -> Self {
        Span { start, end, input }
    }

    pub fn get_line_number(&self, source: &str) -> usize {
        let mut line = 1;
        for c in source.chars().skip(self.start).take(self.end - self.start) {
            if c == '\n' {
                line += 1;
            }
        }
        line
    }
}

impl Hash for Span {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input.hash(state);
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input
    }
}

impl Eq for Span {}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program<T> {
    pub statements: Vec<Statement<T>>,
}

impl<T> Program<T> {
    pub fn new(statements: Vec<Statement<T>>) -> Self {
        Program { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<T> {
    Bind(Bind<T>),
    TypeDecl(TypeDecl),
    TypeAssign(TypeAssign),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AnnExpr<Annot = ()> {
    Identifier {
        id: Identifier,
    },
    PCIdentifier {
        id: Identifier,
    },
    App(App<Annot>),
    Condition {
        cond: Box<AnnExpr<Annot>>,
        then: Box<AnnExpr<Annot>>,
        els: Box<AnnExpr<Annot>>,
        ann: Annot,
    },
    Let {
        binds: Vec<Bind<Annot>>,
        ret: Box<AnnExpr<Annot>>,
        ann: Annot,
    },
    Match {
        referral: Box<AnnExpr<Annot>>,
        cases: Vec<(Pattern, Box<AnnExpr<Annot>>)>,
        ann: Annot,
    },
    Literal(Literal),
    BinOp {
        op: BinOp,
        lhs: Box<AnnExpr<Annot>>,
        rhs: Box<AnnExpr<Annot>>,
        ann: Annot,
    },
    Lambda {
        args: Vec<Pattern>,
        ret: Box<AnnExpr<Annot>>,
        ann: Annot,
    },
    Ann {
        expr: Box<AnnExpr<Annot>>,
        ann: (Span, Type)
    },
    List {
        list: Vec<AnnExpr<Annot>>,
        ann: Annot,
    },
    Tuple {
        list: Vec<AnnExpr<Annot>>,
        ann: Annot,
    },
}

pub type ParsedExpr = AnnExpr<Span>;

impl ParsedExpr {
    pub fn get_span(&self) -> &Span {
        match self {
            AnnExpr::Identifier { id } => &id.span,
            AnnExpr::PCIdentifier { id } => &id.span,
            AnnExpr::App(app) => &app.span,
            AnnExpr::Condition { ann, .. } => ann,
            AnnExpr::Let { ann, .. } => ann,
            AnnExpr::Match { ann, .. } => ann,
            AnnExpr::Literal(lit) => &lit.span,
            AnnExpr::BinOp { ann, .. } => ann,
            AnnExpr::Lambda { ann, .. } => ann,
            AnnExpr::Ann { ann, .. } => &ann.0,
            AnnExpr::List { ann, .. } => ann,
            AnnExpr::Tuple { ann, .. } => ann,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct App<T> {
    pub ident: Identifier,
    pub args: Vec<AnnExpr<T>>,
    pub span: Span,
}

impl<T> App<T> {
    pub fn new(ident: Identifier, args: Vec<AnnExpr<T>>, span: Span) -> Self {
        App { ident, args, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAssign {
    pub id: Identifier,
    pub ty: Type,
    pub span: Span,
}

impl TypeAssign {
    pub fn new(id: Identifier, ty: Type, span: Span) -> Self {
        TypeAssign { id, ty, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bind<T> {
    pub name: Identifier,
    pub args: Vec<Pattern>,
    pub expr: AnnExpr<T>,
    pub span: Span,
}

impl<T> Bind<T> {
    pub fn new(id: Identifier, args: Vec<Pattern>, expr: AnnExpr<T>, span: Span) -> Self {
        Bind {
            name: id,
            args,
            expr,
            span,
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Identifier { name, span }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Exp,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
    Eq,
    Ineq,
    And,
    Or,
    ListCons,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variant {
    pub id: Identifier,
    pub types: Vec<Type>,
    pub span: Span,
}

impl Variant {
    pub fn new(id: Identifier, types: Vec<Type>, span: Span) -> Self {
        Variant { id, types, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDecl {
    pub name: Identifier,
    pub typevars: Vec<Identifier>,
    pub variants: Vec<Variant>,
    pub span: Span,
}

impl TypeDecl {
    pub fn new(
        name: Identifier,
        typevars: Vec<Identifier>,
        variants: Vec<Variant>,
        span: Span,
    ) -> Self {
        TypeDecl {
            name,
            typevars,
            variants,
            span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Generic(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Type>, Span),
    Tuple(Vec<Type>, Span),
    Func(Box<Type>, Vec<Type>, Span),
}

impl Type {
    pub fn get_span(&self) -> &Span {
        match self {
            Type::Generic(id) => &id.span,
            Type::Id(id) => &id.span,
            Type::App(_, _, span) => span,
            Type::Tuple(_, span) => span,
            Type::Func(_, _, span) => span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>, Span),
    Wildcard(Span),
    Variable(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Pattern>, Span),
    Literal(Literal),
}

impl Pattern {
    pub fn get_span(&self) -> &Span {
        match self {
            Pattern::ListCons(_, _, span) => span,
            Pattern::Wildcard(span) => span,
            Pattern::Variable(id) => &id.span,
            Pattern::Id(id) => &id.span,
            Pattern::App(_, _, span) => span,
            Pattern::Literal(lit) => &lit.span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(Bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub lit: LiteralKind,
    pub span: Span,
}

impl Literal {
    pub fn new(lit: LiteralKind, span: Span) -> Self {
        Literal { lit, span }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bool {
    True,
    False,
}


impl Into<bool> for Bool {
    fn into(self) -> bool {
        self == Bool::True
    }
}
