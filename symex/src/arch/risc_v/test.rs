#[cfg(test)]
mod tests {
    use general_assembly::operand::{DataWord, Operand};
    use hashbrown::HashMap;

    use crate::{
        arch::{Architecture, NoArchitectureOverride, RISCV},
        defaults::boolector::DefaultCompositionNoLogger,
        executor::{hooks::HookContainer, state::GAState, vm::VM, GAExecutor},
        logging::NoLogger,
        path_selection::PathSelector,
        project::{dwarf_helper::SubProgramMap, Project},
        smt::smt_boolector::Boolector,
        smt::SmtSolver,
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
        register3: Option<TestRegister>,
    }

    macro_rules! generate_test_data {
        // Three registers
        (
        $inst:expr,
        ($reg1_name:expr, $reg1_initial:expr, $reg1_expected:expr),
        ($reg2_name:expr, $reg2_initial:expr, $reg2_expected:expr),
        ($reg3_name:expr, $reg3_initial:expr, $reg3_expected:expr)
    ) => {{
            TestData {
                instruction_bytes: $inst,
                register1: TestRegister {
                    name: $reg1_name,
                    initial_value: $reg1_initial,
                    expected_value: $reg1_expected,
                },
                register2: Some(TestRegister {
                    name: $reg2_name,
                    initial_value: $reg2_initial,
                    expected_value: $reg2_expected,
                }),
                register3: Some(TestRegister {
                    name: $reg3_name,
                    initial_value: $reg3_initial,
                    expected_value: $reg3_expected,
                }),
            }
        }};

        // Two registers
        (
        $inst:expr,
        ($reg1_name:expr, $reg1_initial:expr, $reg1_expected:expr),
        ($reg2_name:expr, $reg2_initial:expr, $reg2_expected:expr)
    ) => {{
            TestData {
                instruction_bytes: $inst,
                register1: TestRegister {
                    name: $reg1_name,
                    initial_value: $reg1_initial,
                    expected_value: $reg1_expected,
                },
                register2: Some(TestRegister {
                    name: $reg2_name,
                    initial_value: $reg2_initial,
                    expected_value: $reg2_expected,
                }),
                register3: None,
            }
        }};

        // One register
        (
        $inst:expr,
        ($reg1_name:expr, $reg1_initial:expr, $reg1_expected:expr)
    ) => {{
            TestData {
                instruction_bytes: $inst,
                register1: TestRegister {
                    name: $reg1_name,
                    initial_value: $reg1_initial,
                    expected_value: $reg1_expected,
                },
                register2: None,
                register3: None,
            }
        }};
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

    fn run_test<'a>(vm: &'a mut VM<DefaultCompositionNoLogger>, instruction_bytes: [u8; 4], test_data: &'a TestData) -> (GAExecutor<'a, DefaultCompositionNoLogger>) {
        let project = vm.project;

        let mut state = vm.paths.get_path().unwrap().state;

        let instruction = RISCV {}.translate(&instruction_bytes, &state.clone()).expect("Failed to translate instruction");
        let mut executor = GAExecutor::from_state(state, vm, project);

        initiate_test_register(&mut executor, test_data.register1.name, test_data.register1.initial_value);
        if let Some(register2) = &test_data.register2 {
            initiate_test_register(&mut executor, register2.name, register2.initial_value);
        }
        if let Some(register3) = &test_data.register3 {
            initiate_test_register(&mut executor, register3.name, register3.initial_value);
        }
        executor.execute_instruction(&instruction, &mut crate::logging::NoLogger);

        executor
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

        if let Some(register3) = &test_data.register3 {
            let reg3_value = final_state.get_register(register3.name).expect("Register not found");
            assert_eq!(
                reg3_value.get_constant().unwrap(),
                register3.expected_value as u64,
                "Register {} did not match expected value",
                register3.name
            );
        }
    }

    #[test]
    fn test_add() {
        let test_data = generate_test_data!(0x00B50533u32.to_le_bytes(), ("A0", 0x01, 0x02), ("A1", 0x01, 0x01));
        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sub() {
        let test_data = generate_test_data!(0x40B50533u32.to_le_bytes(), ("A0", 25, 0x06), ("A1", 19, 19));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_slt() {
        let test_data = generate_test_data!(0x00B52533u32.to_le_bytes(), ("A0", (-25i32) as u32, 1), ("A1", 5, 5));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sltu() {
        let test_data = generate_test_data!(0x00B53533u32.to_le_bytes(), ("A0", 3, 1), ("A1", 5, 5));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sltu_signed() {
        let test_data = generate_test_data!(0x00B53533u32.to_le_bytes(), ("A0", (-25i32) as u32, 0), ("A1", 5, 5));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_xor() {
        let test_data = generate_test_data!(0x00B54533u32.to_le_bytes(), ("A0", 13, 21), ("A1", 24, 24));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_or() {
        let test_data = generate_test_data!(0x00B56533u32.to_le_bytes(), ("A0", 0b0110111, 0b0111111), ("A1", 0b0001111, 0b0001111));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_and() {
        let test_data = generate_test_data!(0x00B57533u32.to_le_bytes(), ("A0", 0b0110111, 0b0000111), ("A1", 0b0001111, 0b0001111));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_srl() {
        let test_data = generate_test_data!(0x00B55533u32.to_le_bytes(), ("A0", 0b01111001, 0b00011110), ("A1", 0x02, 0x02));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sra_leading_0() {
        let test_data = generate_test_data!(0x40B55533u32.to_le_bytes(), ("A0", 0b01111001, 0b00011110), ("A1", 0x02, 0x02));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sra_leading_1() {
        let test_data = generate_test_data!(0x40B55533u32.to_le_bytes(), ("A0", 0xf0000000, 0xffffffff), ("A1", 31, 31));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sll() {
        let test_data = generate_test_data!(0x00B51533u32.to_le_bytes(), ("A0", 0b01111001, 0x1e400000), ("A1", 22, 22));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_addi() {
        let test_data = generate_test_data!(0x00A50513u32.to_le_bytes(), ("A0", 0x01, 0x01 + 10));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_slti() {
        let test_data = generate_test_data!(0x00A52513u32.to_le_bytes(), ("A0", (-25i32) as u32, 1));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sltiu() {
        let test_data = generate_test_data!(0x00A53513u32.to_le_bytes(), ("A0", 3, 1));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_sltiu_signed() {
        let test_data = generate_test_data!(0x00A53513u32.to_le_bytes(), ("A0", (-25i32) as u32, 0));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_xori() {
        let test_data = generate_test_data!(0x00A54513u32.to_le_bytes(), ("A0", 0xf12, 0xf18));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_ori() {
        let test_data = generate_test_data!(0x00f56513u32.to_le_bytes(), ("A0", 0b0110111, 0b0111111));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }

    #[test]
    fn test_andi() {
        let test_data = generate_test_data!(0x00f57513u32.to_le_bytes(), ("A0", 0b0110111, 0b0000111));

        let mut vm = setup_test_vm();
        let mut final_state = run_test(&mut vm, test_data.instruction_bytes, &test_data).state;

        assert_results(&test_data, &mut final_state);
    }
}
