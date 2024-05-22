

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    PushConst,
    PushLocal,
    PushGlobal,
    PushPcIdent,

    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
    Eq,
    Ineq,
    And,
    Or,
    Not,
    ExprApp,
    ExprCondition,
    ExprLet,
    ExprMatch,
    ExprLambda,
    ExprAnn,
    ExprList,
    ExprTuple,

    Jump,
    JumpIfFalse,
    Call,
    Return,

    TypeId,
    TypeGeneric,
    TypeApp,
    TypeTuple,
    TypeFunc,

    PatternWildCard,
    PatternListCons,
    PatternVar,
    PatternId,
    PatternApp,
    PatternLiteral,

    Variant,

    Bind,
    TypeAssign,
    TypeDecl,

    Vec
}


impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::PushConst,
            1 => OpCode::PushLocal,
            2 => OpCode::PushGlobal,
            3 => OpCode::PushPcIdent,
            4 => OpCode::Pop,
            5 => OpCode::Add,
            6 => OpCode::Sub,
            7 => OpCode::Mul,
            8 => OpCode::Div,
            9 => OpCode::Mod,
            10 => OpCode::Exp,
            11 => OpCode::LessThan,
            12 => OpCode::GreaterThan,
            13 => OpCode::LessThanOrEq,
            14 => OpCode::GreaterThanOrEq,
            15 => OpCode::Eq,
            16 => OpCode::Ineq,
            17 => OpCode::And,
            18 => OpCode::Or,
            19 => OpCode::Not,
            20 => OpCode::ExprApp,
            21 => OpCode::ExprCondition,
            22 => OpCode::ExprLet,
            23 => OpCode::ExprMatch,
            24 => OpCode::ExprLambda,
            25 => OpCode::ExprAnn,
            26 => OpCode::ExprList,
            27 => OpCode::ExprTuple,
            28 => OpCode::Jump,
            29 => OpCode::JumpIfFalse,
            30 => OpCode::Call,
            31 => OpCode::Return,
            32 => OpCode::TypeId,
            33 => OpCode::TypeGeneric,
            34 => OpCode::TypeApp,
            35 => OpCode::TypeTuple,
            36 => OpCode::TypeFunc,
            37 => OpCode::PatternWildCard,
            38 => OpCode::PatternListCons,
            39 => OpCode::PatternVar,
            40 => OpCode::PatternId,
            41 => OpCode::PatternApp,
            42 => OpCode::PatternLiteral,
            43 => OpCode::Variant,
            44 => OpCode::Bind,
            45 => OpCode::TypeAssign,
            46 => OpCode::TypeDecl,
            47 => OpCode::Vec,
            _ => panic!("Invalid OpCode"),
        }


    }
}
