`let` `in`
`if` `then` `else`
`match` `with` + `|`
`type` + `|`
`alias`

`+` `-` `/` `%` `<` `>` `>=` `<=` `,` `->` `=` `==` `^` `:` `\` `&&` `||` (``` ```) `_`
                                                             ^
                                                           lambda
`(` `)` `[` `]`

`Float` `Double`
`Integer`
`Char`
`Bool` : `True` and `False`
`(x, y)`
`[x, y]`
Indentifier as it's parametric polymorphism

If the name consists of letters, it is a function which requires backticks to be called infix => Prefix by default
If the name consists of symbols, it is an operator which requires parentheses to be called prefix. => Infix by default

(##) :: Integer -> Integer -> Integer
1. a ## b = a + b
2. (##) a b = a + b

foo :: Integer -> Integer -> Integer
1. foo a b = a + b
2. a `foo` b = a + b


// #[derive(Logos, Debug, PartialEq)]
// #[logos(error = Error)]
// #[logos(skip r"/[*]([^*]|([*][^/]))*[*]/")] /* ... */
// #[logos(skip r"\/\/.*")] //
// #[logos(skip r"[ \t\n\f]+")]
// pub enum TType {
//     // Keywords
//     #[token("let")]
//     Let,

//     #[token("in")]
//     In,

//     #[token("if")]
//     If,

//     #[token("then")]
//     Then,

//     #[token("else")]
//     Else,
    
//     #[token("match")]
//     Match,

//     #[token("with")]
//     With,

//     #[token("type")]
//     Type,

//     #[token("alias")]
//     Alias,

//     // Syntactic operators
//     #[token("(")]
//     LParen,

//     #[token(")")]
//     RParen,

//     #[token("[")]
//     LBracket,

//     #[token("]")]
//     RBracket,

//     #[token("->")]
//     Arrow,

//     #[token(",")]
//     Comma,

//     #[token("::")]
//     DoubleCollon,

//     #[token("=")]
//     Equal,

//     // Identifiers
//     #[regex(r"[^\w\s\(\)\[\]':,=]+")]
//     Symbol,
    
//     #[regex(r"[a-zA-Z_][a-zA-Z0-9_']*")]
//     Identifier,

//     // Primitives
//     #[regex(r"\d+", priority = 3)] // , |lex| lex.slice().parse().ok(), priority = 3
//     Integer,

//     #[regex(r"\d+(\.\d*)?")] // , |lex| lex.slice().parse::<f64>().ok()
//     Float,

//     #[regex("\"[^\"]+\"")] // , |lex| lex.slice()[1..lex.slice().len() - 1].to_owned()
//     String,

//     #[token("True")]
//     True,

//     #[token("False")]
//     False
// }

// pub enum Token<'a> {
//     Let,
//     In,
//     If,
//     Then,
//     Else,
//     Match,
//     With,
//     Type,
//     Alias,
//     LParen,
//     RParen,
//     LBracket,
//     RBracket,
//     Arrow,
//     Comma,
//     DoubleCollon,
//     Equal,
//     Symbol(&'a str),
//     Identifier(&'a str),
//     Integer(u64),
//     Float(f64),
//     String(&'a str),
//     True,
//     False
// }

// pub struct Lexer<'a> {
//     pub lexer: LLexer<'a, TType>,
//     pub errors: Vec<Error>
// }