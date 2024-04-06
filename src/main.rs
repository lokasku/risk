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

use semantics::analyze;
use semantics::AnalysisOutput;
use std::env;
use std::fs;

mod ast;
mod parser;
mod semantics;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    let mut parser = parser::Parser::new(&content);

    let ast = parser.parse();

    if let Err(e) = ast {
        e.report(&args[1]);
    } else {
        let mut semantics_analyzer = AnalysisOutput::new();

        analyze(&mut semantics_analyzer, ast.unwrap());

        println!("{:#?}", semantics_analyzer.errors);
    }
}
