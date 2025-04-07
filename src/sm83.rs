use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register16 {
    Bc = 0,
    De = 1,
    Hl = 2,
    Sp = 3,
    Pc = -1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    HlPtr = 6,
    A = 7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterType {
    R8(Register8),
    R16(Register16),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterValue {
    Value8(u8),
    Value16(u16),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

#[derive(Default, Clone)]
pub struct RegisterFile {
    a: u8,
    bc: u16,
    de: u16,
    hl: u16,
    pc: u16,
    sp: u16,
    flags: u8,
}

// TODO: remove allow
#[allow(unreachable_code)]
impl RegisterFile {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            flags: 0,
            a: 0,
            bc: 0,
            de: 0,
            hl: 0,
        }
    }

    pub fn get_register(&self, register: RegisterType) -> RegisterValue {
        match register {
            RegisterType::R8(r) => RegisterValue::Value8(self.get_register8(r)),
            RegisterType::R16(r) => RegisterValue::Value16(self.get_register16(r)),
        }
    }

    pub fn set_register(&mut self, register: RegisterType, value: RegisterValue) {
        match register {
            RegisterType::R8(r) => match value {
                RegisterValue::Value8(v) => self.set_register8(r, v),
                _ => {
                    panic!("can only set register8 to value8")
                }
            },
            RegisterType::R16(r) => match value {
                RegisterValue::Value16(v) => self.set_register16(r, v),
                _ => {
                    panic!("can only set register16 to value16")
                }
            },
        }
    }

    pub fn get_register8(&self, register: Register8) -> u8 {
        match register {
            Register8::A => self.a,
            Register8::B => (self.bc >> 8) as u8,
            Register8::C => self.bc as u8,
            Register8::D => (self.de >> 8) as u8,
            Register8::E => self.de as u8,
            Register8::H => (self.hl >> 8) as u8,
            Register8::L => self.hl as u8,

            _ => {
                panic!("get invalid register8")
            }
        }
    }

    pub fn get_register16(&self, register: Register16) -> u16 {
        match register {
            Register16::Pc => self.pc,
            Register16::Sp => self.sp,
            Register16::Bc => self.bc,
            Register16::De => self.de,
            Register16::Hl => self.hl,

            _ => {
                panic!("get invalid register16")
            }
        }
    }

    pub fn set_register8(&mut self, register: Register8, val: u8) {
        match register {
            Register8::A => self.a = val,
            Register8::B => self.bc = (self.bc & 0x00ff) | ((val as u16) << 8),
            Register8::C => self.bc = (self.bc & 0xff00) | (val as u16),
            Register8::D => self.de = (self.de & 0x00ff) | ((val as u16) << 8),
            Register8::E => self.de = (self.de & 0xff00) | (val as u16),
            Register8::H => self.hl = (self.hl & 0x00ff) | ((val as u16) << 8),
            Register8::L => self.hl = (self.hl & 0xff00) | (val as u16),

            _ => {
                panic!("set invalid register8")
            }
        }
    }

    pub fn set_register16(&mut self, register: Register16, val: u16) {
        match register {
            Register16::Pc => self.pc = val,
            Register16::Sp => self.sp = val,
            Register16::Bc => self.bc = val,
            Register16::De => self.de = val,
            Register16::Hl => self.hl = val,

            _ => {
                panic!("set invalid register16")
            }
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Zero => (self.flags & 1u8) != 0,
            Flag::Subtraction => (self.flags & (1u8 << 1)) != 0,
            Flag::HalfCarry => (self.flags & (1u8 << 2)) != 0,
            Flag::Carry => (self.flags & (1u8 << 3)) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        let bit: u8 = match val {
            true => 1u8,
            false => 0u8,
        };
        match flag {
            Flag::Zero => self.flags = (self.flags & (!1u8)) | bit,
            Flag::Subtraction => self.flags = (self.flags & (!(1u8 << 1))) | (bit << 1),
            Flag::HalfCarry => self.flags = (self.flags & (!(1u8 << 2))) | (bit << 2),
            Flag::Carry => self.flags = (self.flags & (!(1u8 << 3))) | (bit << 3),
        }
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PC: {:016b} ", self.pc)?;
        write!(f, "F: {:08b} ", self.flags)?;
        write!(f, "A: {:08b} ", self.a)?;
        write!(f, "BC: {:016b} ", self.bc)?;
        write!(f, "DE: {:016b} ", self.de)?;
        write!(f, "HL: {:016b}", self.hl)?;
        Ok(())
    }
}
