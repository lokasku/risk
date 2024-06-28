use ariadne::*;
use crate::ast::{Type, Span};
use super::ast::Annot;

#[derive(Debug)]
pub enum TypeCheckerErrorKind {
    MismatchedTypes {
        expected: Annot,
        found: Annot,
    },
    NonExhaustiveMatch {
        span: Span,
    },
    InfiniteType {
        span: Span,
    },
    AmbiguousType {
        span: Span,
    },
    IncompatibleTypes {
        expected: Annot,
        found: Annot,
        span: Span,
    },
    UnificationError {
        span: Span,
    },
}

#[derive(Debug)]
pub struct TypeCheckerError {
    pub kind: TypeCheckerErrorKind,
    pub span: Span,
}

impl TypeCheckerError {
    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();
        let mut report = Report::build(
            ReportKind::Error,
            filename,
            self.span.get_line_number(source),
        );

        match &self.kind {
            TypeCheckerErrorKind::MismatchedTypes { expected, found } => {
                report = report.with_message(format!(
                    "Mismatched types. Expected `{:?}`, found `{:?}`",
                    expected.1, found.1
                )).with_label(
                    Label::new((filename, expected.0.start..expected.0.end))
                        .with_message("Expected type here")
                );
            }
            TypeCheckerErrorKind::NonExhaustiveMatch { span } => {
                report = report.with_message("Non-exhaustive pattern match");
            }
            TypeCheckerErrorKind::InfiniteType { span } => {
                report = report.with_message("Infinite type");
            }
            TypeCheckerErrorKind::AmbiguousType { span } => {
                report = report.with_message("Ambiguous type");
            }
            TypeCheckerErrorKind::IncompatibleTypes {
                expected,
                found,
                span,
            } => {
                report = report.with_message(format!(
                    "Incompatible types. Expected `{:?}`, found `{:?}`",
                    expected.1, found.1
                ));
            }
            TypeCheckerErrorKind::UnificationError { span } => {
                report = report.with_message("Unification error");
            }
        }

        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap();
    }
}