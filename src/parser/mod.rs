mod ast;
pub mod lexer;
use ast::*;

use chumsky::prelude::*;

// repeated : *
// then     : ~

// pub fn parser() -> impl Parser<char, ast::Program, Error = Simple<char>> {
//     let int = text::int(10)
//         .map(|s: String| Literal::Integer(s.parse().unwrap()))
//         .padded();
//     let float = text::digits(10)
//         .then(|int_part: String| {
//             text::keyword::<_, _, Simple<char>(".").then(text::digits(10)).map(move |frac_part: String| {
//                 let s = format!("{}.{}", int_part, frac_part);
//                 Literal::Float(s.parse().unwrap())
//             })
//         })
//     .or_else(text::digits(10).map(|int_part: String| Literal::Float(int_part.parse().unwrap())))
//     .padded();    int.then_ignore(end())
// }