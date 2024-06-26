use crate::{ast::{AnnExpr, BinOp, Bind, Identifier, Literal, LiteralKind, Pattern, Span, Statement, Type, TypeAssign, TypeDecl, Variant}, bytecode::{chunk::Chunk, constant::Constant, opcode::OpCode}};


struct Compiler {
    current_chunk: Chunk,
    current: usize,
    stmts: Vec<Statement<Span>>,
    spans: Vec<Span>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            current_chunk: Chunk::new(vec![], 0, vec![], vec![]),
            current: 0,
            stmts: vec![],
            spans: Vec::new(),
        }
    }

    pub fn compile(&mut self, stmts: Vec<Statement<Span>>) {
        self.stmts = stmts;
        self.current = 0;
        while self.current < self.stmts.len() {
            self.compile_statement();
        }
    }

    fn compile_statement(&mut self) {
        let stmt = self.stmts[self.current].clone();
        match stmt {
            Statement::Bind(ref bind) => {
                self.compile_bind(bind);
            },
            Statement::TypeDecl(ref type_decl) => {
                self.compile_type_decl(type_decl);
            },
            Statement::TypeAssign(ref type_assign) => {
                self.compile_type_assign(type_assign);
            },
        }
        self.current += 1;
    }

    fn compile_bind(&mut self, bind: &Bind<Span>) {
        self.spans.push(bind.span.clone());
        let index = self.current_chunk.addConstant(Constant::new_string(&bind.name.name));
        self.compile_vec(bind.args.clone(), Self::compile_pattern);
        self.compile_expr(&bind.expr);
        self.addOpCode(OpCode::Bind);
        self.addByte(index as u8);
        self.spans.pop();
    }

    fn compile_type_decl(&mut self, type_decl: &TypeDecl) {
        self.spans.push(type_decl.span.clone());
        self.compile_identifer(&type_decl.name);
        self.compile_vec(type_decl.typevars.clone(), Self::compile_identifer);
        self.compile_vec(type_decl.variants.clone(), Self::compile_variant);
        self.addOpCode(OpCode::TypeDecl);
        self.spans.pop();
    }

    fn compile_variant(&mut self, variant: &Variant) {
        self.spans.push(variant.span.clone());
        self.compile_identifer(&variant.id);
        self.compile_vec(variant.types.clone(), Self::compile_type);
        self.addOpCode(OpCode::Variant);
        self.spans.pop();
    }

    fn compile_type_assign(&mut self, type_assign: &TypeAssign) {
        self.spans.push(type_assign.span.clone());
        self.compile_identifer(&type_assign.id);
        self.compile_type(&type_assign.ty);
        self.addOpCode(OpCode::TypeAssign);
        self.spans.pop();
    }

    fn compile_vec<T: Clone, F: Fn(&mut Compiler, &T)>(&mut self, vec: Vec<T>, f: F) {
        let constant = self.current_chunk.addConstant(Constant::Int(vec.len() as i64));
        for item in vec.clone() {
            f(self, &item);
        }

        self.addOpCode(OpCode::Vec);
        self.addByte(constant as u8);



    }

    fn compile_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::ListCons(p1, p2,  s) => {
                self.spans.push(s.clone());
                self.compile_pattern(p1);
                self.compile_pattern(p2);
                self.addOpCode(OpCode::PatternListCons);
                self.spans.pop();
           },
           Pattern::App(id, pats,  span) => {
               self.spans.push(span.clone());
               self.compile_identifer(id);
               self.compile_vec(pats.clone(), Self::compile_pattern);
               self.addOpCode(OpCode::PatternApp);
               self.spans.pop();
           },
           Pattern::Wildcard(span) => {
               self.spans.push(span.clone());
               self.addOpCode(OpCode::PatternWildCard);
               self.spans.pop();
           },
           Pattern::Id(id) => {
               self.spans.push(id.span.clone());
               self.compile_identifer(id);
               self.addOpCode(OpCode::PatternId);
               self.spans.pop();
           },
           Pattern::Literal(lit) => {
               self.spans.push(lit.span.clone());
               self.compile_literal(lit);
               self.addOpCode(OpCode::PatternLiteral);
               self.spans.pop();
           },
           Pattern::Variable(var) => {
               self.spans.push(var.span.clone());
               self.compile_identifer(var);
               self.addOpCode(OpCode::PatternVar);
               self.spans.pop();
           },
       }
    }

    fn compile_expr(&mut self, expr: &AnnExpr<Span>) {
        match expr {
            AnnExpr::App(a) => {
                self.spans.push(a.span.clone());
                self.compile_identifer(&a.ident);
                self.compile_vec(a.args.clone(), Self::compile_expr);
                self.addOpCode(OpCode::ExprApp);
                self.spans.pop();
            },
            AnnExpr::Condition { cond, then, els, ann } => {
                self.spans.push(ann.clone());
                self.compile_expr(then);
                self.compile_expr(els);
                self.compile_expr(cond);
                self.addOpCode(OpCode::ExprCondition);
                self.spans.pop();
            },
            AnnExpr::Identifier { id } => {
                self.compile_identifer(id);
            },
            AnnExpr::Lambda { args, ret, ann } => {
                self.spans.push(ann.clone());
                self.compile_vec(args.clone(), Self::compile_pattern);
                self.compile_expr(ret);
                self.addOpCode(OpCode::ExprLambda);
                self.spans.pop();
            },
            AnnExpr::Let { binds, ret, ann } => {
                self.spans.push(ann.clone());
                self.compile_vec(binds.clone(), Self::compile_bind);
                self.compile_expr(expr);
                self.addOpCode(OpCode::ExprLet);
                self.spans.pop();
            },
            AnnExpr::Match { referral, cases,  ann } => {
                self.spans.push(ann.clone());
                self.compile_expr(referral);
                self.compile_vec(cases.clone(), Self::compile_arm);
                self.addOpCode(OpCode::ExprMatch);
                self.spans.pop();
            },
            AnnExpr::BinOp { op, lhs, rhs, ann } => {
                self.spans.push(ann.clone());
                self.compile_expr(lhs);
                self.compile_expr(rhs);
                self.addOpCode(match op {
                    BinOp::Add => OpCode::Add,
                    BinOp::Sub => OpCode::Sub,
                    BinOp::Mul => OpCode::Mul,
                    BinOp::Div => OpCode::Div,
                    BinOp::Mod => OpCode::Mod,
                    BinOp::Eq => OpCode::Eq,
                    BinOp::Exp => OpCode::Exp,
                    BinOp::LessThan => OpCode::LessThan,
                    BinOp::GreaterThan => OpCode::GreaterThan,
                    BinOp::LessThanOrEq => OpCode::LessThanOrEq,
                    BinOp::GreaterThanOrEq => OpCode::GreaterThanOrEq,
                    _ => panic!("Invalid binary operator")
                });
                self.spans.pop();
            },
            AnnExpr::List { list, ann } => {
                self.spans.push(ann.clone());
                self.compile_vec(list.clone(), Self::compile_expr);
                self.addOpCode(OpCode::ExprList);
                self.spans.pop();
            },
            AnnExpr::PCIdentifier { id } => {
                self.spans.push(id.span.clone());
                self.compile_identifer(id);
                self.addOpCode(OpCode::PushPcIdent);
                self.spans.pop();

            },
            AnnExpr::Literal(lit) => {
                self.compile_literal(lit);
            },
            AnnExpr::Tuple { list, ann } => {
                self.spans.push(ann.clone());
                self.compile_vec(list.clone(), Self::compile_expr);
                self.addOpCode(OpCode::ExprTuple);
                self.spans.pop();
            },
            AnnExpr::Ann { expr, ann } => {
                self.spans.push(ann.0.clone());
                self.compile_expr(expr);
                self.compile_type(&ann.1);
                self.addOpCode(OpCode::ExprAnn);
                self.spans.pop();
            },
        }
    }

    fn compile_arm(&mut self, arms: &(Pattern, Box<AnnExpr<Span>>)) {
        self.compile_pattern(&arms.0);
        self.compile_expr(&arms.1);
    }

    fn compile_identifer(&mut self, id: &Identifier) {
        self.spans.push(id.span.clone());
        let index = self.current_chunk.addConstant(Constant::new_string(&id.name));
        self.addOpCode(OpCode::PushGlobal);
        self.addByte(index as u8);
        self.spans.pop();
    }

    fn compile_type(&mut self, ty: &Type) {
        match ty {
            Type::Generic(gen) => {
                self.spans.push(gen.span.clone());
                let index = self.current_chunk.addConstant(Constant::new_string(&gen.name));
                self.addOpCode(OpCode::TypeGeneric);
                self.addByte(index as u8);
                self.spans.pop();
            },
            Type::Tuple(tys, span) => {
                self.spans.push(span.clone());
                self.compile_vec(tys.clone(), Self::compile_type);
                self.addOpCode(OpCode::TypeTuple);
                self.spans.pop();
            },
            Type::Id(id) => {
                self.spans.push(id.span.clone());
                let index = self.current_chunk.addConstant(Constant::new_string(&id.name));
                self.addByte(index as u8);
                self.addOpCode(OpCode::TypeId);
                self.spans.pop();
            },
            Type::Func(ret, args, span) => {
                self.spans.push(span.clone());
                self.compile_vec(args.clone(), Self::compile_type);
                self.compile_type(ret);
                self.addOpCode(OpCode::TypeFunc);
                self.spans.pop();
            },
            Type::App(app, tys, span) => {
                self.spans.push(span.clone());
                self.compile_identifer(app);
                self.compile_vec(tys.clone(), Self::compile_type);
                self.addOpCode(OpCode::TypeApp);
                self.spans.pop();
            },
        }
    }

    fn compile_literal(&mut self, lit: &Literal) {
        self.spans.push(lit.span.clone());
        self.addOpCode(OpCode::PushConst);
        match lit.lit.clone() {
            LiteralKind::Integer(i) => {
                let index = self.current_chunk.addConstant(Constant::Int(i));
                self.addByte(index as u8);
            },
            LiteralKind::Float(f) => {
                let index = self.current_chunk.addConstant(Constant::Float(f));
                self.addByte(index as u8);
            },
            LiteralKind::String(s) => {
                let index = self.current_chunk.addConstant(Constant::new_string(&s));
                self.addByte(index as u8);
            },
            LiteralKind::Char(c) => {
                let index = self.current_chunk.addConstant(Constant::Char(c));
                self.addByte(index as u8);
            },
            LiteralKind::Bool(b) => {
                let index = self.current_chunk.addConstant(Constant::Bool(b.into()));
                self.addByte(index as u8);
            },
        }
        self.spans.pop();
    }

    fn addByte(&mut self, byte: u8) {
        let span = self.spans.last().unwrap();
        self.current_chunk.addByte(byte, span.clone());
    }

    fn addOpCode(&mut self, op: OpCode) {
        let span = self.spans.last().unwrap();
        self.current_chunk.addOpCode(op, span.clone());
    }
}


pub fn compile_program(program: Vec<Statement<Span>>) -> Chunk {
    let mut compiler = Compiler::new();
    compiler.compile(program);
    compiler.current_chunk
}
