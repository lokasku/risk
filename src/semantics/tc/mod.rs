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

mod ast;
mod error;

use std::collections::HashMap;

use crate::ast::*;
use ast::Annot;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub span: Span,
    pub scope_id: u16,
    pub level: u16
}

type Variant = String;

#[derive(Debug)]
pub struct TypeChecker {
    pub output: Vec<Statement<Annot>>,
    pub symbols: HashMap<Symbol, Type>, // Symbol -> Type
    pub signatures: HashMap<Symbol, Type>, // Symbol -> Signature
    pub variants: HashMap<Variant, Type>, // Variant -> App Type
    pub filename: &'static str,
    pub scope_id: u16,
    pub level: u16,
}

impl TypeChecker {
    pub fn new(filename: &'static str) -> Self {
        TypeChecker {
            output: Vec::new(),
            symbols: HashMap::new(),
            signatures: HashMap::new(),
            variants: HashMap::new(),
            filename,
            scope_id: 0,
            level: 0,
        }
    }

    pub fn get_type(&self, expr: AnnExpr<Span>) -> Option<Type> {
        match expr {
            AnnExpr::Identifier { id } => {
                if let Some(typ) = self.symbols.get(&Symbol {
                    span: id.span.clone(),
                    scope_id: self.scope_id,
                    level: self.level
                }) {
                    Some(typ.clone())
                } else {
                    None
                }
            }
            AnnExpr::PCIdentifier { id } => {
                if let Some(typ) = self.variants.get(&id.name) {
                    Some(typ.clone())
                } else {
                    None
                }
            }
            AnnExpr::App(App { ident, args, span }) => {
                if let Some(typ) = self.variants.get(&ident.name) {
                    Some(typ.clone())
                } else {
                    None
                }
            }
            _ => todo!()
        }
    }

    pub fn analyze_statement(&mut self, statement: Statement<Span>) {
        match statement {
            Statement::Bind(Bind {
                name,
                args,
                expr,
                span
            }) => {
                // let sym = Symbol {
                //     span: name.span.clone(),
                //     scope_id: self.scope_id,
                //     level: self.level
                // };

                // let expr_type = expr.get_type();

                // if let Some(symbol_type) = self.signatures.get(&sym) {
                //     if expr.get_type() != *symbol_type {
                //         error::TypeCheckerError {
                //             kind: error::TypeCheckerErrorKind::MismatchedTypes {
                //                 expected: (symbol_type.get_span().clone(), symbol_type.clone()),
                //                 found: (expr_type.get_span().clone(), expr_type)
                //             },
                //             span: span.clone()
                //         }.report(self.filename);
                //     }
                // } else {
                //     if let Some(typ) = expr.get_type() {
                //         self.signatures.insert(sym, typ);
                //     } else {
                //         error::TypeCheckerError {
                //             kind: error::TypeCheckerErrorKind::AmbiguousType {
                //                 span: span.clone()
                //             },
                //             span: span.clone()
                //         }.report(self.filename);
                //     }
                // }
            }
            _ => todo!()
        }
    }
}