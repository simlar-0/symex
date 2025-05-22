#![allow(warnings)] // Until all the unimplemented!() are implemented

use std::fmt::Display;

use crate::{
    arch::{ArchError, Architecture, SupportedArchitecture},
    executor::{
        hooks::HookContainer,
        instruction::Instruction,
        state::GAState,
    },
    project::dwarf_helper::SubProgramMap,
};

pub mod decoder;
pub mod decoder_implementations;
pub mod timing;

#[derive(Debug, Default, Clone)]
pub struct RISCV {}

impl Architecture for RISCV {
    type ISA = ();
    
    fn translate<C: Composition>(&self, buff: &[u8], state: &GAState<C>) -> Result<Instruction<C>, ArchError> {
        unimplemented!();
    }

    fn add_hooks<C: Composition>(&self, hooks: &mut HookContainer<C>, sub_program_lookup: &mut SubProgramMap) {
        unimplemented!();
    }

    fn pre_instruction_loading_hook<C>(state: &mut GAState<C>)
    where
        C: Composition<ArchitectureOverride = Self>,
    {
        unimplemented!();
    }

    fn post_instruction_execution_hook<C>(state: &mut GAState<C>)
    where
        C: Composition<ArchitectureOverride = Self>,
    {
        unimplemented!();
    }

    fn initiate_state<C>(state: &mut GAState<C>)
    where
        C: Composition<ArchitectureOverride = Self>,
    {
        unimplemented!();
    }

    fn get_return_address_register_name<C>() -> String
    where
        C: Composition<ArchitectureOverride = Override>,
    {
        "X1".to_string()
    }
    
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

impl Display for RISCV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RISC-V")
    }
}

impl From<risc_v_disassembler::DisassemblerError> for ArchError{
    fn from(value: risc_v_disassembler::DisassemblerError) -> Self {
        unimplemented!()
    }
}

impl Into<SupportedArchitecture> for RISCV {
    fn into(self) -> SupportedArchitecture {
        SupportedArchitecture::RISCV(self)
    }
}
