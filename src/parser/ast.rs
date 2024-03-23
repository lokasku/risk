use pest::iterators::{Pair, Pairs};
use crate::parser::Rule;

#[derive(Debug, PartialEq)]
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
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span) -> Self {
        Span {
            start: span.start(),
            end: span.end(),
            input: span.as_str().to_string()
        }
    }
}

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

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let span = Span::from(pair.clone().as_span());
        let rule = pair.as_rule().clone();
        let mut inner = pair.clone().into_inner();
        match rule {
            Rule::annotation => {
                let expr = Expr::from_pair(inner.next().unwrap());
                let ty = Type::from_pair(inner.next().unwrap());
                Expr::Ann(Box::new(expr), ty, span)
            },
            Rule::let_ => {
                let mut binds = Vec::new();
                let mut inner = inner.clone();
                let mut expr = None;
                while let Some(ref e) = inner.next() {
                    if e.as_rule() != Rule::bind {
                        expr = Some(Expr::from_pair(e.clone()));
                        continue;
                    }
                    
                    let bind = Bind::from_pair(e.clone());
                    binds.push(bind);
                }
                
                
                Expr::Let(binds, Box::new(expr.unwrap()), span)
            }
            Rule::condition => {
                let cond = Expr::from_pair(inner.next().unwrap());
                let then = Expr::from_pair(inner.next().unwrap());
                let else_ = Expr::from_pair(inner.next().unwrap());
                Expr::Condition(Box::new(cond), Box::new(then), Box::new(else_), span)
            },
            Rule::lambda => {
                let mut patterns = Vec::new();
                for e in inner.next().unwrap().into_inner() {
                    let pattern = Pattern::from_pair(e);
                    patterns.push(pattern);
                }
                let expr = Expr::from_pair(inner.next().unwrap());
                Expr::Lambda(patterns, Box::new(expr), span)
            },
            Rule::match_ => {
                let expr = Expr::from_pair(inner.next().unwrap());
                let mut patterns = Vec::new();
                for e in inner.next().unwrap().into_inner() {
                    let mut inner = e.into_inner();
                    let pattern = Pattern::from_pair(inner.next().unwrap());
                    let expr = Expr::from_pair(inner.next().unwrap());
                    patterns.push((Box::new(pattern), Box::new(expr)));
                }
                Expr::Match(Box::new(expr), patterns, span)
            },
            Rule::app_expr => {
                let app = App::from_pair(inner.next().unwrap());
                Expr::App(app)
            },
            Rule::literals => {
                let lit = Literal::from_pair(inner.next().unwrap());
                Expr::Literal(lit)
            },
            Rule::pc_id => {
                let id = Identifier::from_pair(inner.next().unwrap());
                Expr::Identifier(id)
            },
            Rule::ident => {
                let id = Identifier::from_pair(pair.clone());
                Expr::Id(id)
            },
            Rule::list_expr => {
                let mut exprs = Vec::new();
                for e in inner.next().unwrap().into_inner() {
                    let expr = Expr::from_pair(e);
                    exprs.push(expr);
                }
                Expr::List(exprs, span)
            },
            Rule::tuple_expr => {
                let mut exprs = Vec::new();
                for e in inner.next().unwrap().into_inner() {
                    let expr = Expr::from_pair(e);
                    exprs.push(expr);
                }
                Expr::Tuple(exprs, span)
            },
            Rule::binop => {
                if inner.len() == 1 {
                    return Expr::from_pair(inner.next().unwrap());
                }
                let lhs = Expr::from_pair(inner.next().unwrap());
                let op = match inner.next().unwrap().as_str() {
                    "+" => BinOp::Add,
                    "-" => BinOp::Sub,
                    e => panic!("Invalid operator: {:?}", e)
                };
                
                let rhs = Expr::from_pair(inner.next().unwrap());
                Expr::BinOp(op, Box::new(lhs), Box::new(rhs), span)
            },
            Rule::factor => {
                if inner.len() == 1 {
                    return Expr::from_pair(inner.next().unwrap());
                }
                let lhs = Expr::from_pair(inner.next().unwrap());
                let op = match inner.next().unwrap().as_str() {
                    "*" => BinOp::Mul,
                    "/" => BinOp::Div,
                    "%" => BinOp::Mod,
                    e => panic!("Invalid operator: {:?}", e)
                };
                
                let rhs = Expr::from_pair(inner.next().unwrap());
                
                Expr::BinOp(op, Box::new(lhs), Box::new(rhs), span)
            },
            Rule::cmpop => {
                if inner.len() == 1 {
                    return Expr::from_pair(inner.next().unwrap());
                }
                let lhs = Expr::from_pair(inner.next().unwrap());
                let op = match inner.next().unwrap().as_str() {
                    "<" => BinOp::LessThan,
                    ">" => BinOp::GreaterThan,
                    "<=" => BinOp::LessThanOrEq,
                    ">=" => BinOp::GreaterThanOrEq,
                    "==" => BinOp::Eq,
                    "!=" => BinOp::Ineq,
                    e => panic!("Invalid operator: {:?}", e)
                };
                
                let rhs = Expr::from_pair(inner.next().unwrap());
                
                Expr::BinOp(op, Box::new(lhs), Box::new(rhs), span)
            },
            e => panic!("Invalid expression: {:?}", e)
        }

    }
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


