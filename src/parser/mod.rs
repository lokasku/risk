mod ast;

use chumsky::prelude::*;

// repeated : *
// then     : ~

pub fn parser() -> impl Parser<char, ast::Program, Error = Simple<char>> {
    todo!()
}