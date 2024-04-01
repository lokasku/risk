use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub input: String
}

impl Span {
    pub fn new(start: usize, end: usize, input: String) -> Self {
        Span {
            start,
            end,
            input
        }
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

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}){}..{}", self.input, self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statment {
    Bind(Bind),
    TypeDecl(TypeDecl),
    TypeAssign(TypeAssign)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Identifier(Identifier), // e.g. foo
    Id(Identifier), // e.g.  Maybe
    App(App),
    Condition(Box<Expr>, Box<Expr>, Box<Expr>, Span),
    Let(Vec<Bind>, Box<Expr>, Span),
    Match(Box<Expr>, Vec<(Box<Pattern>, Box<Expr>)>, Span),
    Literal(Literal),
    BinOp(BinOp, Box<Expr>, Box<Expr>, Span),
    Lambda(Vec<Pattern>, Box<Expr>, Span),
    Ann(Box<Expr>, Type, Span),
    List(Vec<Expr>, Span),
    Tuple(Vec<Expr>, Span),
}


impl Expr {
    pub fn get_span(&self) -> &Span {
        match self {
            Expr::Identifier(id) => &id.span,
            Expr::Id(id) => &id.span,
            Expr::App(app) => &app.span,
            Expr::Condition(_, _, _, span) => span,
            Expr::Let(_, _, span) => span,
            Expr::Match(_, _, span) => span,
            Expr::Literal(lit) => &lit.span,
            Expr::BinOp(_, _, _, span) => span,
            Expr::Lambda(_, _, span) => span,
            Expr::Ann(_, _, span) => span,
            Expr::List(_, span) => span,
            Expr::Tuple(_, span) => span
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub ident: Identifier,
    pub args: Vec<Expr>,
    pub span: Span

}

impl App {
    pub fn new(ident: Identifier, args: Vec<Expr>, span: Span) -> Self {
        App {
            ident,
            args,
            span
        }
    }
}

////////// Type Assignment

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAssign {
    pub id: Identifier,
    pub ty: Type,
    pub span: Span
}

impl TypeAssign {
    pub fn new(id: Identifier, ty: Type, span: Span) -> Self {
        TypeAssign {
            id,
            ty,
            span
        }
    }
}

////////// Bind

#[derive(Debug, PartialEq, Clone)]
pub struct Bind {
    pub id: Identifier,
    pub args: Vec<Pattern>,
    pub expr: Expr,
    pub span: Span
}

impl Bind {
    pub fn new(id: Identifier, args: Vec<Pattern>, expr: Expr, span: Span) -> Self {
        Bind {
            id,
            args,
            expr,
            span
        }
    }
}


////////// Identifier

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Identifier {
            name,
            span
        }
    }
}

////////// Binary Operator

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
    ListCons
}

////////// Type Declaration

#[derive(Debug, PartialEq, Clone)]
pub struct Variant {
    pub id: Identifier,
    pub types:  Vec<Type>,
    pub span: Span
}

impl Variant {
    pub fn new(id: Identifier, types: Vec<Type>, span: Span) -> Self {
        Variant {
            id,
            types,
            span
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDecl { // type X a b = A a | B b
    pub name: Identifier, // X
    pub typevars: Vec<Identifier>, // [a, b]
    pub variants: Vec<Variant>, // [[A, [a]], [B, [b]]]
    pub span: Span
}

impl TypeDecl {
    pub fn new(name: Identifier, typevars: Vec<Identifier>, variants: Vec<Variant>, span: Span) -> Self {
        TypeDecl {
            name,
            typevars,
            variants,
            span
        }
    }
}

////////// Type

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Generic(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Type>, Span),
    Tuple(Vec<Type>, Span),
    Func(Box<Type>, Vec<Type>, Span)
}

impl Type {
    pub fn get_span(&self) -> &Span {
        match self {
            Type::Generic(id) => &id.span,
            Type::Id(id) => &id.span,
            Type::App(_, _, span) => span,
            Type::Tuple(_, span) => span,
            Type::Func(_, _, span) => span
        }
    }
}

////////// Pattern

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>, Span),
    Wildcard(Span),
    Variable(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Type>, Span),
    Literal(Literal)
}

impl Pattern {
    pub fn get_span(&self) -> &Span {
        match self {
            Pattern::ListCons(_, _, span) => span,
            Pattern::Wildcard(span) => span,
            Pattern::Variable(id) => &id.span,
            Pattern::Id(id) => &id.span,
            Pattern::App(_, _, span) => span,
            Pattern::Literal(lit) => &lit.span
        }
    }
}

////////// Literal

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(Bool)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub lit: LiteralKind,
    pub span: Span
}

impl Literal {
    pub fn new(lit: LiteralKind, span: Span) -> Self {
        Literal {
            lit,
            span
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bool {
    True,
    False
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statment>
}

impl Program {
    pub fn new(statements: Vec<Statment>) -> Self {
        Program {
            statements
        }
    }
}