#[derive(Debug, PartialEq)]
pub struct App {
    pub expr: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span

}

impl App {
    pub fn new(expr: Expr, args: Vec<Expr>, span: Span) -> Self {
        App {
            expr: Box::new(expr),
            args,
            span
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut inner = pair.clone().into_inner();
        let expr = Expr::from_pair(inner.next().unwrap());
        let mut args = Vec::new();
        for e in inner {
            let arg = Expr::from_pair(e);
            args.push(arg);
        }
        App::new(expr, args, Span::from(pair.as_span()))
    }
}

////////// Type Assignment 

#[derive(Debug, PartialEq)]
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

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut inner = pair.clone().into_inner();
        let span = Span::from(pair.as_span());
        let id = Identifier::from_pair(inner.next().unwrap());
        let ty = Type::from_pair(inner.next().unwrap());
        TypeAssign::new(id, ty, span)
    }
}

////////// Bind

#[derive(Debug, PartialEq)]
pub struct Bind {
    pub id: Identifier,
    pub expr: Expr,
    pub span: Span
}

impl Bind {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let span = Span::from(pair.as_span());
        let mut inner = pair.into_inner();
        let id = Identifier::from_pair(inner.next().unwrap());
        let expr = Expr::from_pair(inner.next().unwrap());
        Bind {
            id,
            expr,
            span
        }
    }
}

////////// Identifier

#[derive(Debug, PartialEq)]
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
    
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let span = Span::from(pair.as_span());
        let name = pair.as_str().to_string();
        Identifier::new(name, span)
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

////////// Type Declaration

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
    
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
        let id_pair = inner.next().unwrap();
        let id = Identifier::from_pair(id_pair);
        let mut types = Vec::new();
        for e in inner {
            let ty = Type::from_pair(e);
            types.push(ty);
        }
        Variant::new(id, types)
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
    
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
        let id = inner.next().unwrap();
        let id = Identifier::from_pair(id);
        let mut typevars = Vec::new();
        let mut variants = Vec::new();
        for e in inner {
            match e.as_rule() {
                Rule::ident => {
                    let typevar = Identifier::from_pair(e);
                    typevars.push(typevar);
                },
                Rule::variant => {
                    let variant = Variant::from_pair(e);
                    variants.push(variant);
                },
                _ => panic!("Invalid type declaration")
            }
        }
        
        TypeDecl::new(id, typevars, variants)
        
    }
}


////////// Type

