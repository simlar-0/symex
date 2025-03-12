//#![allow(warnings)]

pub mod decoder;
pub mod timing;

use std::fmt::Display;
use object::{File, Object};
use regex::Regex;
use tracing::trace;

use crate::{
    elf_util::{ExpressionType, Variable},
    general_assembly::{
        arch::{Arch, ArchError, ParseError},
        instruction::Instruction,
        project::{MemoryHookAddress, MemoryReadHook, PCHook, RegisterReadHook, RegisterWriteHook},
        state::GAState,
        RunConfig,
    },
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