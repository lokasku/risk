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
pub struct Symbol {
    name: String,
    scope_id: u16,
    level: u16,
}

#[derive(Debug, PartialEq)]
pub struct SymbolData {
    arity: u8,
    used: bool,
}

#[derive(Debug)]
pub struct TypeData {
    arity: u8,
    used: bool,
}

#[derive(Debug)]
pub struct VariantData {
    arity: u8,
    used: bool,
}

type VariantName = String;
type TypeName = String;
type FuncName = String;

#[derive(Debug)]
pub struct AnalysisOutput {
    pub errors: Vec<SemanticError>,
    pub warnings: Vec<SemanticWarning>,
    pub symbols: HashMap<Symbol, SymbolData>,
    pub types: HashMap<TypeName, TypeData>,
    pub variants: HashMap<VariantName, VariantData>,
    pub signatures: Vec<FuncName>, // symbols to which a type has been assigned
    pub scope_id: u16,
    pub level: u16,
}

impl AnalysisOutput {
    pub fn new() -> Self {
        Self {
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

    fn find_identifier(&mut self, name: &str) -> Option<&mut SymbolData> {
        let mut sym = Symbol {
            name: name.into(),
            scope_id: self.scope_id + 1,
            level: 0,
        };

        let mut symbols = &mut self.symbols;

        for level in (0..=self.level).rev() {
            sym.level = level;
            sym.scope_id -= 1;

            polonius!(|symbols| -> Option<&'polonius mut SymbolData> {
                if let Some(data) = symbols.get_mut(&sym) {
                    data.used = true;
                    polonius_return!(Some(data));
                }
            });
        }
        None
    }

    pub fn analyze_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Bind(Bind {
                name, args, expr, ..
            }) => {
                let sym = Symbol {
                    name: name.name.clone(),
                    scope_id: self.scope_id,
                    level: self.level,
                };

                let some_arguments = args.len() != 0;

                if some_arguments {
                    self.level += 1;
                    self.scope_id += 1;
                }

                if !some_arguments {
                    if !self.symbols.contains_key(&sym) {
                        self.symbols.insert(
                            sym,
                            SymbolData {
                                arity: 0,
                                used: false,
                            },
                        );
                    } else {
                        self.errors.push(SemanticError::MultipleDeclarations);
                    }
                } else if !self.symbols.contains_key(&sym) {
                    self.symbols.insert(
                        sym,
                        SymbolData {
                            arity: args.len() as u8,
                            used: false,
                        },
                    );
                }

                for arg in args {
                    self.analyze_pattern(arg);
                }

                self.analyze_expr(expr);

                if some_arguments {
                    self.level -= 1;
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
            Statement::TypeAssign(TypeAssign { id, ty, .. }) => {
                if !self.signatures.contains(&id.name) {
                    self.signatures.push(id.name);
                } else {
                    self.errors.push(SemanticError::AlreadyTypedSymbol);
                }
                self.analyze_type(ty);
            }
        }
    }

    pub fn analyze_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Identifier(Identifier { name, .. }) => {
                if self.find_identifier(name.as_str()).is_none() {
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
                if ident.name.chars().next().unwrap().is_lowercase() {
                    if let Some(data) = self.find_identifier(ident.name.as_str()) {
                        if data.arity != args.len() as u8 {
                            self.errors.push(SemanticError::WrongArity);
                        }
                    }
                } else if let Some(data) = self.variants.get_mut(&ident.name) {
                    data.used = true;

                    if data.arity != args.len() as u8 {
                        self.errors.push(SemanticError::WrongArity);
                    }
                } else {
                    self.errors.push(SemanticError::NotACallee);
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
                    self.analyze_statement(Statement::Bind(bind));
                }

                self.analyze_expr(*expr);

                self.level -= 1;
            }
            Expr::Match(referral, cases, ..) => {
                self.analyze_expr(*referral);

                for case in cases {
                    self.level += 1;
                    self.scope_id += 1;

                    self.analyze_pattern(*case.0);
                    self.analyze_expr(*case.1);

                    self.level -= 1;
                }
            }
            Expr::BinOp(_, lhs, rhs, _) => {
                self.analyze_expr(*lhs);
                self.analyze_expr(*rhs);
            }
            Expr::Lambda(args, expr, _) => {
                let some_arguments = args.len() != 0;

                if some_arguments {
                    self.level += 1;
                    self.scope_id += 1;

                    for arg in args {
                        self.analyze_pattern(arg);
                    }
                }

                self.analyze_expr(*expr);

                if some_arguments {
                    self.level -= 1;
                }
            }
            Expr::Ann(expr, r#type, ..) => {
                self.analyze_expr(*expr);
                self.analyze_type(r#type);
            }
            Expr::List(exprs, ..) | Expr::Tuple(exprs, ..) => {
                for expr in exprs {
                    self.analyze_expr(expr);
                }
            }
            Expr::Literal(_) => {}
        }
    }

    pub fn analyze_pattern(&mut self, pattern: Pattern) {
        match pattern {
            Pattern::Variable(id) => {
                let sym = Symbol {
                    name: id.name.clone(),
                    scope_id: self.scope_id,
                    level: self.level,
                };

                if !self.symbols.contains_key(&sym) {
                    self.symbols.insert(
                        sym,
                        SymbolData {
                            arity: 0,
                            used: false,
                        },
                    );
                } else {
                    self.errors.push(SemanticError::MultipleDeclarations);
                }
            }
            Pattern::ListCons(lhs, rhs, _) => {
                self.analyze_pattern(*lhs);
                self.analyze_pattern(*rhs);
            }
            Pattern::App(id, patterns, ..) => {
                if let Some(data) = self.variants.get_mut(&id.name) {
                    data.used = true;

                    if data.arity != patterns.len() as u8 {
                        self.errors.push(SemanticError::WrongArity);
                    }
                } else {
                    self.errors.push(SemanticError::UndefinedConstructor);
                }
                // for pattern in patterns {
                //    self.analyze_pattern(pattern);
                // }
            }
            Pattern::Id(id, ..) => {
                if self.variants.get_mut(&id.name).is_none() {
                    self.errors.push(SemanticError::UndefinedConstructor);
                }
            }
            _ => {}
        }
    }

    pub fn analyze_type(&mut self, r#type: Type) {
        match r#type {
            Type::Id(Identifier { name, .. }) => {
                if BUILTIN_TYPES.contains(&name.as_str()) {
                } else if let Some(data) = self.types.get_mut(&name) {
                    data.used = true;

                    if data.arity != 0 {
                        self.errors.push(SemanticError::WrongArity)
                    }
                } else {
                    self.errors.push(SemanticError::UndefinedType)
                }
            }
            Type::App(id, types, ..) => {
                if BUILTIN_TYPES.contains(&id.name.as_str()) {
                    self.errors.push(SemanticError::WrongArity)
                } else if let Some(data) = self.types.get_mut(&id.name) {
                    data.used = true;

                    if types.len() as u8 != data.arity {
                        self.errors.push(SemanticError::WrongArity)
                    }
                } else {
                    self.errors.push(SemanticError::UndefinedType)
                }
            }
            Type::Tuple(types, ..) => {
                for ty in types {
                    self.analyze_type(ty);
                }
            }
            Type::Func(ret, args, ..) => {
                for arg in args {
                    self.analyze_type(arg);
                }

                self.analyze_type(*ret);
            }
            _ => {}
        }
    }
}

pub fn analyze(ao: &mut AnalysisOutput, input: Program) {
    for statement in input.statements {
        ao.analyze_statement(statement);
    }
}