#[derive(Debug, PartialEq)]
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

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::ident => {
                let id = Identifier::from_pair(pair);
                Type::Id(id)
            },
            Rule::function_type => {
                let mut inner = pair.clone().into_inner();
                let id = Type::from_pair(inner.next().unwrap());
                let mut types = Vec::new();
                for e in inner {
                    let ty = Type::from_pair(e);
                    types.push(ty);
                }
                Type::Func(Box::new(id), types, Span::from(pair.as_span()))
            },
            Rule::app_type => {
                let mut inner = pair.clone().into_inner();
                let id = Identifier::from_pair(inner.next().unwrap());
                let mut types = Vec::new();
                for e in inner {
                    let ty = Type::from_pair(e);
                    types.push(ty);
                }
                Type::App(id, types, Span::from(pair.as_span()))
            },
            Rule::group_type => {
                Type::from_pair(pair.into_inner().next().unwrap())
            },
            Rule::tuple_type => {
                let mut types = Vec::new();
                for e in pair.clone().into_inner() {
                    let ty = Type::from_pair(e);
                    types.push(ty);
                }
                Type::Tuple(types, Span::from(pair.as_span()))
            },

            e => panic!("Invalid type: {:?}", e)
        }
    }
}

////////// Pattern

#[derive(Debug, PartialEq)]
pub enum Pattern {
    ListCons(Box<Pattern>, Box<Pattern>, Span),
    Wildcard(Span),
    Variable(Identifier),
    Id(Identifier),
    App(Box<Pattern>, Vec<Type>, Span),
    Literal(Literal)
}

impl Pattern {

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let span = Span::from(pair.as_span());
        match pair.as_rule() {
            Rule::ident => {
                let id = Identifier::from_pair(pair);
                Pattern::Id(id)
            },
            Rule::pc_id => {
                let id = Identifier::from_pair(pair);
                Pattern::Variable(id)
            },
            Rule::literals => {
                let lit = Literal::from_pair(pair);
                Pattern::Literal(lit)
            },
            Rule::list_cons_pattern => {
                let mut inner = pair.into_inner();
                let head = Pattern::from_pair(inner.next().unwrap());
                let tail = Pattern::from_pair(inner.next().unwrap());
                Pattern::ListCons(Box::new(head), Box::new(tail), span)
            },

            Rule::wildcard => {
                Pattern::Wildcard(span)
            },

            Rule::app_pattern => {
                let mut inner = pair.into_inner();
                let head = Pattern::from_pair(inner.next().unwrap());
                let mut types = Vec::new();
                for e in inner {
                    let ty = Type::from_pair(e);
                    types.push(ty);
                }
                Pattern::App(Box::new(head), types, span)
            },

            e => panic!("Invalid pattern: {:?}", e)
        }
    }
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

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(Bool)
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub lit: LiteralKind,
    pub span: Span
}

impl Literal {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let span = Span::from(pair.as_span());
        let rule = pair.as_rule();
        let lit = match rule {
            Rule::int => {
                let int = pair.as_str().parse::<i64>().unwrap();
                LiteralKind::Integer(int)
            },
            Rule::float => {
                let float = pair.as_str().parse::<f64>().unwrap();
                LiteralKind::Float(float)
            },
            Rule::string => {
                let string = pair.as_str().to_string();
                LiteralKind::String(string)
            },
            Rule::character => {
                let char_ = pair.as_str().chars().next().unwrap();
                LiteralKind::Char(char_)
            },
            Rule::true_ => {
                LiteralKind::Bool(Bool::True)
            },
            Rule::false_ => {
                LiteralKind::Bool(Bool::False)
            },
            e => panic!("Invalid literal: {:?}", e)
        };
        Literal {
            lit,
            span
        }

    }
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
        let mut programs = Vec::new();
        let program = pairs.clone().next().unwrap();
        for pair in program.into_inner() {
            let rule = pair.as_rule();
            match rule {
                Rule::type_decl => {
                    let ty = TypeDecl::from_pair(pair);
                    programs.push(Statment::TypeDecl(ty));
                },
                Rule::bind => {
                    let bind = Bind::from_pair(pair);
                    programs.push(Statment::Bind(bind));
                },
                Rule::type_assign => {
                    let ty = TypeAssign::from_pair(pair);
                    programs.push(Statment::TypeAssign(ty));
                },
                e => panic!("Invalid program: {:?}", e)
            }
            
        }
        Program::new(programs)
    }
}