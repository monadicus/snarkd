use super::*;

fn example_header() -> Header {
    Header {
        private_inputs: vec![InputType {
            variable: 0,
            name: "a".into(),
            type_: Type::U8,
        }],
        constant_inputs: vec![InputType {
            variable: 1,
            name: "b".into(),
            type_: Type::U8,
        }],
        public_inputs: vec![InputType {
            variable: 2,
            name: "c".into(),
            type_: Type::U8,
        }],
        public_states: vec![InputType {
            variable: 3,
            name: "d".into(),
            type_: Type::U8,
        }],
        private_record_states: vec![InputType {
            variable: 4,
            name: "e".into(),
            type_: Type::U8,
        }],
        private_leaf_states: vec![InputType {
            variable: 5,
            name: "f".into(),
            type_: Type::U8,
        }],
        ..Default::default()
    }
}

fn example_program() -> Program {
    Program {
        header: example_header(),
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::Add(BinaryData {
                lhs: Operand::Ref(0),
                rhs: Operand::Ref(1),
                dest: 2,
            })],
        }],
    }
}

fn zero_group() -> Group {
    Group::Single(Field {
        negate: false,
        values: vec![0],
    })
}

fn example_basic_record_program() -> Program {
    Program {
        header: example_header(),
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::IsEq(BinaryData {
                lhs: Operand::Record(Box::new(Record {
                    owner: Address { address: vec![0] },
                    owner_visibility: Visibility::Constant,
                    gates: 0,
                    gates_visibility: Visibility::Constant,
                    data: vec![VisibleData {
                        value: Operand::Boolean(false),
                        visibility: Visibility::Constant,
                    }],
                    nonce: zero_group(),
                    nonce_visibility: Visibility::Constant,
                })),
                rhs: Operand::Record(Box::new(Record {
                    owner: Address { address: vec![0] },
                    owner_visibility: Visibility::Constant,
                    gates: 0,
                    gates_visibility: Visibility::Constant,
                    data: vec![VisibleData {
                        value: Operand::Boolean(false),
                        visibility: Visibility::Constant,
                    }],
                    nonce: zero_group(),
                    nonce_visibility: Visibility::Constant,
                })),
                dest: 2,
            })],
        }],
    }
}

fn struct_program() -> Program {
    Program {
        header: Header {
            private_inputs: vec![InputType {
                variable: 0,
                name: "a".into(),
                type_: Type::Struct(StructType {
                    fields: vec![StructTypeEntry {
                        name: "hello".into(),
                        type_: Type::Boolean,
                    }],
                }),
            }],
            ..Default::default()
        },
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::AssertEq(AssertData {
                lhs: Operand::Struct(Struct {
                    values: vec![Operand::Boolean(false)],
                }),
                rhs: Operand::Ref(0),
            })],
        }],
    }
}

fn test_program(mut input: Program) {
    let bytes = input.serialize().unwrap();
    let output = Program::deserialize(&bytes).unwrap();
    assert_eq!(input, output);
}

#[test]
fn encode_decode_test() {
    test_program(example_program())
}

#[test]
fn struct_test() {
    test_program(struct_program())
}

#[test]
fn basic_record_test() {
    test_program(example_basic_record_program())
}
