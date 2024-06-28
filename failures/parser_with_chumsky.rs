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

mod error;
mod lexer;

use crate::ast::*;
use chumsky::prelude::*;
use chumsky::text::*;
use error::Error;

pub fn parser<'a>() -> impl Parser<&'a str, Result<Vec<Expr>, Error>> {
    let integer = chumsky::text::int::<_, Simple<char>>(10).map_with_span(|lit, span| Literal {
        lit: LiteralKind::Integer(lit.parse().unwrap()),
        span: Span::new(span.start, span.end, lit),
    });

    let float = integer.then(just('.').then(text::digits(10).or_not())).map_with_span(|lit, span| Literal {
         
    });

    let string = just('"').ignore_then(none_of('"').repeated()).then_ignore(just('"')).map_with_span(|lit, span: Span| Literal {
        lit: LiteralKind::String(lit.iter().collect()),
        span: Span::new(span.start, span.end, lit.iter().collect()),
    });

    let boolean = just("True").or(just("False")).map_with_span(|lit, span: Span| Literal {
        lit: LiteralKind::Bool(if lit == "True" { Bool::True } else { Bool::False }),
        span: Span::new(span.start, span.end, lit.to_string()),
    });

    let literals = integer.or(float).or(string).or(boolean).map(Expr::Literal);
}
