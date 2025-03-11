#![allow(warnings)]

use risc_v_disassembler::{
    ParsedInstruction32,
    Register,
};

use general_assembly::{
    condition::Condition,
    operand::{DataWord, Operand},
    operation::Operation as GAOperation,
};

use super::RISCV;
use crate::general_assembly::instruction::Instruction as GAInstruction;

impl RISCV {
    pub(super) fn expand(instr: &ParsedInstruction32) -> GAInstruction<RISCV> {
        let instruction_size: u32 = 32;
        let max_cycle= todo!();
        let memory_access: bool = todo!();

        let operations = instruction_to_ga_operations(instr);

        GAInstruction {
            instruction_size,
            max_cycle,
            memory_access,
            operations,
        }
    }
}

fn instruction_to_ga_operations(instr: &ParsedInstruction32) -> Vec<GAOperation> {
    match *instr {
        ParsedInstruction32::add { rd, rs1, rs2 } => {

        }
        ParsedInstruction32::sub { rd, rs1, rs2 } => {
            
        }
        ParsedInstruction32::xor { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::or { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::and { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::sll { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::srl { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::sra { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::slt { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::sltu { rd, rs1, rs2 } => {todo!();}
        ParsedInstruction32::addi { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::xori { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::ori { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::andi { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::slli { rd, rs1, shamt } => {todo!();}
        ParsedInstruction32::srli { rd, rs1, shamt } => {todo!();}
        ParsedInstruction32::srai { rd, rs1, shamt } => {todo!();}
        ParsedInstruction32::slti { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::sltiu { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lb { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lh { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lw { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lbu { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lhu { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::sb { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::sh { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::sw { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::beq { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::bne { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::blt { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::bge { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::bltu { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::bgeu { rs1, rs2, imm } => {todo!();}
        ParsedInstruction32::jal { rd, imm } => {todo!();}
        ParsedInstruction32::jalr { rd, rs1, imm } => {todo!();}
        ParsedInstruction32::lui { rd, imm } => {todo!();}
        ParsedInstruction32::auipc { rd, imm } => {todo!();}
        ParsedInstruction32::ecall => {todo!();}
        ParsedInstruction32::ebreak => {todo!();}
    };
}

fn risc_v_register_to_ga_operand(reg: &Register) -> Operand {
    Operand::Register(match reg {
        Register::x0 => "R0".to_owned(),
        Register::x1 => "R1".to_owned(),
        Register::x2 => "R2".to_owned(),
        Register::x3 => "R3".to_owned(),
        Register::x4 => "R4".to_owned(),
        Register::x5 => "R5".to_owned(),
        Register::x6 => "R6".to_owned(),
        Register::x7 => "R7".to_owned(),
        Register::x8 => "R8".to_owned(),
        Register::x9 => "R9".to_owned(),
        Register::x10 => "R10".to_owned(),
        Register::x11 => "R11".to_owned(),
        Register::x12 => "R12".to_owned(),
        Register::x13 => "R13".to_owned(),
        Register::x14 => "R14".to_owned(),
        Register::x15 => "R15".to_owned(),
        Register::x16 => "R16".to_owned(),
        Register::x17 => "R17".to_owned(),
        Register::x18 => "R18".to_owned(),
        Register::x19 => "R19".to_owned(),
        Register::x20 => "R20".to_owned(),
        Register::x21 => "R21".to_owned(),
        Register::x22 => "R22".to_owned(),
        Register::x23 => "R23".to_owned(),
        Register::x24 => "R24".to_owned(),
        Register::x25 => "R25".to_owned(),
        Register::x26 => "R26".to_owned(),
        Register::x27 => "R27".to_owned(),
        Register::x28 => "R28".to_owned(),
        Register::x29 => "R29".to_owned(),
        Register::x30 => "R30".to_owned(),
        Register::x31 => "R31".to_owned(),
    })
}

fn risc_v_special_register_to_operand() -> Operand {
    todo!() // Must make special register public in RISC-V disassembler
}