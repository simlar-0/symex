#![allow(warnings)]

use risc_v_disassembler::{
    ParsedInstruction32,
    Register,
    SpecialRegister,
};

use general_assembly::{
    condition::{Condition, Comparison},
    operand::{DataWord, Operand},
    operation::Operation as GAOperation,
};

use super::RISCV;
use crate::executor::instruction::Instruction as GAInstruction;

impl RISCV {
    pub(super) fn expand<C: crate::Composition>(instr: &ParsedInstruction32) -> GAInstruction<C> {
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

// Make sure to change the shift implementation to 6-bits if you want to support RV64I
fn instruction_to_ga_operations(instr: &ParsedInstruction32) -> Vec<GAOperation> {
    match *instr {
        ParsedInstruction32::add { rd, rs1, rs2 } => {
            vec![
                GAOperation::Add {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: risc_v_register_to_ga_operand(&rs2),
                }
            ]
        }
        ParsedInstruction32::sub { rd, rs1, rs2 } => {
            vec![
                GAOperation::Sub {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: risc_v_register_to_ga_operand(&rs2),
                }
            ]
        }
        ParsedInstruction32::xor { rd, rs1, rs2 } => {
            vec![
                GAOperation::Xor {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: risc_v_register_to_ga_operand(&rs2),
                }
            ]
        }
        ParsedInstruction32::or { rd, rs1, rs2 } => {
            vec![
                GAOperation::Or {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: risc_v_register_to_ga_operand(&rs2),
                }
            ]
        }
        ParsedInstruction32::and { rd, rs1, rs2 } => {
            vec![
                GAOperation::And {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: risc_v_register_to_ga_operand(&rs2),
                }
            ]
        }
        ParsedInstruction32::sll { rd, rs1, rs2 } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: risc_v_register_to_ga_operand(&rs2),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Sl {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::srl { rd, rs1, rs2 } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: risc_v_register_to_ga_operand(&rs2),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Srl {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::sra { rd, rs1, rs2 } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: risc_v_register_to_ga_operand(&rs2),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Sra {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::slt { rd, rs1, rs2 } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: todo!(), // Need to implement signed lt comparison
                    then: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(1)),
                        }
                    ], 
                    otherwise: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(0)),
                        }
                    ], 
                }
            ]
        }
        ParsedInstruction32::sltu { rd, rs1, rs2 } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: Comparison::Lt,
                    then: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(1)),
                        }
                    ], 
                    otherwise: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(0)),
                        }
                    ], 
                }
            ]
        }
        ParsedInstruction32::addi { rd, rs1, imm } => {
            vec![
                GAOperation::Add {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                }
            ]
        }
        ParsedInstruction32::xori { rd, rs1, imm } => {
            vec![
                GAOperation::Xor {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                }
            ]
        }
        ParsedInstruction32::ori { rd, rs1, imm } => {
            vec![
                GAOperation::Or {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                }
            ]
        }
        ParsedInstruction32::andi { rd, rs1, imm } => {
            vec![
                GAOperation::And {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                }
            ]
        }
        ParsedInstruction32::slli { rd, rs1, shamt } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: Operand::Immediate(DataWord::Word8(shamt)),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Sl {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::srli { rd, rs1, shamt } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: Operand::Immediate(DataWord::Word8(shamt)),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Srl {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::srai { rd, rs1, shamt } => {
            vec![
                GAOperation::And {
                    destination: Operand::Local("shift".to_owned()),
                    operand1: Operand::Immediate(DataWord::Word8(shamt)),
                    operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
                },
                GAOperation::Sra {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand: risc_v_register_to_ga_operand(&rs1),
                    shift: Operand::Local("shift".to_owned()),
                },
            ]
        }
        ParsedInstruction32::slti { rd, rs1, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: Operand::Immediate(DataWord::Word32(imm as u32)),
                    operation: todo!(), // Need to implement signed lt comparison
                    then: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(1)),
                        }
                    ], 
                    otherwise: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(0)),
                        }
                    ], 
                }
            ]
        }
        ParsedInstruction32::sltiu { rd, rs1, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: Operand::Immediate(DataWord::Word32(imm as u32)),
                    operation: Comparison::Lt,
                    then: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(1)),
                        }
                    ], 
                    otherwise: vec![
                        GAOperation::Move {
                            destination: risc_v_register_to_ga_operand(&rd),
                            source: Operand::Immediate(DataWord::Word32(0)),
                        }
                    ], 
                }
            ]
        }
        ParsedInstruction32::lb { rd, rs1, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::AddressInLocal("addr".to_owned(), 8),
                }
            ]
        }
        ParsedInstruction32::lh { rd, rs1, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::AddressInLocal("addr".to_owned(), 16),
                }
            ]
        }
        ParsedInstruction32::lw { rd, rs1, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::AddressInLocal("addr".to_owned(), 32),
                }
            ]
        }
        ParsedInstruction32::lbu { rd, rs1, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::AddressInLocal("addr".to_owned(), 8),
                },
                GAOperation::ZeroExtend { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    operand: Operand::Local("addr".to_owned()), 
                    bits: 8, 
                    target_bits: 32
                }
            ]
        }
        ParsedInstruction32::lhu { rd, rs1, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::AddressInLocal("addr".to_owned(), 8),
                },
                GAOperation::ZeroExtend { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    operand: Operand::Local("addr".to_owned()), 
                    bits: 16, 
                    target_bits: 32
                }
            ]
        }
        ParsedInstruction32::sb { rs1, rs2, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: Operand::AddressInLocal("addr".to_owned(), 8),
                    source: risc_v_register_to_ga_operand(&rs2), 
                }
            ]
        }
        ParsedInstruction32::sh { rs1, rs2, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: Operand::AddressInLocal("addr".to_owned(), 16),
                    source: risc_v_register_to_ga_operand(&rs2), 
                }
            ]
        }
        ParsedInstruction32::sw { rs1, rs2, imm } => {
            vec![
                GAOperation::Add { 
                    destination: Operand::Local("addr".to_owned()), 
                    operand1: risc_v_register_to_ga_operand(&rs1), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), 
                },
                GAOperation::Move { 
                    destination: Operand::AddressInLocal("addr".to_owned(), 32),
                    source: risc_v_register_to_ga_operand(&rs2), 
                }
            ]
        }
        ParsedInstruction32::beq { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: Comparison::Eq,
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::EQ ,
                },
            ]
        }
        ParsedInstruction32::bne { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: Comparison::Eq,
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::NE,
                },
            ]
        }
        ParsedInstruction32::blt { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: todo!(), // Need to implement signed lt comparison
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::EQ ,
                },
            ]
        }
        ParsedInstruction32::bge { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: todo!(), // Need to implement signed ge comparison
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::EQ ,
                },
            ]
        }
        ParsedInstruction32::bltu { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: Comparison::Lt,
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::EQ ,
                },
            ]
        }
        ParsedInstruction32::bgeu { rs1, rs2, imm } => {
            vec![
                GAOperation::Ite { 
                    lhs: risc_v_register_to_ga_operand(&rs1), 
                    rhs: risc_v_register_to_ga_operand(&rs2),
                    operation: Comparison::Geq,
                    then: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(0)))
                    ],
                    otherwise: vec![
                        GAOperation::SetZFlag(Operand::Immediate(DataWord::Word32(1)))
                    ],
                },
                GAOperation::Add {
                    destination: Operand::Local("new_pc".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
                GAOperation::ConditionalJump {
                    destination: Operand::Local("new_pc".to_owned()),
                    condition: Condition::EQ ,
                },
            ]
        }
        ParsedInstruction32::jal { rd, imm } => {
            vec![
                GAOperation::Add {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(4)),
                },
                GAOperation::Add {
                    destination: Operand::Register("PC".to_owned()),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
            ]
        }
        ParsedInstruction32::jalr { rd, rs1, imm } => {
            vec![
                GAOperation::Add {
                    destination: risc_v_register_to_ga_operand(&rd),
                    operand1: Operand::Register("PC".to_owned()),
                    operand2: Operand::Immediate(DataWord::Word32(4)),
                },
                GAOperation::Add {
                    destination: Operand::Register("PC".to_owned()),
                    operand1: risc_v_register_to_ga_operand(&rs1),
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)),
                },
            ]
        }
        ParsedInstruction32::lui { rd, imm } => {
            vec![
                GAOperation::Move { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    source: Operand::Immediate(DataWord::Word32(imm as u32)), // The disassmebler already shifts imm
                }
            ]
        }
        ParsedInstruction32::auipc { rd, imm } => {
            vec![
                GAOperation::Add { 
                    destination: risc_v_register_to_ga_operand(&rd), 
                    operand1: Operand::Register("PC".to_owned()), 
                    operand2: Operand::Immediate(DataWord::Word32(imm as u32)), // The disassmebler already shifts imm 
                },
            ]
        }
        ParsedInstruction32::ecall => {todo!();}
        ParsedInstruction32::ebreak => {todo!();}
    }
}

