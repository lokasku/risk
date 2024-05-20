use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    StrPtr(*const u8, usize), // a pointer to a string stored in memory
}

impl Constant {
    pub fn new_string(s: &str) -> Self {
        Constant::StrPtr(s.as_ptr(), s.len())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Constant::StrPtr(ptr, length) => unsafe { std::str::from_raw_parts(*ptr, *length) },
            _ => panic!("Not a string"),
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
            Constant::StrPtr(ptr, length) => {
                let mut bytes = length.to_be_bytes().to_vec();
                bytes.extend(ptr.addr().to_le_bytes());
                bytes.push(4);
                bytes
            },
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
                let length_bytes = &bytes[..8];
                let ptr_bytes = &bytes[8..16];
                let mut length = [0u8; 8];
                let mut addr = [0u8; 8];
                length.copy_from_slice(length_bytes);
                addr.copy_from_slice(ptr_bytes);
                let length = usize::from_be_bytes(length);
                let ptr = usize::from_le_bytes(addr);
                let ptr = ptr as *const u8;
                Constant::StrPtr(ptr, length)
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
            Constant::StrPtr(ptr, length) => {
                let s = unsafe { std::str::from_raw_parts(*ptr, *length) };
                write!(f, "{}", s)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let s = "Hello, world!";
        let c = Constant::new_string(s);
        assert_eq!(c.as_str(), s);
    }

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
