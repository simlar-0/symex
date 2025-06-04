#[cfg(test)]
mod tests {
    use disarmv7::prelude::{operation::*, *};
    use general_assembly::{
        operand::{DataWord, Operand},
        operation::Operation as GAOperation,
    };
    use hashbrown::HashMap;

    use crate::{
        arch::{risc_v::decoder::InstructionToGAOperations, Architecture, NoArchitectureOverride, RISCV},
        defaults::boolector::DefaultCompositionNoLogger,
        executor::{
            hooks::HookContainer,
            instruction::{CycleCount, Instruction},
            state::GAState,
            vm::VM,
            GAExecutor, ResultOrTerminate,
        },
        logging::NoLogger,
        path_selection::PathSelector,
        project::{dwarf_helper::SubProgramMap, Project},
        smt::smt_boolector::Boolector,
        smt::{bitwuzla::Bitwuzla, SmtExpr, SmtSolver},
        Endianness, WordSize,
    };

    struct TestRegister {
        name: &'static str,
        initial_value: u32,
        expected_value: u32,
    }
    struct TestData {
        instruction_bytes: [u8; 4],
        register1: TestRegister,
        register2: Option<TestRegister>,
    }

    fn setup_test_vm() -> VM<DefaultCompositionNoLogger> {
        let ctx = Boolector::new();
        let project_global = Box::new(Project::manual_project(vec![], 0, 0, WordSize::Bit32, Endianness::Little, HashMap::new()));
        let project: &'static Project = Box::leak(project_global);
        let mut hooks = HookContainer::new();
        RISCV {}.add_hooks(&mut hooks, &mut SubProgramMap::empty());
        let state = GAState::<DefaultCompositionNoLogger>::create_test_state(
            project,
            ctx.clone(),
            ctx.clone(),
            0,
            0,
            hooks,
            (),
            crate::arch::SupportedArchitecture::RISCV(<RISCV as Architecture<NoArchitectureOverride>>::new()),
        );
        VM::new_test_vm(project, state, NoLogger).unwrap()
    }

    fn initiate_test_register(executor: &mut GAExecutor<'_, DefaultCompositionNoLogger>, reg: &str, value: u32) {
        let register_operand = Operand::Register(reg.to_string());
        let immediate_operand = Operand::Immediate(DataWord::Word32(value));
        let operation = general_assembly::operation::Operation::Move {
            destination: register_operand,
            source: immediate_operand,
        };
        executor.execute_operation(&operation, &mut crate::logging::NoLogger).expect("Malformed test");
    }

    fn run_test<'a>(vm: &'a mut VM<DefaultCompositionNoLogger>, instruction_bytes: [u8; 4], test_data: &'a TestData) -> (GAState<DefaultCompositionNoLogger>) {
        let project = vm.project;

        let mut state = vm.paths.get_path().unwrap().state;

        let instruction = RISCV {}.translate(&instruction_bytes, &state.clone()).expect("Failed to translate instruction");
        let mut executor = GAExecutor::from_state(state, vm, project);

        initiate_test_register(&mut executor, test_data.register1.name, test_data.register1.initial_value);
        if let Some(register2) = &test_data.register2 {
            initiate_test_register(&mut executor, register2.name, register2.initial_value);
        }
        executor.execute_instruction(&instruction, &mut crate::logging::NoLogger);

        executor.state
    }

    fn assert_results(test_data: &TestData, final_state: &mut GAState<DefaultCompositionNoLogger>) {
        let reg1_value = final_state.get_register(test_data.register1.name).expect("Register not found");
        assert_eq!(
            reg1_value.get_constant().unwrap(),
            test_data.register1.expected_value as u64,
            "Register {} did not match expected value",
            test_data.register1.name
        );

        if let Some(register2) = &test_data.register2 {
            let reg2_value = final_state.get_register(register2.name).expect("Register not found");
            assert_eq!(
                reg2_value.get_constant().unwrap(),
                register2.expected_value as u64,
                "Register {} did not match expected value",
                register2.name
            );
        }
    }

    #[test]
    fn test_add() {
        let test_data = TestData {
            instruction_bytes: 0x00B50533u32.to_le_bytes(), // Example instruction bytes for an ADDI
            register1: TestRegister {
                name: "A0",
                initial_value: 0x01,
                expected_value: 0x02,
            },
            register2: Some(TestRegister {
                name: "A1",
                initial_value: 0x01,
                expected_value: 0x01,
            }),
        };

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data);

        assert_results(&test_data, &mut final_state);
    }
}
