//#![allow(warnings)]

pub mod decoder;
pub mod timing;

use std::fmt::Display;
use object::{File, Object};
use regex::Regex;
use tracing::trace;

use risc_v_disassembler::DisassemblerError;

use crate::{
    arch::{ArchError, Architecture, ParseError, SupportedArchitecture},
    executor::{hooks::PCHook, state::GAState},
    smt::{SmtExpr, SmtMap},
    trace,
};

#[derive(Clone, Copy, Debug)]
pub struct RISCV {}

impl Arch for RISCV {
    fn add_hooks(&self, cfg: &mut RunConfig<Self>) {
        todo!()
    }

    fn translate(&self, buff: &[u8], _state: &GAState<Self>,) 
    -> Result<Instruction<Self>, ArchError> {
        todo!()
    }

    fn discover(file: &File<'_>) -> Result<Option<Self>, ArchError> {
        todo!()
    }
}

impl Display for RISCV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn map_error(e: DisassemblerError) -> ArchError {
    ArchError::ParsingError( match e {
        DisassemblerError::UnsupportedInstructionLength => ParseError::InsufficientInput,
        DisassemblerError::InvalidFunct3(_) => ParseError::MalformedInstruction,
        DisassemblerError::InvalidFunct7(_) => ParseError::MalformedInstruction,
        DisassemblerError::InvalidOpcode(_) => ParseError::InvalidInstruction,
        DisassemblerError::InvalidImmediate(_) => ParseError::MalformedInstruction,
        DisassemblerError::InvalidRegister(_) => ParseError::InvalidRegister,
        DisassemblerError::BitExtensionError(str) => ParseError::Generic(str),
        DisassemblerError::BitExtractionError(str) => ParseError::Generic(str),
    })
}