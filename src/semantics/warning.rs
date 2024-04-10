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
pub struct SemanticWarning {
    pub kind: SemanticWarningKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum SemanticWarningKind {
    UnusedSymbol,
    UnusedType,
    UnusedVariant,
}

impl SemanticWarning {
    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();
        let mut report = Report::build(
            ReportKind::Warning,
            filename,
            self.span.get_line_number(source),
        );

        match &self.kind {
            SemanticWarningKind::UnusedSymbol => {
                report = report
                    .with_code("unused-symbol")
                    .with_message(format!("The '{}' symbol is never used.", self.span.input))
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end))
                            .with_message("Never used")
                            .with_color(Color::Cyan),
                    )
            }
            SemanticWarningKind::UnusedType => {
                report = report
                    .with_code("unused-type")
                    .with_message(format!("The '{}' type is never used.", self.span.input))
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end))
                            .with_message("Never used")
                            .with_color(Color::Cyan),
                    )
            }
            SemanticWarningKind::UnusedVariant => {
                report = report
                    .with_code("unused-variant")
                    .with_message(format!("The '{}' variant is never used.", self.span.input))
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end))
                            .with_message("Never used")
                            .with_color(Color::Cyan),
                    )
            }
        }

        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();
    }
}
