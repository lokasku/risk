use crate::ast::Span;
use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::OpCode;

#[derive(Debug, Clone)]
pub struct Chunk {
    data: Vec<u8>,
    count: usize,
    constant: Vec<Constant>,
    spans: Vec<Span>,
    dis_constinue: bool,
}

impl Chunk {
    pub fn new(data: Vec<u8>, count: usize, constant: Vec<Constant>, spans: Vec<Span>) -> Self {
        Chunk {
            data,
            count,
            constant,
            spans,
            dis_constinue: false,
        }
    }

    pub fn addOpCode(&mut self, op: OpCode, span: Span) {
        self.data.push(op as u8);
        self.spans.push(span);
        self.count += 1;
    }

    pub fn addByte(&mut self, byte: u8, span: Span) {
        self.data.push(byte);
        self.spans.push(span);
        self.count += 1;
    }

    pub fn addConstant(&mut self, constant: Constant) -> usize {
        self.constant.push(constant);
        self.constant.len() - 1
    }

    pub fn getOpCode(&self, index: usize) -> OpCode {
        OpCode::from(self.data[index])
    }

    pub fn getConstant(&self, index: usize) -> Constant {
        self.constant[index].clone()
    }

    pub fn getOpCodeCount(&self) -> usize {
        self.count
    }

    pub fn getConstantCount(&self) -> usize {
        self.constant.len()
    }

    pub fn dissassemble(&mut self, name: &str) {
        let special_char = "━";
        let bordure_char0 = "╭";
        let bordure_char1 = "╯";
        let bordure_char2 = "╮";
        let bordure_char3 = "╰";
        let byte_code_s = " Bytecode ";
        let space0 = " ".repeat(2 + name.len() / 2);
        let space1 = " ".repeat(2 + name.len() / 2);
        let bordure0 = format!(
            "{}{}{}{}{}",
            bordure_char0,
            special_char.repeat(name.len() / 2).as_str(),
            byte_code_s,
            special_char.repeat(name.len() / 2).as_str(),
            bordure_char2
        );
        let bordure1 = format!(
            "{}{}{}",
            bordure_char3,
            special_char.repeat(name.len() + byte_code_s.len()).as_str(),
            bordure_char1
        );
        println!("{}", bordure0);
        println!("{}{}{}", space0, name, space1);
        println!("{}", bordure1);
        let mut index = 0;
        while index < self.count {
            index = self.dissassembleInstruction(index);
        }
    }

    pub fn dissassembleInstruction(&mut self, offset: usize) -> usize {
        let instruction = self.getOpCode(offset);
        match instruction {
            OpCode::PushConst => {
                print!("{:04} ", offset);
                self.constantInstruction("PUSH_CONST", offset)
            }
            OpCode::Vec => {
                print!("{:04} ", offset);
                self.constantInstruction("VEC", offset)
            }
            OpCode::PushLocal => {
                print!("{:04} ", offset);
                self.simpleInstruction("PUSH_LOCAL", offset)
            }
            OpCode::PushGlobal => {
                print!("{:04} ", offset);
                self.constantInstruction("PUSH_GLOBAL", offset)
            }
            OpCode::Bind => {
                print!("{:04} ", offset);
                self.constantInstruction("BIND", offset)
            }
            OpCode::ExprApp => {
                print!("{:04} ", offset);
                self.simpleInstruction("EXPR_APP", offset)
            }
            OpCode::ExprList => {
                print!("{:04} ", offset);
                self.simpleInstruction("EXPR_LIST", offset)
            }

            OpCode::TypeGeneric => {
                print!("{:04} ", offset);
                self.constantInstruction("TYPE_GENERIC", offset)
            }

            OpCode::Variant => {
                print!("{:04} ", offset);
                self.simpleInstruction("VARIANT", offset)
            }

            OpCode::TypeDecl => {
                print!("{:04} ", offset);
                self.simpleInstruction("TYPE_DECL", offset)
            }
            OpCode::ExprCondition => {
                print!("{:04} ", offset);
                self.simpleInstruction("EXPR_CONDITION", offset)
            }

            _ => {
                panic!("Unknown opcode {:?}", instruction)
            }
        }
    }

    pub fn constantInstruction(&mut self, name: &str, offset: usize) -> usize {
        let constant = self.getConstant(self.data[offset + 1] as usize);
        println!("{:<16} '{}'   \"{}\"", name, constant, self.spans[offset]);
        offset + 2
    }

    pub fn simpleInstruction(&self, name: &str, offset: usize) -> usize {
        println!("{:<16}        \"{}\"", name, self.spans[offset]);
        offset + 1
    }
}
