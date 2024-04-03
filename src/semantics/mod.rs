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
use std::collections::HashMap;
use std::hash::Hash;


const BUILTIN_TYPES: [&str; 7] = ["Integer", "Float", "Bool", "String", "Char", "True", "False"];

#[derive(Debug, Eq, Hash, PartialEq)]
struct Symbol {
    name: String,
    scope_id: u16
}

#[derive(Debug, PartialEq)]
struct SymbolData {
    arity: u8,
    used: bool
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
    symbols: HashMap<Symbol, SymbolData>, // Symbol(name, scope_id) --> (args, used?)
    types: HashMap<TypeName, TypeData>,   // TypeName               --> (args, used?)
    variants: HashMap<VariantName, VariantData>,
    signatures: Vec<Symbol>, // symbols to which a type has been assigned
    scope_id: u16
}

impl SemanticsAnalyzer {
    pub fn new(input: Program) -> Self {
        Self {
            input,
            errors: Vec::new(),
            warnings: Vec::new(),
            symbols: HashMap::new(),
            types: HashMap::new(),
            signatures: Vec::new(),
            variants: HashMap::new(),
            scope_id: 0
        }
    }

    pub fn analyze(&mut self) {
        for node in self.input.statements.clone() {
            match node {
                Statement::Bind(Bind { name, args, .. }) => {
                    if args.len() == 0 {
                        if !self.symbols.contains_key(&Symbol { name: name.name.clone(), scope_id: self.scope_id }) {
                            self.symbols.insert(Symbol { name: name.name.clone(), scope_id: self.scope_id }, SymbolData {
                                arity: 0, used: false
                            });
                        } else {
                            self.errors.push(SemanticError::MultipleDeclarations);
                        }
                    } else {
                        if let Some(data) = self.symbols.get_mut(&Symbol { name: name.name.clone(), scope_id: self.scope_id }) {
                            data.arity = args.len() as u8;
                        } else {
                            self.symbols.insert(Symbol { name: name.name.clone(), scope_id: self.scope_id }, SymbolData {
                                arity: args.len() as u8, used: false
                            });
                        }
                    }
                }
                Statement::TypeDecl(TypeDecl { name, variants, typevars, .. }) => {
                    if !self.types.contains_key(&name.name) {
                        if !BUILTIN_TYPES.contains(&name.name.as_str()) {
                            self.types.insert(name.name, TypeData {
                                arity: typevars.len() as u8, used: false
                            });
                        } else {
                            self.errors.push(SemanticError::ReservedName);
                        }
                    } else {
                        self.errors.push(SemanticError::TypeAlreadyDefined);
                    }

                    for variant in variants {
                        if !self.variants.contains_key(&variant.id.name) {
                            if !BUILTIN_TYPES.contains(&variant.id.name.as_str()) {
                                self.variants.insert(variant.id.name, VariantData {
                                    arity: variant.types.len() as u8, used: false
                                });
                            } else {
                                self.errors.push(SemanticError::ReservedName);
                            }
                        } else {
                            self.errors.push(SemanticError::MultipleDeclarations);
                        }
                    }
                }
                Statement::TypeAssign(TypeAssign { id, .. }) => {
                    if !self.signatures.contains( &Symbol { name: id.name.clone(), scope_id: 0 }) {
                        self.signatures.push(Symbol { name: id.name.clone(), scope_id: 0 });
                    } else {
                        self.errors.push(SemanticError::AlreadyTypedSymbol);
                    }
                }
            }
        }
    }
}