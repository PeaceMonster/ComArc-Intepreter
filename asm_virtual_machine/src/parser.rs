use pest::Parser;
use pest_derive::Parser;

use crate::machine::{Instruction, Label, ProgramLine, Register};

#[derive(Parser)]
#[grammar = "./asm.pest"]
struct ASMProgramParser;

pub fn parse_program(file: &str) -> anyhow::Result<Vec<ProgramLine>> {
    let prg = ASMProgramParser::parse(Rule::Program, file)?;
    let mut result: Vec<ProgramLine> = Vec::new();

    for pair in prg {
        let ins = pair.into_inner().next().unwrap();

        let line = match ins.as_rule() {
            Rule::label => {
                let label = ins.into_inner().next().unwrap().as_str();
                ProgramLine::Lbl(Label(label.to_string()))
            }
            Rule::JumpIns => {
                let mut registers = ins.into_inner();
                let action = registers.next().unwrap().as_str();
                let label = registers.next().unwrap().as_str();
                let action = match action {
                    "J" => Instruction::J(Label(label.to_string())),
                    "JZ" => Instruction::Jz(Label(label.to_string())),
                    "JNZ" => Instruction::Jnz(Label(label.to_string())),
                    _ => unreachable!(),
                };
                ProgramLine::Ins(action)
            }
            Rule::UnIns => {
                let mut registers = ins.into_inner();
                let action = registers.next().unwrap().as_str();
                let reg: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let action = match action {
                    "ZERO" => Instruction::Zero(reg),
                    "INC" => Instruction::Inc(reg),
                    "DEC" => Instruction::Dec(reg),
                    "NOT" => Instruction::Not(reg),
                    _ => unreachable!(),
                };
                ProgramLine::Ins(action)
            }
            Rule::MovIns => {
                let mut registers = ins.into_inner();
                let dest: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let src: Register = registers.next().unwrap().as_str().try_into().unwrap();
                ProgramLine::Ins(Instruction::Mov(dest, src))
            }
            Rule::TriIns => {
                let mut registers = ins.into_inner();
                let action = registers.next().unwrap().as_str();
                let dest: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let op1: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let op2: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let action = match action {
                    "ADD" => Instruction::Add(dest, op1, op2),
                    "SUB" => Instruction::Sub(dest, op1, op2),
                    "AND" => Instruction::And(dest, op1, op2),
                    "OR" => Instruction::Or(dest, op1, op2),
                    "XOR" => Instruction::Xor(dest, op1, op2),
                    _ => unreachable!(),
                };
                ProgramLine::Ins(action)
            }
            Rule::ShiftIns => {
                let mut registers = ins.into_inner();
                let action = registers.next().unwrap().as_str();
                let reg: Register = registers.next().unwrap().as_str().try_into().unwrap();
                let k: u8 = registers.next().unwrap().as_str().parse().unwrap();
                let action = match action {
                    "SHL" => Instruction::Shl(reg, k),
                    "SHR" => Instruction::Shr(reg, k),
                    _ => unreachable!(),
                };
                ProgramLine::Ins(action)
            }
            _ => unreachable!(),
        };
        result.push(line);
    }
    Ok(result)
}
