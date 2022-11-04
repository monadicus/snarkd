use super::*;

#[derive(Clone, Debug)]
pub struct EvaluatorState<'a, F: Field, G: Group> {
    pub program: &'a Program,
    pub variables: IndexMap<u32, ConstrainedValue<F, G>>,
    pub call_depth: u32,
    pub parent_variables: IndexMap<u32, ConstrainedValue<F, G>>,
    pub function_index: u32,
    pub instruction_index: u32,
    pub namespace_id: usize,
}

impl<'a, F: Field, G: Group> EvaluatorState<'a, F, G> {
    /// creates a new state with no information other than a reference to the program its evaluating
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            variables: IndexMap::new(),
            call_depth: 0,
            parent_variables: IndexMap::new(),
            function_index: 0,
            instruction_index: 0,
            namespace_id: 0,
        }
    }

    pub fn cs<'b, CS: ConstraintSystem<F>>(
        &'b mut self,
        cs: &'b mut CS,
    ) -> impl ConstraintSystem<F> + 'b {
        let function_index = self.function_index;
        let instruction_index = self.instruction_index;
        let namespace_id = self.namespace_id;
        cs.ns(move || {
            format!(
                "id#{}f#{}i#{}",
                namespace_id, function_index, instruction_index
            )
        })
    }

    pub fn cs_meta<'b, CS: ConstraintSystem<F>>(
        &'b mut self,
        meta: &str,
        cs: &'b mut CS,
    ) -> impl ConstraintSystem<F> + 'b {
        let function_index = self.function_index;
        let instruction_index = self.instruction_index;
        let namespace_id = self.namespace_id;
        cs.ns(move || {
            format!(
                "id#{}f#{}i#{}: {}",
                namespace_id, function_index, instruction_index, meta
            )
        })
    }

    fn allocate_input<CS2: ConstraintSystem<F>>(
        cs: &mut CS2,
        type_: &Type,
        name: &str,
        value: Operand,
    ) -> Result<ConstrainedValue<F, G>, ValueError> {
        Ok(match type_ {
            Type::Address => {
                ConstrainedAddress::from_input(&mut cs.ns(|| name.to_string()), name, value)?
            }
            Type::Boolean => bool_from_input(&mut cs.ns(|| name.to_string()), name, value)?,
            Type::Field => {
                ConstrainedField::from_input(&mut cs.ns(|| name.to_string()), name, value)?
            }
            // Type::Char => Char::from_input(&mut cs.ns(|| name.to_string()), name, value)?,
            Type::Group => match value {
                Operand::Group(g) => ConstrainedValue::Group(
                    // G::constant(&g)?.to_allocated(&mut cs.ns(|| name.to_string()))?,
                    todo!(),
                ),
                _ => {
                    // return Err(GroupError::invalid_group(
                    //     "expected group, didn't find group in input".to_string(),
                    // )
                    // .into());
                    return Err("expected group, didn't find group in input".into());
                }
            },
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::U128
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::I128 => todo!(),
            Type::Scalar => todo!(),
            Type::String => todo!(),
            Type::Struct(_) => todo!(),
            Type::Record(_) => todo!(),
        })
    }

    pub fn resolve<'b, CS: ConstraintSystem<F>>(
        &'b mut self,
        value: &Operand,
        cs: &'b mut CS,
    ) -> Result<Cow<'b, ConstrainedValue<F, G>>> {
        Ok(Cow::Owned(match value {
            Operand::Address(address) => {
                ConstrainedValue::Address(ConstrainedAddress::constant(address)?)
            }
            Operand::Boolean(value) => ConstrainedValue::Boolean(ConstrainedBool(*value)),
            Operand::Field(field) => {
                ConstrainedValue::Field(ConstrainedField::constant(cs, field)?)
            }
            Operand::Group(g) => ConstrainedValue::Group(todo!()),
            // TODO i think this is wrong since now aleo uses static strings
            Operand::String(_) => return Err(anyhow!("cannot have resolved control string")),
            Operand::Ref(i) => {
                return Ok(Cow::Borrowed(
                    self.variables
                        .get(i)
                        .or(self.parent_variables.get(i))
                        // .expect("reference to unknown variable")
                        .ok_or_else(|| anyhow!("reference to unknown variable"))?,
                ));
            }
            Operand::U8(_) => todo!(),
            Operand::U16(_) => todo!(),
            Operand::U32(_) => todo!(),
            Operand::U64(_) => todo!(),
            Operand::U128(_) => todo!(),
            Operand::I8(_) => todo!(),
            Operand::I16(_) => todo!(),
            Operand::I32(_) => todo!(),
            Operand::I64(_) => todo!(),
            Operand::I128(_) => todo!(),
            Operand::Scalar(_) => todo!(),
            Operand::Record(_) => todo!(),
            Operand::Struct(_) => todo!(),
        }))
    }

    pub fn store(&mut self, variable: u32, value: ConstrainedValue<F, G>) {
        self.variables.insert(variable, value);
    }

    pub fn handle_input_block<CS: ConstraintSystem<F>>(
        &mut self,
        name: &str,
        input_header: &[InputType],
        input_values: &[InputValue],
        cs: &mut CS,
    ) -> Result<()> {
        let mut cs = cs.ns(|| format!("input {}", name));
        // for ir_input in input_header {
        //     let value = input_values
        //         .get(&ir_input.name)
        //         .ok_or_else(|| anyhow!("missing input value for '{}'", ir_input.name))?;
        //     if !value.matches_input_type(&ir_input.type_) {
        //         return Err(anyhow!(
        //             "type mismatch for input '{}', expected {}",
        //             ir_input.name,
        //             ir_input.type_
        //         ));
        //     }
        //     let value =
        //         Self::allocate_input(&mut cs, &ir_input.type_, &*ir_input.name, value.clone())?;
        //     self.variables.insert(ir_input.variable, value);
        // }
        // Ok(())
        todo!()
    }

    pub fn handle_const_input_block<CS: ConstraintSystem<F>>(
        &mut self,
        input_header: &[InputType],
        input_values: &[InputValue],
        cs: &mut CS,
    ) -> Result<()> {
        // for ir_input in input_header {
        //     let value = input_values
        //         .get(&ir_input.name)
        //         .ok_or_else(|| anyhow!("missing input value for '{}'", ir_input.name))?;
        //     if !value.matches_input_type(&ir_input.type_) {
        //         return Err(anyhow!(
        //             "type mismatch for input '{}', expected {}",
        //             ir_input.name,
        //             ir_input.type_
        //         ));
        //     }
        //     let value = self.resolve(value, cs)?.into_owned();
        //     self.variables.insert(ir_input.variable, value);
        // }
        // Ok(())
        todo!()
    }

    /// loads the arguments for a function into the states variable list
    pub fn setup_evaluate_function(
        &mut self,
        index: u32,
        arguments: &[ConstrainedValue<F, G>],
    ) -> Result<&'a Function> {
        let function = self
            .program
            .functions
            .get(index as usize)
            .expect("missing function");

        let mut arg_register = function.argument_start_variable;
        for argument in arguments {
            self.variables.insert(arg_register, argument.clone());
            arg_register += 1;
        }

        self.function_index = index;
        self.instruction_index = 0;
        self.call_depth += 1;

        Ok(&function)
    }
}
