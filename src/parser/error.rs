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

