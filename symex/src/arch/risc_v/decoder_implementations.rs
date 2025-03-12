use risc_v_disassembler::{
    ParsedInstruction32,
    parsed_instructions,
    Register,
    SpecialRegister,
};

use general_assembly::{
    condition::{Condition, Comparison},
    operand::{DataWord, Operand},
    operation::Operation as GAOperation,
};

use super::{
    RISCV,
    decoder::risc_v_register_to_ga_operand,
};
use crate::executor::instruction::Instruction as GAInstruction;

pub(crate) trait Instruction32ToGAOperations {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation>;
}


impl Instruction32ToGAOperations for parsed_instructions::add {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: risc_v_register_to_ga_operand(&self.rs2),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sub {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Sub {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: risc_v_register_to_ga_operand(&self.rs2),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::xor {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Xor {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: risc_v_register_to_ga_operand(&self.rs2),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::or {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Or {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: risc_v_register_to_ga_operand(&self.rs2),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::and {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: risc_v_register_to_ga_operand(&self.rs2),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sll {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: risc_v_register_to_ga_operand(&self.rs2),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Sl {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::srl {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: risc_v_register_to_ga_operand(&self.rs2),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Srl {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sra {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: risc_v_register_to_ga_operand(&self.rs2),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Sra {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::slt {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::SLt,
                then: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(1)),
                    }
                ], 
                otherwise: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(0)),
                    }
                ], 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sltu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::ULt,
                then: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(1)),
                    }
                ], 
                otherwise: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(0)),
                    }
                ], 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::addi {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::xori {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Xor {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::ori {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Or {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::andi {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::slli {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: Operand::Immediate(DataWord::Word8(self.shamt)),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Sl {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::srli {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: Operand::Immediate(DataWord::Word8(self.shamt)),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Srl {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::srai {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::And {
                destination: Operand::Local("shift".to_owned()),
                operand1: Operand::Immediate(DataWord::Word8(self.shamt)),
                operand2: Operand::Immediate(DataWord::Word32(0x1f)), // 5-bits
            },
            GAOperation::Sra {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand: risc_v_register_to_ga_operand(&self.rs1),
                shift: Operand::Local("shift".to_owned()),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::slti {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: Operand::Immediate(DataWord::Word32(self.imm as u32)),
                operation: Comparison::SLt,
                then: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(1)),
                    }
                ], 
                otherwise: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(0)),
                    }
                ], 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sltiu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: Operand::Immediate(DataWord::Word32(self.imm as u32)),
                operation: Comparison::ULt,
                then: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(1)),
                    }
                ], 
                otherwise: vec![
                    GAOperation::Move {
                        destination: risc_v_register_to_ga_operand(&self.rd),
                        source: Operand::Immediate(DataWord::Word32(0)),
                    }
                ], 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lb {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::AddressInLocal("addr".to_owned(), 8),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lh {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::AddressInLocal("addr".to_owned(), 16),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lw {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::AddressInLocal("addr".to_owned(), 32),
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lbu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::AddressInLocal("addr".to_owned(), 8),
            },
            GAOperation::ZeroExtend { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                operand: Operand::Local("addr".to_owned()), 
                bits: 8, 
                target_bits: 32
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lhu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::AddressInLocal("addr".to_owned(), 8),
            },
            GAOperation::ZeroExtend { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                operand: Operand::Local("addr".to_owned()), 
                bits: 16, 
                target_bits: 32
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sb {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: Operand::AddressInLocal("addr".to_owned(), 8),
                source: risc_v_register_to_ga_operand(&self.rs2), 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sh {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: Operand::AddressInLocal("addr".to_owned(), 16),
                source: risc_v_register_to_ga_operand(&self.rs2), 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::sw {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: Operand::Local("addr".to_owned()), 
                operand1: risc_v_register_to_ga_operand(&self.rs1), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), 
            },
            GAOperation::Move { 
                destination: Operand::AddressInLocal("addr".to_owned(), 32),
                source: risc_v_register_to_ga_operand(&self.rs2), 
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::beq {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::EQ ,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::bne {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::NE,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::blt {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::SLt,
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::EQ ,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::bge {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::SGt,
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::EQ ,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::bltu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::ULt,
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::EQ ,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::bgeu {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Ite { 
                lhs: risc_v_register_to_ga_operand(&self.rs1), 
                rhs: risc_v_register_to_ga_operand(&self.rs2),
                operation: Comparison::UGeq,
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
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
            GAOperation::ConditionalJump {
                destination: Operand::Local("new_pc".to_owned()),
                condition: Condition::EQ ,
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::jal {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: Operand::Register("PC".to_owned()),
                operand2: Operand::Immediate(DataWord::Word32(4)),
            },
            GAOperation::Add {
                destination: Operand::Register("PC".to_owned()),
                operand1: Operand::Register("PC".to_owned()),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::jalr {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add {
                destination: risc_v_register_to_ga_operand(&self.rd),
                operand1: Operand::Register("PC".to_owned()),
                operand2: Operand::Immediate(DataWord::Word32(4)),
            },
            GAOperation::Add {
                destination: Operand::Register("PC".to_owned()),
                operand1: risc_v_register_to_ga_operand(&self.rs1),
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)),
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::lui {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Move { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                source: Operand::Immediate(DataWord::Word32(self.imm as u32)), // The disassmebler already shifts imm
            }
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::auipc {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Add { 
                destination: risc_v_register_to_ga_operand(&self.rd), 
                operand1: Operand::Register("PC".to_owned()), 
                operand2: Operand::Immediate(DataWord::Word32(self.imm as u32)), // The disassmebler already shifts imm 
            },
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::ecall {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Abort {error:"ecall requires external system modelling".to_string()}
        ]
    }
}

impl Instruction32ToGAOperations for parsed_instructions::ebreak {
    fn instruction_to_ga_operations(&self, instr: &ParsedInstruction32) -> Vec<GAOperation> {
        vec![
            GAOperation::Abort {error:"ebreak requires external system modelling".to_string()}
        ]
    }
}