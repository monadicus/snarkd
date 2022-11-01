use super::*;

fn example_header() -> Header {
    Header {
        main_inputs: vec![Input {
            variable: 0,
            name: "a".into(),
            type_: Type::U8,
        }],
        constant_inputs: vec![Input {
            variable: 1,
            name: "b".into(),
            type_: Type::U8,
        }],
        register_inputs: vec![Input {
            variable: 2,
            name: "c".into(),
            type_: Type::U8,
        }],
        public_states: vec![Input {
            variable: 3,
            name: "d".into(),
            type_: Type::U8,
        }],
        private_record_states: vec![Input {
            variable: 4,
            name: "e".into(),
            type_: Type::U8,
        }],
        private_leaf_states: vec![Input {
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

fn zero_group() -> Operand {
    Operand::Group(Group::Single(Field {
        negate: false,
        values: vec![0],
    }))
}

fn example_basic_record_program() -> Program {
    Program {
        header: example_header(),
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::IsEq(BinaryData {
                lhs: Operand::Record(Box::new(Record {
                    owner: VisibleData {
                        value: Operand::Address(Address { address: vec![0] }),
                        visibility: Visibility::Constant,
                    },
                    gates: VisibleData {
                        value: Operand::U64(0),
                        visibility: Visibility::Constant,
                    },
                    data: vec![VisibleData {
                        value: Operand::Boolean(false),
                        visibility: Visibility::Constant,
                    }],
                    nonce: VisibleData {
                        value: zero_group(),
                        visibility: Visibility::Constant,
                    },
                })),
                rhs: Operand::Record(Box::new(Record {
                    owner: VisibleData {
                        value: Operand::Address(Address { address: vec![0] }),
                        visibility: Visibility::Constant,
                    },
                    gates: VisibleData {
                        value: Operand::U64(0),
                        visibility: Visibility::Constant,
                    },
                    data: vec![VisibleData {
                        value: Operand::Boolean(false),
                        visibility: Visibility::Constant,
                    }],
                    nonce: VisibleData {
                        value: zero_group(),
                        visibility: Visibility::Constant,
                    },
                })),
                dest: 2,
            })],
        }],
    }
}

fn incorrect_record_owner_type() -> Program {
    Program {
        header: example_header(),
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::IsEq(BinaryData {
                lhs: Operand::Record(Box::new(Record {
                    owner: VisibleData {
                        value: zero_group(),
                        visibility: Visibility::Constant,
                    },
                    gates: VisibleData {
                        value: Operand::U64(0),
                        visibility: Visibility::Constant,
                    },
                    data: Vec::new(),
                    nonce: VisibleData {
                        value: zero_group(),
                        visibility: Visibility::Constant,
                    },
                })),
                rhs: Operand::Record(Box::new(Record {
                    owner: VisibleData {
                        value: Operand::Address(Address { address: vec![0] }),
                        visibility: Visibility::Constant,
                    },
                    gates: VisibleData {
                        value: Operand::U64(0),
                        visibility: Visibility::Constant,
                    },
                    data: Vec::new(),
                    nonce: VisibleData {
                        value: zero_group(),
                        visibility: Visibility::Constant,
                    },
                })),
                dest: 2,
            })],
        }],
    }
}

fn struct_program() -> Program {
    Program {
        header: Header {
            main_inputs: vec![Input {
                variable: 0,
                name: "a".into(),
                type_: Type::Struct(StructType {
                    subtypes: vec![(Type::Boolean)],
                    subtype_names: vec!["hello".into()],
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

// TODO this restriction should be conveyed in the protobuf
#[test]
#[should_panic]
fn record_wrong_owner_type_test() {
    test_program(incorrect_record_owner_type())
}
