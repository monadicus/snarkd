use super::*;

pub(super) struct FunctionEvaluator<'a, F: Field, G: Parameters> {
    call_stack: Vec<StateData<'a, F, G>>,
    state_data: StateData<'a, F, G>,
    namespace_id_counter: usize,
}

impl<'a, F: Field, G: Parameters> FunctionEvaluator<'a, F, G> {
    fn nest(&mut self, state: StateData<'a, F, G>) {
        self.call_stack
            .push(mem::replace(&mut self.state_data, state));
    }

    fn unnest(&mut self) -> StateData<'a, F, G> {
        let last_state = self.call_stack.pop().expect("no state to return to");
        mem::replace(&mut self.state_data, last_state)
    }

    // /// if run condition `false`, returns error; else returns to last state with `true` run condition
    // fn unwind(&mut self, e: Error) -> Result<()> {
    //     if self.state_data.condition {
    //         return Err(anyhow!(
    //             "f#{} i#{}: {:?}",
    //             self.state_data.function_index,
    //             self.state_data.state.instruction_index,
    //             e
    //         ));
    //     } else {
    //         self.call_stack = self.call_stack.drain(..).rev().skip_while(|s| !s.condition).collect();
    //         self.call_stack.reverse();
    //         self.state_data = self.call_stack.pop().expect("no state to return to");
    //         Ok(())
    //     }
    // }

    /// setup the state and call stack to start evaluating the target call instruction
    fn setup_call<CS: ConstraintSystem<F>>(&mut self, data: &'a Todo, cs: &mut CS) -> Result<()> {
        // let arguments = data
        //     .arguments
        //     .iter()
        //     .map(|x| self.state_data.state.resolve(x, cs).map(|x| x.into_owned()))
        //     .collect::<Result<Vec<_>, _>>()?;

        // self.namespace_id_counter += 1;
        // let mut parent_variables = self.state_data.state.parent_variables.clone();
        // parent_variables.extend(self.state_data.state.variables.clone());
        // let mut state = EvaluatorState {
        //     program: self.state_data.state.program,
        //     variables: IndexMap::new(),
        //     call_depth: self.state_data.state.call_depth,
        //     parent_variables, // todo: eval perf of IndexMap here
        //     function_index: self.state_data.state.function_index,
        //     instruction_index: 0,
        //     namespace_id: self.namespace_id_counter,
        // };
        // state.cs_meta("function call", cs);

        // let function = state.setup_evaluate_function(data.index, &arguments)?;
        // let state_data = StateData {
        //     arguments: Rc::new(arguments),
        //     function: &function,
        //     function_index: data.index,
        //     block_start: 0,
        //     block_instruction_count: function.instructions.len() as u32,
        //     state,
        //     parent_instruction: ParentInstruction::Call(data),
        //     result: None,
        //     condition: self.state_data.condition,
        // };

        // self.state_data.state.instruction_index += 1;
        // self.nest(state_data);
        // if self.state_data.state.call_depth > self.state_data.state.program.header.inline_limit {
        //     Err(anyhow!("max inline limit hit"))
        // } else if self.call_stack.iter().any(|old_state| {
        //     old_state.arguments == self.state_data.arguments
        //         && old_state.function_index == self.state_data.function_index
        // }) {
        //     Err(anyhow!("infinite recursion detected"))
        // } else {
        //     Ok(())
        // }
        todo!()
    }

    /// returns to the previous state in the call stack and stores the result of the function call's evaluation
    fn finish_call(&mut self, data: &Todo) -> Result<()> {
        // let res = self
        //     .unnest()
        //     .result
        //     .unwrap_or_else(|| ConstrainedValue::Tuple(vec![]));
        // self.state_data.state.store(data.destination, res);
        // Ok(())
        todo!()
    }

    /// setup the state and call stack to start evaluating the target mask instruction
    fn setup_mask<CS: ConstraintSystem<F>>(&mut self, data: &'a Todo, cs: &mut CS) -> Result<()> {
        // if data.instruction_count + self.state_data.state.instruction_index
        //     >= self.state_data.function.instructions.len() as u32
        // {
        //     return Err(anyhow!("illegal mask block length"));
        // }

        // let condition = self
        //     .state_data
        //     .state
        //     .resolve(&data.condition, cs)?
        //     .into_owned()
        //     .extract_bool()
        //     .map_err(|value| anyhow!("illegal condition type for conditional block: {}", value))?
        //     .clone();

        // self.state_data.state.instruction_index += 1;

        // let condition_const_value = todo!(;)

        // // let condition_const_value = match &condition {
        // //     ConstrainedBool::Constant(c) => *c,
        // //     _ => true,
        // // };

        // if condition_const_value {
        //     self.namespace_id_counter += 1;

        //     let mut parent_variables = self.state_data.state.parent_variables.clone();
        //     parent_variables.extend(self.state_data.state.variables.clone());
        //     let state = EvaluatorState {
        //         program: self.state_data.state.program,
        //         variables: IndexMap::new(),
        //         call_depth: self.state_data.state.call_depth,
        //         parent_variables, // todo: eval perf of IndexMap here
        //         function_index: self.state_data.state.function_index,
        //         instruction_index: self.state_data.state.instruction_index,
        //         namespace_id: self.namespace_id_counter,
        //     };
        //     let state_data = StateData {
        //         arguments: self.state_data.arguments.clone(),
        //         function: self.state_data.function,
        //         function_index: self.state_data.function_index,
        //         block_start: self.state_data.state.instruction_index,
        //         block_instruction_count: data.instruction_count,
        //         state,
        //         parent_instruction: ParentInstruction::Mask(condition),
        //         result: self.state_data.result.clone(),
        //         condition: self.state_data.condition && condition.get_value().unwrap_or(true),
        //     };

        //     self.state_data.state.instruction_index += data.instruction_count;
        //     self.nest(state_data);
        // } else {
        //     self.state_data.state.instruction_index += data.instruction_count;
        // }
        // Ok(())
        todo!()
    }

    /// returns to the previous state in the call stack and updates variables from the mask instructions evaluation
    fn finish_mask<CS: ConstraintSystem<F>>(
        &mut self,
        condition: ConstrainedBool,
        cs: &mut CS,
    ) -> Result<()> {
        // let inner_state = self.unnest();
        // let assignments = inner_state.state.variables;
        // let target_index = inner_state.block_start + inner_state.block_instruction_count;
        // let result = inner_state.result.clone();
        // for (variable, value) in assignments {
        //     if let Some(prior) = self.state_data.state.variables.get(&variable).or(self
        //         .state_data
        //         .state
        //         .parent_variables
        //         .get(&variable))
        //     {
        //         let prior = prior.clone(); //todo: optimize away clone
        //         let value = ConstrainedValue::conditionally_select(
        //             &mut self
        //                 .state_data
        //                 .state
        //                 .cs_meta(&*format!("selection {}", variable), cs),
        //             &condition,
        //             &value,
        //             &prior,
        //         )?;
        //         self.state_data.state.store(variable, value);
        //     }
        // }
        // assert_eq!(self.state_data.state.instruction_index, target_index);
        // match (self.state_data.result.clone(), result.clone()) {
        //     (Some(prior), Some(interior)) => {
        //         self.state_data.result = Some(ConstrainedValue::conditionally_select(
        //             &mut self
        //                 .state_data
        //                 .state
        //                 .cs_meta(&*format!("selection result"), cs),
        //             &condition,
        //             &interior,
        //             &prior,
        //         )?);
        //     }
        //     (None, Some(interior)) => {
        //         // will produce garbage if invalid IR (incomplete return path)
        //         self.state_data.result = Some(interior);
        //     }
        //     (_, None) => (),
        // }
        // Ok(())
        todo!()
    }

    /// setup the state and call stack to start evaluating the target repeat instruction.
    /// creates a state for every iteration and adds them all to the call stack
    fn setup_repeat<CS: ConstraintSystem<F>>(&mut self, data: &'a Todo, cs: &mut CS) -> Result<()> {
        // if data.instruction_count + self.state_data.state.instruction_index
        //     >= self.state_data.function.instructions.len() as u32
        // {
        //     return Err(anyhow!("illegal repeat block length"));
        // }

        // let from = self.state_data.state.resolve(&data.from, cs)?.into_owned();
        // let from_int = from
        //     .extract_integer()
        //     .map_err(|value| anyhow!("illegal type for loop init: {}", value))?
        //     .to_owned();
        // let from = from_int
        //     .to_usize()
        //     .ok_or_else(|| anyhow!("illegal input-derived loop index"))?;

        // let to = self.state_data.state.resolve(&data.to, cs)?.into_owned();
        // let to = to
        //     .extract_integer()
        //     .map_err(|value| anyhow!("illegal type for loop terminator: {}", value))?
        //     .to_usize()
        //     .ok_or_else(|| anyhow!("illegal input-derived loop terminator"))?;

        // let iter: Box<dyn Iterator<Item = usize>> = match (from < to, data.inclusive) {
        //     (true, true) => Box::new(from..=to),
        //     (true, false) => Box::new(from..to),
        //     (false, true) => Box::new((to..=from).rev()),
        //     // add the range to the values to get correct bound
        //     (false, false) => Box::new(((to + 1)..(from + 1)).rev()),
        // };

        // self.state_data.state.instruction_index += 1;
        // let mut iter_state_data = Vec::new();
        // //todo: max loop count (DOS vector)
        // for i in iter {
        //     self.namespace_id_counter += 1;
        //     let mut parent_variables = self.state_data.state.parent_variables.clone();
        //     parent_variables.extend(self.state_data.state.variables.clone());
        //     let mut state = EvaluatorState {
        //         program: self.state_data.state.program,
        //         variables: IndexMap::new(),
        //         call_depth: self.state_data.state.call_depth,
        //         parent_variables, // todo: eval perf of IndexMap here
        //         function_index: self.state_data.state.function_index,
        //         instruction_index: self.state_data.state.instruction_index,
        //         namespace_id: self.namespace_id_counter,
        //     };
        //     state.variables.insert(
        //         data.iter_variable,
        //         ConstrainedValue::Integer(match from_int.get_type() {
        //             IntegerType::U8 => Integer::U8(UInt8::constant(
        //                 i.try_into()
        //                     .map_err(|_| anyhow!("loop index out of range for u8"))?,
        //             )),
        //             IntegerType::U16 => Integer::U16(UInt16::constant(
        //                 i.try_into()
        //                     .map_err(|_| anyhow!("loop index out of range for u16"))?,
        //             )),
        //             IntegerType::U32 => Integer::U32(UInt32::constant(
        //                 i.try_into()
        //                     .map_err(|_| anyhow!("loop index out of range for u32"))?,
        //             )),
        //             _ => return Err(anyhow!("illegal type for loop index")),
        //         }),
        //     );
        //     let new_state_data = StateData {
        //         arguments: self.state_data.arguments.clone(),
        //         function: self.state_data.function,
        //         function_index: self.state_data.function_index,
        //         block_start: self.state_data.state.instruction_index,
        //         block_instruction_count: data.instruction_count,
        //         state,
        //         parent_instruction: ParentInstruction::Repeat(data.iter_variable),
        //         result: self.state_data.result.clone(),
        //         condition: self.state_data.condition,
        //     };
        //     iter_state_data.push(new_state_data);
        // }
        // iter_state_data.reverse();
        // self.state_data.state.instruction_index += data.instruction_count;
        // if !iter_state_data.is_empty() {
        //     let new_state = iter_state_data.pop().unwrap();
        //     self.nest(new_state);
        //     self.call_stack.extend(iter_state_data.into_iter());
        // }
        // Ok(())
        todo!()
    }

    /// returns to the previous state in the call stack and updates variables from the repeat instructions evaluation
    fn finish_repeat(&mut self, iter_variable: u32) -> Result<()> {
        let inner_state = self.unnest();
        for (variable, value) in inner_state.state.variables {
            if self
                .state_data
                .state
                .variables
                .get(&variable)
                .or_else(|| self.state_data.state.parent_variables.get(&variable))
                .is_some()
                && variable != iter_variable
            {
                self.state_data.state.store(variable, value);
            }
        }
        Ok(())
    }

    /// returns the output of the initial function call's evaluation. panics if the call stack isn't empty
    fn finish_evaluation(self) -> ConstrainedValue<F, G> {
        // if !self.call_stack.is_empty() {
        //     panic!(
        //         "cant finish evaluation if call stack's not_empty. {:?} element(s) remain",
        //         self.call_stack.len()
        //     )
        // }
        // let res = self
        //     .state_data
        //     .result
        //     .unwrap_or_else(|| ConstrainedValue::Tuple(vec![]));
        // res
        todo!()
    }

    /// iterates over every instruction in the function's code block and evaluates it.
    /// if a new code block is hit (via mask, repeat, or call instructions) then the current blocks state is stored on a call stack while the new code block is evaluated
    pub fn evaluate_function<CS: ConstraintSystem<F>>(
        function: &'a Function,
        state: EvaluatorState<'a, F, G>,
        index: u32,
        cs: &mut CS,
    ) -> Result<ConstrainedValue<F, G>> {
        let mut evaluator = Self {
            call_stack: Vec::new(),
            namespace_id_counter: state.namespace_id,
            state_data: StateData::create_initial_state_data(
                state,
                function,
                Rc::new(Vec::new()),
                index,
            )?,
        };
        loop {
            match evaluator.state_data.evaluate_block(cs) {
                // TODO
                // Ok(Some(Instruction::Call(data))) => {
                //     evaluator.setup_call(data, cs)?;
                // }
                // Ok(Some(Instruction::Mask(data))) => {
                //     evaluator.setup_mask(data, cs)?;
                // }
                // Ok(Some(Instruction::Repeat(data))) => {
                //     evaluator.setup_repeat(data, cs)?;
                // }
                Ok(Some(e)) => panic!("invalid control instruction: {:?}", e),
                Ok(None) => match evaluator.state_data.parent_instruction {
                    ParentInstruction::Call(data) => {
                        evaluator.finish_call(data)?;
                    }
                    ParentInstruction::Mask(condition) => {
                        evaluator.finish_mask(condition, cs)?;
                    }
                    ParentInstruction::Repeat(iter_variable) => {
                        evaluator.finish_repeat(iter_variable)?;
                    }
                    ParentInstruction::None => {
                        return Ok(evaluator.finish_evaluation());
                    }
                },
                Err(e) => {
                    return Err(anyhow!(
                        "f#{} i#{}: {:?}",
                        evaluator.state_data.function_index,
                        evaluator.state_data.state.instruction_index,
                        e
                    ));
                }
            }
        }
    }
}
