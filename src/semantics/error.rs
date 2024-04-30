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

use crate::semantics::Span;
use ariadne::*;

#[derive(Debug)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    MultipleDeclarations {
        name: String,
        span: Span,
    },
    TypeAlreadyDefined {
        type_name: String,
    },
    ReservedName {
        name: String,
        span: Span,
    },
    WrongArity {
        expected: usize,
        found: usize,
        span: Span,
    },
    AlreadyTypedSymbol {
        symbol_name: String,
    },
    UndefinedSymbol {
        symbol_name: String,
        span: Span,
    },
    UndefinedType {
        type_name: String,
        span: Span,
    },
    NotACallee {
        name: String,
        span: Span,
    },
    UndefinedConstructor {
        constructor_name: String,
        span: Span,
    },
}

impl SemanticError {
    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();
        let mut report = Report::build(
            ReportKind::Error,
            filename,
            self.span.get_line_number(source),
        );

        match &self.kind {
            SemanticErrorKind::MultipleDeclarations { name, span } => {
                report = report
                    .with_code("multiple-declarations")
                    .with_message(format!("Multiple declarations of '{}'", name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Already defined previously")
                            .with_color(Color::Cyan)
                    )
                    .with_note("A symbol that takes no arguments cannot be instantiated several times, as it is considered a constant.");
            }
            SemanticErrorKind::TypeAlreadyDefined { type_name } => {
                report = report
                    .with_code("type-already-defined")
                    .with_message(format!("Type '{}' is already defined", type_name))
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end))
                            .with_message("Type already defined")
                            .with_color(Color::Cyan),
                    );
            }
            SemanticErrorKind::ReservedName { name, span } => {
                report = report
                    .with_code("reserved-name")
                    .with_message(format!("'{}' is a reserved name", name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Reserved name")
                            .with_color(Color::Cyan),
                    )
                    .with_note("Reserved names cannot be used as identifiers.");
            }
            SemanticErrorKind::WrongArity {
                expected,
                found,
                span,
            } => {
                report = report
                    .with_code("wrong-arity")
                    .with_message(format!("Expected {} arguments, found {}", expected, found))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message(format!("Expected {} arguments", expected))
                            .with_color(Color::Cyan),
                    );
            }
            SemanticErrorKind::AlreadyTypedSymbol { symbol_name } => {
                report = report
                    .with_code("already-typed-symbol")
                    .with_message(format!("'{}' is already typed", symbol_name))
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end))
                            .with_message("Symbol already typed")
                            .with_color(Color::Cyan),
                    );
            }
            SemanticErrorKind::UndefinedSymbol { symbol_name, span } => {
                report = report
                    .with_code("undefined-symbol")
                    .with_message(format!("Undefined symbol '{}'", symbol_name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Symbol not found")
                            .with_color(Color::Cyan),
                    );
            }
            SemanticErrorKind::UndefinedType { type_name, span } => {
                report = report
                    .with_code("undefined-type")
                    .with_message(format!("Undefined type '{}'", type_name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Type not found")
                            .with_color(Color::Cyan),
                    );
            }
            SemanticErrorKind::NotACallee { name, span } => {
                report = report
                    .with_code("not-a-callee")
                    .with_message(format!("'{}' is not a callee", name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Not a callee")
                            .with_color(Color::Cyan)
                    )
                    .with_note("Callee are symbols that take arguments, such as constructors or function names.");
            }
            SemanticErrorKind::UndefinedConstructor {
                constructor_name,
                span,
            } => {
                report = report
                    .with_code("undefined-constructor")
                    .with_message(format!("Undefined constructor '{}'", constructor_name))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message("Constructor not found")
                            .with_color(Color::Cyan),
                    );
            }
        }

        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();
    }
}
