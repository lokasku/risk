#[derive(Debug, PartialEq)]
pub enum Statment {
    Bind(Bind),
    Type(Type),
    TypeAssign(TypeAssign)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(Identifier),
    Id(Identifier),
    App(Identifier, Vec<Expr>),
    Condition(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Bind>, Box<Expr>),
    Match(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>),
    Literal(Literal),
    Builtin(Builtin),
    Lambda(Vec<Pattern>, Box<Expr>),
    List(Vec<Expr>),
    Tuple(Vec<Expr>)
}

#[derive(Debug, PartialEq)]
pub struct TypeAssign(pub Identifier, pub Types);

#[derive(Debug, PartialEq)]
pub struct Bind(pub Identifier, pub Expr);

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
    Colon
}

#[derive(Debug, PartialEq)]
pub struct Type { // type X a b = A a | B b
    pub name: Identifier, // X
    pub typevars: Vec<Identifier>, // [a, b]
    pub variants: Vec<Variant> // [[A, [a]], [B, [b]]]
}

#[derive(Debug, PartialEq)]
pub struct Variant(pub Identifier, pub Vec<Types>);

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
    Arr(Vec<Types>, Box<Types>)
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>), // :
    Wildcard, // _
    Variable(Identifier), // x, xs
    Id(Identifier), // Integer, Maybe
    App(Box<Pattern>, Vec<Pattern>), // Either a (x:xs)
    Literal(Literal) // 2
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