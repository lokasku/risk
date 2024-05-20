use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::OpCode;

struct Chunk {
    data: Vec<u8>,
    count: usize,
    constant: Vec<Constant>,
}

impl Chunk {
    pub fn new(data: Vec<u8>, count: usize, constant: Vec<Constant>) -> Self {
        Chunk { data, count, constant }
    }

    pub fn addOpCode(&mut self, op: OpCode) {
        self.data.push(op as u8);
        self.count += 1;
    }

    pub fn addByte(&mut self, byte: u8) {
        self.data.push(byte);
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
        self.constant[index]
    }

    pub fn getOpCodeCount(&self) -> usize {
        self.count
    }

    pub fn getConstantCount(&self) -> usize {
        self.constant.len()
    }

    pub fn dissassemble(&self, name: &str) {
        println!("== {} ==", name);
        for offset in 0..self.count {
            self.dissassembleInstruction(offset);
        }
    }

    pub fn dissassembleInstruction(&self, offset: usize) {
        print!("{:04} ", offset);
        let instruction = self.getOpCode(offset);
        match instruction {
            OpCode::PushConst => {
                let index = self.data[offset + 1] as usize;
                println!("PUSH_CONST {}", self.getConstant(index));
            },
            _ => {
                println!("Unknown opcode {:?}", instruction);
            },
        }
    }
}