fn risc_v_register_to_ga_operand(reg: &Register) -> Operand {
    Operand::Register(match reg {
        Register::x0 => "X0".to_owned(),
        Register::x1 => "X1".to_owned(),
        Register::x2 => "X2".to_owned(),
        Register::x3 => "X3".to_owned(),
        Register::x4 => "X4".to_owned(),
        Register::x5 => "X5".to_owned(),
        Register::x6 => "X6".to_owned(),
        Register::x7 => "X7".to_owned(),
        Register::x8 => "X8".to_owned(),
        Register::x9 => "X9".to_owned(),
        Register::x10 => "X10".to_owned(),
        Register::x11 => "X11".to_owned(),
        Register::x12 => "X12".to_owned(),
        Register::x13 => "X13".to_owned(),
        Register::x14 => "X14".to_owned(),
        Register::x15 => "X15".to_owned(),
        Register::x16 => "X16".to_owned(),
        Register::x17 => "X17".to_owned(),
        Register::x18 => "X18".to_owned(),
        Register::x19 => "X19".to_owned(),
        Register::x20 => "X20".to_owned(),
        Register::x21 => "X21".to_owned(),
        Register::x22 => "X22".to_owned(),
        Register::x23 => "X23".to_owned(),
        Register::x24 => "X24".to_owned(),
        Register::x25 => "X25".to_owned(),
        Register::x26 => "X26".to_owned(),
        Register::x27 => "X27".to_owned(),
        Register::x28 => "X28".to_owned(),
        Register::x29 => "X29".to_owned(),
        Register::x30 => "X30".to_owned(),
        Register::x31 => "X31".to_owned(),
    })
}

fn risc_v_special_register_to_operand() -> Operand {
    todo!() // Must make special register public in RISC-V disassembler
}