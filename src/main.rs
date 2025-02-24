use std::ops::{BitAnd, BitOr, BitXor};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Register {
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
struct Label(String);

#[derive(Debug, Clone)]
enum Instruction {
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
}

#[derive(Debug, Clone)]
enum ProgramLine {
    Ins(Instruction),
    Lbl(Label),
}

struct Machine {
    registers: Vec<(Register, i8)>,
    flag: bool,
    program: Vec<ProgramLine>,
    index: usize,
}

impl Machine {
    fn init_registers() -> Vec<(Register, i8)> {
        vec![
            (Register::R0, 0),
            (Register::R1, 0),
            (Register::R2, 0),
            (Register::R3, 0),
            (Register::R4, 0),
            (Register::R5, 0),
            (Register::R6, 0),
            (Register::R7, 0),
        ]
    }

    fn new() -> Machine {
        Machine {
            registers: Machine::init_registers(),
            flag: false,
            program: Vec::new(),
            index: 0,
        }
    }

    fn init_program(&mut self, program: Vec<ProgramLine>) {
        self.program = program;
        self.index = 0;
    }

    fn get_register(&self, r: &Register) -> i8 {
        let res = self
            .registers
            .iter()
            .filter(|re| re.0 == *r)
            .nth(0)
            .unwrap();
        res.1
    }

    fn modify_register(&mut self, r: &Register, value: i8) {
        let reg_pos = self.registers.iter().position(|re| re.0 == *r).unwrap();
        let reg = self.registers.get_mut(reg_pos).unwrap();
        reg.1 = value;
    }

    fn goto_label(&mut self, lbl: &Label) {
        let label_pos = self
            .program
            .iter()
            .position(|pl| match pl {
                ProgramLine::Ins(_) => false,
                ProgramLine::Lbl(label) => label.0.eq(&lbl.0),
            })
            .unwrap();
        self.index = label_pos;
    }

    fn set_flag(&mut self, value: bool) {
        self.flag = value;
    }

    fn intepret_instruction(&mut self, ins: &Instruction) {
        match ins {
            Instruction::Zero(register) => self.modify_register(&register, 0),
            Instruction::Mov(register, register1) => {
                let val = self.get_register(&register1);
                self.modify_register(&register, val);
            }
            Instruction::Add(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let (res, _) = val1.overflowing_add(val2);
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Sub(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let (res, _) = val1.overflowing_sub(val2);
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Inc(register) => {
                let val = self.get_register(&register);
                self.modify_register(&register, val.overflowing_add(1).0);
                self.set_flag(val == 0);
            }
            Instruction::Dec(register) => {
                let val = self.get_register(&register);
                self.modify_register(&register, val.overflowing_sub(1).0);
                self.set_flag(val == 0);
            }
            Instruction::And(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitand(val2);
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Or(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitor(val2);
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Xor(register, register1, register2) => {
                let val1 = self.get_register(&register1);
                let val2 = self.get_register(&register2);
                let res = val1.bitxor(val2);
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Not(register) => {
                let val = self.get_register(&register);
                let res = !val;
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Shl(register, k) => {
                let val = self.get_register(&register);
                let res = val.overflowing_shl(*k as u32).0;
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Shr(register, k) => {
                let val = self.get_register(&register);
                let res = val.overflowing_shl(*k as u32).0;
                self.modify_register(&register, res);
                self.set_flag(res == 0);
            }
            Instruction::Jz(label) => {
                if self.flag {
                    self.goto_label(&label);
                }
            }
            Instruction::Jnz(label) => {
                if !self.flag {
                    self.goto_label(&label);
                }
            }
            Instruction::J(label) => self.goto_label(&label),
        };
        self.index += 1;
    }

    fn step(&mut self) -> Result<(), String> {
        let Some(line) = self.program.get(self.index) else {
            return Err("End of program".to_owned());
        };

        match line {
            ProgramLine::Ins(instruction) => self.intepret_instruction(&instruction.clone()),
            ProgramLine::Lbl(_) => {}
        }

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
