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

use ariadne::{Label, Report, ReportKind, Source};
use crate::ast::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    UnexpectedToken { expected: String, found: Span },
    UnexpectedEOF { expected: String },
    UnexpectedEndOfInput,
    UnexpectedTokenInPattern { found: Span },
    TooMuchExpr { found: Span }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span
}

impl Error {
    pub fn new(kind: ErrorKind, span: Span) -> Error {
        Error { kind, span }
    }

    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();
        let mut report = Report::build(ReportKind::Error, filename, self.span.get_line_number(source));
        match &self.kind {
            ErrorKind::UnexpectedToken { expected, found } => {
                report = report.with_code("unexpected-token")
                    .with_message(format!("Unexpected token found at line {}",
                                          found.get_line_number(source)))
                    .with_label(Label::new(
                            (filename, found.start..found.end)
                        )
                        .with_message(format!("Expected {}", expected))
                    )
                ;

            },
            ErrorKind::UnexpectedEOF { expected } => {
                report = report.with_code("unexpected-eof")
                    .with_message("Unexpected end of input")
                    .with_label(Label::new(
                            (filename, self.span.start..self.span.end)
                        )
                        .with_message(format!("Expected {}", expected))
                    )
                ;
            },
            ErrorKind::UnexpectedEndOfInput => {
                report = report.with_code("unexpected-end-of-input")
                    .with_message("Unexpected end of input")
                    .with_label(Label::new(
                            (filename, self.span.start..self.span.end)
                        )
                        .with_message("Expected more input")
                    )
                ;
            },
            ErrorKind::UnexpectedTokenInPattern { found } => {
                report = report.with_code("unexpected-token-in-pattern")
                    .with_message(format!("Unexpected token found in pattern at line {}",
                                          found.get_line_number(source)))
                    .with_label(Label::new(
                            (filename, found.start..found.end)
                        )
                        .with_message("Unexpected token in pattern")
                    )
                ;
            },
            ErrorKind::TooMuchExpr { found } => {
                report = report.with_code("too-much-expr")
                    .with_message(format!("Too much expr found at line {}",
                                          found.get_line_number(source)))
                    .with_label(Label::new(
                            (filename, found.start..found.end)
                        )
                        .with_message("Too much expr")
                    )
                ;
            }
        }
        
        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();
    }
}