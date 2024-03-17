use pest::iterators::Pairs;
use crate::parser::Rule;

#[derive(Debug, PartialEq)]
pub enum Statment {
    Bind(Bind),
    TypeDecl(TypeDecl),
    TypeAssign(TypeAssign)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(Identifier), // e.g. foo
    Id(Identifier), // e.g.  Maybe
    App(Box<Expr>, Vec<Expr>),
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

////////// Type Assignment 

#[derive(Debug, PartialEq)]
pub struct TypeAssign(pub Identifier, pub Type);

////////// Bind

#[derive(Debug, PartialEq)]
pub struct Bind(pub Identifier, pub Expr);

////////// Identifier

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

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

////////// Type Declaration

#[derive(Debug, PartialEq)]
pub struct Variant(pub Identifier, pub Vec<Type>);

#[derive(Debug, PartialEq)]
pub struct TypeDecl { // type X a b = A a | B b
    pub name: Identifier, // X
    pub typevars: Vec<Identifier>, // [a, b]
    pub variants: Vec<Variant> // [[A, [a]], [B, [b]]]
}

////////// Type

#[derive(Debug, PartialEq)]
pub enum Type {
    Generic(Identifier),
    Id(Identifier),
    App(Box<Type>, Vec<Type>),
    Tuple(Vec<Type>)
}

////////// Pattern

#[derive(Debug, PartialEq)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>),
    Wildcard,
    Variable(Identifier),
    Id(Identifier),
    App(Box<Pattern>, Vec<Type>),
    Literal(Literal)
}

////////// Literal

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(Bool)
}

#[derive(Debug, PartialEq)]
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
    
    pub fn from_pairs(pairs: Pairs<Rule>) -> Self {
        todo!()
    }
}