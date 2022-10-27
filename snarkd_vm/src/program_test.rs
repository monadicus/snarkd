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
                destination: 2,
                values: vec![Value::Ref(0), Value::Ref(1)],
            })],
        }],
    }
}

fn zero_group() -> Value {
    Value::Group(Group::Single(Field {
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
                destination: 2,
                values: vec![
                    Value::Record(Box::new(Record {
                        owner: Data {
                            value: Value::Address(Address { address: vec![0] }),
                            visibility: Visibility::Constant,
                        },
                        gates: Data {
                            value: Value::Integer(Integer::U64(0)),
                            visibility: Visibility::Constant,
                        },
                        data: vec![Data {
                            value: Value::Boolean(false),
                            visibility: Visibility::Constant,
                        }],
                        nonce: Data {
                            value: zero_group(),
                            visibility: Visibility::Constant,
                        },
                    })),
                    Value::Record(Box::new(Record {
                        owner: Data {
                            value: Value::Address(Address { address: vec![0] }),
                            visibility: Visibility::Constant,
                        },
                        gates: Data {
                            value: Value::Integer(Integer::U64(0)),
                            visibility: Visibility::Constant,
                        },
                        data: vec![Data {
                            value: Value::Boolean(false),
                            visibility: Visibility::Constant,
                        }],
                        nonce: Data {
                            value: zero_group(),
                            visibility: Visibility::Constant,
                        },
                    })),
                ],
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
                destination: 2,
                values: vec![
                    Value::Record(Box::new(Record {
                        owner: Data {
                            value: zero_group(),
                            visibility: Visibility::Constant,
                        },
                        gates: Data {
                            value: Value::Integer(Integer::U64(0)),
                            visibility: Visibility::Constant,
                        },
                        data: Vec::new(),
                        nonce: Data {
                            value: zero_group(),
                            visibility: Visibility::Constant,
                        },
                    })),
                    Value::Record(Box::new(Record {
                        owner: Data {
                            value: Value::Address(Address { address: vec![0] }),
                            visibility: Visibility::Constant,
                        },
                        gates: Data {
                            value: Value::Integer(Integer::U64(0)),
                            visibility: Visibility::Constant,
                        },
                        data: Vec::new(),
                        nonce: Data {
                            value: zero_group(),
                            visibility: Visibility::Constant,
                        },
                    })),
                ],
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
                type_: Type::Struct(vec![("hello".into(), Type::Boolean)]),
            }],
            ..Default::default()
        },
        functions: vec![Function {
            argument_start_variable: 0,
            instructions: vec![Instruction::AssertEq(AssertData {
                values: vec![Value::Struct(vec![Value::Boolean(false)]), Value::Ref(0)],
            })],
        }],
    }
}

fn test_program(input: Program) {
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

#[test]
#[should_panic]
fn record_wrong_owner_type_test() {
    test_program(incorrect_record_owner_type())
}
