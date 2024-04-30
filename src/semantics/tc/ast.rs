use crate::ast::*;

type TypedExpr = AnnExpr<(Span, Type)>;

impl TypedExpr {
    pub fn get_span(&self) -> &Span {
        match self {
            AnnExpr::Identifier { id } => &id.span,
            AnnExpr::PCIdentifier { id } => &id.span,
            AnnExpr::App(app) => &app.span,
            AnnExpr::Condition { ann, .. } => &ann.0,
            AnnExpr::Let { ann, .. } => &ann.0,
            AnnExpr::Match { ann, .. } => &ann.0,
            AnnExpr::Literal(lit) => &lit.span,
            AnnExpr::BinOp { ann, .. } => &ann.0,
            AnnExpr::Lambda { ann, .. } => &ann.0,
            AnnExpr::Ann { ann, .. } => &ann.0,
            AnnExpr::List { ann, .. } => &ann.0,
            AnnExpr::Tuple { ann, .. } => &ann.0,
        }
    }
}
