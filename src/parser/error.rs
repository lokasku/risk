use crate::ast::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    UnexpectedToken { expected: String, found: Span },
    UnexpectedEOF { expected: String },
    UnexpectedEndOfInput,
    UnexpectedTokenInPattern { found: Span },
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
}