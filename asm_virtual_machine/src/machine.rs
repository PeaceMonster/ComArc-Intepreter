use std::{
    fmt::Display,
    num::Wrapping,
    ops::{BitAnd, BitOr, BitXor},
};

use strum::EnumString;

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, EnumString, strum::Display)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug, Clone)]
pub struct Label(pub String);

#[derive(Debug, Clone)]
pub enum Instruction {
    Zero(Register),
    Mov(Register, Register),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Inc(Register),
    Dec(Register),
    And(Register, Register, Register),
    Or(Register, Register, Register),
    Xor(Register, Register, Register),
    Not(Register),
    Shl(Register, u8),
    Shr(Register, u8),
    Jz(Label),
    Jnz(Label),
    J(Label),
    Set(Register, u8),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Zero(register) => write!(f, "ZERO {}", register),
            Instruction::Mov(register, register1) => write!(f, "MOV {}, {}", register, register1),
            Instruction::Add(register, register1, register2) => {
                write!(f, "ADD {}, {}, {}", register, register1, register2)
            }
            Instruction::Sub(register, register1, register2) => {
                write!(f, "SUB {}, {}, {}", register, register1, register2)
            }
            Instruction::Inc(register) => write!(f, "INC {}", register),
            Instruction::Dec(register) => write!(f, "DEC {}", register),
            Instruction::And(register, register1, register2) => {
                write!(f, "AND {}, {}, {}", register, register1, register2)
            }
            Instruction::Or(register, register1, register2) => {
                write!(f, "OR {}, {}, {}", register, register1, register2)
            }
            Instruction::Xor(register, register1, register2) => {
                write!(f, "XOR {}, {}, {}", register, register1, register2)
            }
            Instruction::Not(register) => write!(f, "NOT {}", register),
            Instruction::Shl(register, k) => write!(f, "SHL {}, {}", register, k),
            Instruction::Shr(register, k) => write!(f, "SHR {}, {}", register, k),
            Instruction::Jz(label) => write!(f, "JZ {}", label.0),
            Instruction::Jnz(label) => write!(f, "JNZ {}", label.0),
            Instruction::J(label) => write!(f, "J {}", label.0),
            Instruction::Set(register, k) => write!(f, "SET {}, {}", register, k),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProgramLine {
    Ins(Instruction),
    Lbl(Label),
}

pub struct Machine {
    registers: Vec<(Register, Wrapping<u8>)>,
    flag: bool,
    program: Vec<ProgramLine>,
    index: usize,
}

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    EndOfProgram,
    MissingLabel,
}

impl Machine {
    fn init_registers() -> Vec<(Register, Wrapping<u8>)> {
        vec![
            (Register::R0, Wrapping(0)),
            (Register::R1, Wrapping(0)),
            (Register::R2, Wrapping(0)),
            (Register::R3, Wrapping(0)),
            (Register::R4, Wrapping(0)),
            (Register::R5, Wrapping(0)),
            (Register::R6, Wrapping(0)),
            (Register::R7, Wrapping(0)),
        ]
    }

    pub fn new() -> Machine {
        Machine {
            registers: Machine::init_registers(),
            flag: false,
            program: Vec::new(),
            index: 0,
        }
    }

    pub fn get_current_instruction(&self) -> String {
        let current_instruction = self.program.get(self.index);
        match current_instruction {
            None => format!("End of program"),
            Some(ProgramLine::Ins(i)) => format!("At instruction: {}", i),
            Some(ProgramLine::Lbl(l)) => format!("At label: {:?}", l),
        }
    }

    pub fn print_current_instruction(&self) {
        println!("{}", self.get_current_instruction());
        println!("------");
    }

    pub fn get_string_registers(&self) -> String {
        let mut result = String::new();
        for (r, v) in self.registers.iter() {
            result = format!("{}Register: {:?} = {}\n", result, r, v);
        }
        result
    }

    pub fn print_registers(&self) {
        print!("{}", self.get_string_registers());
    }

    pub fn init_program(&mut self, program: Vec<ProgramLine>) {
        self.program = program;
        self.index = 0;
    }

    fn get_register(&self, r: &Register) -> Wrapping<u8> {
        let res = self
            .registers
            .iter()
            .filter(|re| re.0 == *r)
            .nth(0)
            .unwrap();
        res.1
    }

    fn modify_register(&mut self, r: &Register, value: Wrapping<u8>) {
        let reg_pos = self.registers.iter().position(|re| re.0 == *r).unwrap();
        let reg = self.registers.get_mut(reg_pos).unwrap();
        reg.1 = value;
    }

    fn goto_label(&mut self, lbl: &Label) -> Result<(), ProgramError> {
        let Some(label_pos) = self.program.iter().position(|pl| match pl {
            ProgramLine::Ins(_) => false,
            ProgramLine::Lbl(label) => label.0.eq(&lbl.0),
        }) else {
            return Err(ProgramError::MissingLabel);
        };

        self.index = label_pos;
        Ok(())
    }

    fn set_flag(&mut self, value: bool) {
        self.flag = value;
    }

    fn intepret_instruction(&mut self, ins: &Instruction) -> Result<(), ProgramError> {
        match ins {
            Instruction::Zero(register) => self.modify_register(&register, Wrapping(0)),
            Instruction::Mov(register, register1) => {
                let val = self.get_register(&register1);
                self.modify_register(&register, val);
            }
            Instruction::Add(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1 + val2;
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Sub(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1 - val2;
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Inc(register) => {
                let val = self.get_register(&register);
                let res = val + Wrapping(1);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Dec(register) => {
                let val = self.get_register(&register);
                let res = val - Wrapping(1);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::And(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitand(val2);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Or(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitor(val2);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Xor(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitxor(val2);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Not(register) => {
                let val = self.get_register(&register);
                let res = !val;
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Shl(register, k) => {
                let val = self.get_register(&register);
                let res = Wrapping(val.0 << k);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Shr(register, k) => {
                let val = self.get_register(&register);
                let res = Wrapping(val.0 >> k);
                self.modify_register(&register, res);
                self.set_flag(res.0 == 0);
            }
            Instruction::Jz(label) => {
                if self.flag {
                    self.goto_label(&label)?;
                }
            }
            Instruction::Jnz(label) => {
                if !self.flag {
                    self.goto_label(&label)?;
                }
            }
            Instruction::J(label) => self.goto_label(&label)?,
            Instruction::Set(register, k) => {
                self.modify_register(&register, Wrapping(*k));
            }
        };
        self.index += 1;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ProgramError> {
        let Some(line) = self.program.get(self.index) else {
            return Err(ProgramError::EndOfProgram);
        };

        match line {
            ProgramLine::Ins(instruction) => self.intepret_instruction(&instruction.clone())?,
            ProgramLine::Lbl(_) => {
                self.index += 1;
            }
        }

        Ok(())
    }
}
