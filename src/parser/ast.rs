#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(Identifier),
    Condition(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Bind>, Box<Expr>),
    Match(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>),
    Literal(Literal),
    Builtin(Builtin),
    Lambda(Vec<Types>, Box<Expr>),
    List(Vec<Expr>),
    Tuple(Vec<Expr>)
}

#[derive(Debug, PartialEq)]
pub struct Bind(Identifier, Expr);

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Plus, 
    Minus,
    Div,
    Mod,
    Exp, // ^
    LT, // <
    GT, // >
    GE, // >=
    LE, // <=
    Eq, // ==
    Ineq, // /=
    And, // &&
    Or, // ||
    Pipe,
    Colon,
    Lambda // \
}

#[derive(Debug, PartialEq)]
pub struct Type { // data X a b = A a | B b
    name: String, // X
    typevars: Vec<Identifier>, // [a, b]
    variants: Vec<Variant> // [[A, [a]], [B, [b]]]
}

#[derive(Debug, PartialEq)]
pub struct Variant(String, Vec<Types>);

#[derive(Debug, PartialEq)]
pub enum Types {
    Generic(Identifier), // Generic(Integer("a"))
    Id(Identifier), // Id(Identifier("Integer"))

    /// App(
    ///   Id("Maybe"),
    ///   [
    ///     Generic("a"),
    ///     Id("Integer")
    ///   ]
    /// )
    App(Box<Types>, Vec<Types>)
}

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