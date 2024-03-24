////////// Span

// #[derive(Debug, PartialEq)]
// pub struct Span {
//     pub start: usize,
//     pub end: usize,
//     pub input: String
// }

// impl Span {
//     pub fn new(start: usize, end: usize, input: String) -> Self {
//         Span {
//             start,
//             end,
//             input
//         }
//     }
// }

////////// Program

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

////////// Statment

#[derive(Debug, PartialEq)]
pub enum Statment {
    Bind(Bind),
    TypeDecl(TypeDecl),
    TypeAssign(TypeAssign)
}

////////// Expression

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(Identifier), // e.g. foo
    Id(Identifier), // e.g.  Maybe
    App(App),
    Condition(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Bind>, Box<Expr>),
    Match(Box<Expr>, Vec<(Box<Pattern>, Box<Expr>)>),
    Literal(Literal),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Lambda(Vec<Pattern>, Box<Expr>),
    Ann(Box<Expr>, Type),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}


// impl Expr {
//     pub fn get_span(&self) -> &Span {
//         match self {
//             Expr::Identifier(id) => &id.span,
//             Expr::Id(id) => &id.span,
//             Expr::App(app) => &app.span,
//             Expr::Condition(_, _, _) => span,
//             Expr::Let(_, _) => span,
//             Expr::Match(_, _) => span,
//             Expr::Literal(lit) => &lit.span,
//             Expr::BinOp(_, _, _) => span,
//             Expr::Lambda(_, _) => span,
//             Expr::Ann(_, _) => span,
//             Expr::List(_) => span,
//             Expr::Tuple(_) => span
//         }
//     }
// }

////////// Application

#[derive(Debug, PartialEq)]
pub struct App {
    pub expr: Box<Expr>,
    pub args: Vec<Expr>,
    // pub span: Span

}

impl App {
    pub fn new(expr: Expr, args: Vec<Expr>/*, span: Span*/) -> Self {
        App {
            expr: Box::new(expr),
            args,
            // span
        }
    }
}

////////// Type Assignment 

#[derive(Debug, PartialEq)]
pub struct TypeAssign {
    pub id: Identifier,
    pub ty: Type,
    // pub span: Span
}

impl TypeAssign {
    pub fn new(id: Identifier, ty: Type, /* span: Span */) -> Self {
        TypeAssign {
            id,
            ty,
            // span
        }
    }
}

////////// Bind

#[derive(Debug, PartialEq)]
pub struct Bind {
    pub id: Identifier,
    pub expr: Expr,
    // pub span: Span
}

////////// Identifier

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    // pub span: Span
}

impl Identifier {
    pub fn new(name: String, /* span : Span */) -> Self {
        Identifier {
            name,
            // span
        }
    }
}

////////// Binary Operator

#[derive(Debug, PartialEq)]
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

////////// Type Declaration & Variants

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub id: Identifier,
    pub types:  Vec<Type>
}

impl Variant {
    pub fn new(id: Identifier, types: Vec<Type>) -> Self {
        Variant {
            id,
            types
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeDecl { // type X a b = A a | B b
    pub name: Identifier, // X
    pub typevars: Vec<Identifier>, // [a, b]
    pub variants: Vec<Variant> // [[A, [a]], [B, [b]]]
}

impl TypeDecl {
    pub fn new(name: Identifier, typevars: Vec<Identifier>, variants: Vec<Variant>) -> Self {
        TypeDecl {
            name,
            typevars,
            variants
        }
    }
}

////////// Types

#[derive(Debug, PartialEq)]
pub enum Type {
    Generic(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Type>),
    Tuple(Vec<Type>),
    Func(Box<Type>, Vec<Type>)
}

// impl Type {
//     pub fn get_span(&self) -> &Span {
//         match self {
//             Type::Generic(id) => &id.span,
//             Type::Id(id) => &id.span,
//             Type::App(_, _) => span,
//             Type::Tuple(_) => span,
//             Type::Func(_, _) => span
//         }
//     }
// }

////////// Pattern

#[derive(Debug, PartialEq)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>),
    Wildcard,
    Variable(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Type>),
    Literal(Literal)
}

// impl Pattern {
//     pub fn get_span(&self) -> &Span {
//         match self {
//             Pattern::ListCons(_, _) => span,
//             Pattern::Wildcard(span) => span,
//             Pattern::Variable(id) => &id.span,
//             Pattern::Id(id) => &id.span,
//             Pattern::App(_, _) => span,
//             Pattern::Literal(lit) => &lit.span
//         }
//     }
// }

////////// Literal

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(Bool)
}

// #[derive(Debug, PartialEq)]
// pub struct Literal {
//     pub lit: LiteralKind,
//     // pub span: Span
// }

#[derive(Debug, PartialEq)]
pub enum Bool {
    True,
    False
}