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

mod error;
mod warning;

use crate::ast::*;
use crate::semantics::error::SemanticError;
use crate::semantics::warning::SemanticWarning;
use polonius_the_crab::{polonius, polonius_return};
use std::collections::HashMap;
use std::hash::Hash;

const BUILTIN_TYPES: [&str; 7] = [
    "Integer", "Float", "Bool", "String", "Char", "True", "False",
];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Symbol {
    name: String,
    scope_id: u16,
    level: u16,
}

#[derive(Debug, PartialEq)]
struct SymbolData {
    arity: u8,
    used: bool,
}

#[derive(Debug)]
struct TypeData {
    arity: u8,
    used: bool,
}

#[derive(Debug)]
struct VariantData {
    arity: u8,
    used: bool,
}

type VariantName = String;
type TypeName = String;

#[derive(Debug)]
pub struct SemanticsAnalyzer {
    pub input: Program,
    pub errors: Vec<SemanticError>,
    pub warnings: Vec<SemanticWarning>,
    symbols: HashMap<Symbol, SymbolData>,
    types: HashMap<TypeName, TypeData>,
    variants: HashMap<VariantName, VariantData>,
    signatures: Vec<Symbol>, // symbols to which a type has been assigned
    scope_id: u16,
    level: u16,
}

impl SemanticsAnalyzer {
    pub fn new(input: Program) -> Self {
        Self {
            input,
            errors: Vec::new(),
            warnings: Vec::new(),
            symbols: HashMap::new(),
            types: HashMap::new(),
            variants: HashMap::new(),
            signatures: Vec::new(),
            scope_id: 0,
            level: 0,
        }
    }

    pub fn find_identifier(&mut self, name: &str) -> Option<&mut SymbolData> {
        let mut sym = Symbol {
            name: name.into(),
            scope_id: self.scope_id,
            level: 0,
        };

        let mut symbols = &mut self.symbols;

        for level in (0..=self.level).rev() {
            sym.level = level;

            polonius!(|symbols| -> Option<&'polonius mut SymbolData> {
                if let Some(data) = symbols.get_mut(&sym) {
                    data.used = true;
                    polonius_return!(Some(data));
                }
            });
            sym.scope_id -= 1;
        }
        None
    }

    pub fn analyze(&mut self, statement: Statement) {
        match statement {
            Statement::Bind(Bind { name, args, .. }) => {
                if args.len() == 0 {
                    if !self.symbols.contains_key(&Symbol {
                        name: name.name.clone(),
                        scope_id: self.scope_id,
                        level: self.level,
                    }) {
                        self.symbols.insert(
                            Symbol {
                                name: name.name.clone(),
                                scope_id: self.scope_id,
                                level: self.level,
                            },
                            SymbolData {
                                arity: 0,
                                used: false,
                            },
                        );
                    } else {
                        self.errors.push(SemanticError::MultipleDeclarations);
                    }
                } else {
                    if let Some(data) = self.symbols.get_mut(&Symbol {
                        name: name.name.clone(),
                        scope_id: self.scope_id,
                        level: self.level,
                    }) {
                        data.arity = args.len() as u8;
                    } else {
                        self.symbols.insert(
                            Symbol {
                                name: name.name.clone(),
                                scope_id: self.scope_id,
                                level: self.level,
                            },
                            SymbolData {
                                arity: args.len() as u8,
                                used: false,
                            },
                        );
                    }
                }
            }
            Statement::TypeDecl(TypeDecl {
                name,
                variants,
                typevars,
                ..
            }) => {
                if !self.types.contains_key(&name.name) {
                    if !BUILTIN_TYPES.contains(&name.name.as_str()) {
                        self.types.insert(
                            name.name,
                            TypeData {
                                arity: typevars.len() as u8,
                                used: false,
                            },
                        );
                    } else {
                        self.errors.push(SemanticError::ReservedName);
                    }
                } else {
                    self.errors.push(SemanticError::TypeAlreadyDefined);
                }

                for variant in variants {
                    if !self.variants.contains_key(&variant.id.name) {
                        if !BUILTIN_TYPES.contains(&variant.id.name.as_str()) {
                            self.variants.insert(
                                variant.id.name,
                                VariantData {
                                    arity: variant.types.len() as u8,
                                    used: false,
                                },
                            );
                        } else {
                            self.errors.push(SemanticError::ReservedName);
                        }
                    } else {
                        self.errors.push(SemanticError::MultipleDeclarations);
                    }
                }
            }
            Statement::TypeAssign(TypeAssign { id, .. }) => {
                if !self.signatures.contains(&Symbol {
                    name: id.name.clone(),
                    scope_id: 0,
                    level: 0,
                }) {
                    self.signatures.push(Symbol {
                        name: id.name.clone(),
                        scope_id: 0,
                        level: 0,
                    });
                } else {
                    self.errors.push(SemanticError::AlreadyTypedSymbol);
                }
            }
        }
    }

    /// Analyze the expression, checking that there are no calls to non-existent symbols and that functions are applied to the right number of arguments.
    pub fn analyze_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Identifier(Identifier { name, .. }) => {
                if let None = self.find_identifier(name.as_str()) {
                    self.errors.push(SemanticError::UndefinedSymbol);
                }
            }
            Expr::PCIdentifier(Identifier { name, .. }) => {
                if let Some(data) = self.variants.get_mut(&name) {
                    data.used = true;
                } else {
                    self.errors.push(SemanticError::UndefinedSymbol);
                }
            }
            Expr::App(App { ident, args, .. }) => {
                if let Some(data) = self.find_identifier(ident.name.as_str()) {
                    if data.arity != args.len() as u8 {
                        self.errors.push(SemanticError::WrongArity);
                    }
                } else if let Some(data) = self.variants.get_mut(&ident.name) {
                    data.used = true;

                    if data.arity != args.len() as u8 {
                        self.errors.push(SemanticError::WrongArity);
                    }
                } else {
                    self.errors.push(SemanticError::UndefinedSymbol);
                }
            }
            Expr::Condition(condition, then, r#else, ..) => {
                self.analyze_expr(*condition);
                self.analyze_expr(*then);
                self.analyze_expr(*r#else);
            }
            Expr::Let(binds, expr, ..) => {
                self.level += 1;
                self.scope_id += 1;

                for bind in binds {
                    self.analyze(Statement::Bind(bind));
                }

                self.analyze_expr(*expr);

                self.level -= 1;
            }

            // TODO: Analyze patterns
            Expr::Match(referral, cases, ..) => {
                self.analyze_expr(*referral);

                for case in cases {
                    // self.analyze_pattern(case.0);
                    self.analyze_expr(*case.1)
                }
            }
            Expr::BinOp(_, lhs, rhs, _) => {
                self.analyze_expr(*lhs);
                self.analyze_expr(*rhs);
            }

            // TODO: Analyze patterns
            Expr::Lambda(_, expr, _) => {
                // for arg in args {
                //    self.analyze_pattern(arg);
                // }
                self.level += 1;
                self.scope_id += 1;

                self.analyze_expr(*expr);

                self.scope_id -= 1;
            }
            Expr::Ann(expr, ..) => self.analyze_expr(*expr),
            Expr::List(exprs, ..) | Expr::Tuple(exprs, ..) => {
                for expr in exprs {
                    self.analyze_expr(expr);
                }
            }
            Expr::Literal(_) => {}
        }
    }
}
