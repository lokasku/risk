use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String), // a pointer to a string stored in memory
}

impl Constant {
    pub fn new_string(s: &str) -> Self {
        Constant::String(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Constant::String(s) => s,
            _ => panic!("Not a string"),
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            Constant::Int(i) => *i,
            _ => panic!("Not an integer"),
        }
    }

    pub fn into_bytecode(&self) -> Vec<u8> {
        match self {
            Constant::Int(i) => {
                let mut bytes = i.to_be_bytes().to_vec();
                bytes.push(0);
                bytes
            },
            Constant::Float(f) => {
                let mut bytes = f.to_be_bytes().to_vec();
                bytes.push(1);
                bytes
            },
            Constant::Bool(b) => {
                let mut bytes = Vec::from([0u8; 8]);
                bytes[0] = *b as u8;
                bytes.push(2);
                bytes.to_vec()
            },
            Constant::Char(c) => {
                let mut bytes = Vec::from([0u8; 8]);
                bytes[0] = *c as u8;
                bytes.push(3);
                bytes
            },
            Constant::String(s) => {
                let mut bytes = Vec::from(s.as_bytes());
                bytes.push(4);
                bytes
            }
        }
    }

    pub fn from_bytecode(bytes: &[u8]) -> Self {
        let tag = bytes.last().unwrap();
        match tag {
            0 => {
                let mut i_bytes = [0u8; 8];
                i_bytes.copy_from_slice(&bytes[..8]);
                let i = i64::from_be_bytes(i_bytes);
                Constant::Int(i)
            },
            1 => {
                let mut f_bytes = [0u8; 8];
                f_bytes.copy_from_slice(&bytes[..8]);
                let f = f64::from_be_bytes(f_bytes);
                Constant::Float(f)
            },
            2 => {
                let b = bytes[0] != 0;
                Constant::Bool(b)
            },
            3 => {
                let c = bytes[0] as char;
                Constant::Char(c)
            },
            4 => {
                let s = String::from_utf8(bytes[..bytes.len() - 1].to_vec()).unwrap();
                Constant::String(s)
            },
            _ => panic!("Invalid tag"),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Float(fl) => write!(f, "{}", fl),
            Constant::Bool(b) => write!(f, "{}", b),
            Constant::Char(c) => write!(f, "{}", c),
            Constant::String(_) => {
                self.as_str().fmt(f)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_bytecode() {
        let c = Constant::Int(42);
        let bytes = c.into_bytecode();
        let c2 = Constant::from_bytecode(&bytes);
        assert_eq!(c, c2);

        let c = Constant::Float(3.14);
        let bytes = c.into_bytecode();
        let c2 = Constant::from_bytecode(&bytes);
        assert_eq!(c, c2);

        let c = Constant::Bool(true);
        let bytes = c.into_bytecode();
        let c2 = Constant::from_bytecode(&bytes);
        assert_eq!(c, c2);

        let c = Constant::Char('c');
        let bytes = c.into_bytecode();
        let c2 = Constant::from_bytecode(&bytes);
        assert_eq!(c, c2);

        let c = Constant::new_string("Hello, world!");
        let bytes = c.into_bytecode();
        let c2 = Constant::from_bytecode(&bytes);
        let s = c.as_str();
        let s2 = c2.as_str();
        assert_eq!(s, s2);
        assert_eq!(c, c2);
    }
}
