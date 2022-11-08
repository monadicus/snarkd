use super::*;

/// stores data about the state for scope recursion
pub struct StateData<'a, F: Field, G: Parameters> {
    /// the arguments passed to the function
    pub arguments: Rc<Vec<ConstrainedValue<F, G>>>,
    /// the function currently being evaluated
    pub function: &'a Function,
    pub function_index: u32,
    /// the instruction that created this scope
    pub parent_instruction: ParentInstruction<'a>,
    /// the starting instruction index of the scope
    pub block_start: u32,
    /// the length of the scope
    pub block_instruction_count: u32,
    /// the state of the scope
    pub state: EvaluatorState<'a, F, G>,
    /// the value returned by the scope, if anything has been returned yet
    pub result: Option<ConstrainedValue<F, G>>,
    /// if the code doesnt always execute
    pub condition: bool,
}

/// implemented this so i could easily debug execution flow
impl<'a, F: Field, G: Parameters> fmt::Debug for StateData<'a, F, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StateData {{
          ...
          function: {{
              ...
              instructions: {:?},
              ...
          }},
          function_index: {},
          instruction: {:?},
          block_start: {},
          block_instruction_count: {},
          state: {{
              ...
              instruction_index: {},
              call_depth: {},
              ...
          }}
          result: {:?},
          condition: {},
      }}",
            self.function.instructions,
            self.function_index,
            self.parent_instruction,
            self.block_start,
            self.block_instruction_count,
            self.state.instruction_index,
            self.state.call_depth,
            self.result,
            self.condition,
        )
    }
}

impl<'a, F: Field, G: Parameters> StateData<'a, F, G> {
    /// creates the initial state at the start of a programs function evaluation step.
    /// the function passed should be main and the index should point to main
    pub fn create_initial_state_data(
        state: EvaluatorState<'a, F, G>,
        function: &'a Function,
        arguments: Rc<Vec<ConstrainedValue<F, G>>>,
        index: u32,
    ) -> Result<Self> {
        Ok(Self {
            arguments,
            function,
            function_index: index,
            block_start: 0,
            block_instruction_count: function.instructions.len() as u32,
            state,
            parent_instruction: ParentInstruction::None,
            result: None,
            condition: true,
        })
    }

    /// evaluates each instruction in a block.
    /// if a control instruction was hit (mask, repeat, call) then it halts evaluation and returns the instruction
    pub fn evaluate_block<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
    ) -> Result<Option<&'a Instruction>> {
        while self.state.instruction_index < self.block_start + self.block_instruction_count {
            let instruction = &self.function.instructions[self.state.instruction_index as usize];
            match self.evaluate_instruction(instruction, cs)? {
                ControlFlow::Recurse(ins) => return Ok(Some(ins)),
                ControlFlow::Return => {
                    return Ok(None);
                }
                ControlFlow::Continue => continue,
            }
        }
        Ok(None)
    }

    /// handles the control flow of instruction evaluation.
    /// returns the instruction if mask, call, or repeat was hit; else passes it to state for evaluation
    pub fn evaluate_instruction<CS: ConstraintSystem<F>>(
        &mut self,
        instruction: &'a Instruction,
        cs: &mut CS,
    ) -> Result<ControlFlow<'a>> {
        match instruction {
            // TODO
            // Instruction::Call(_) | Instruction::Mask(_) | Instruction::Repeat(_) => {
            //     Ok(ControlFlow::Recurse(instruction))
            // }
            _ => match self
                .state
                .evaluate_instruction(instruction, self.condition, cs)
            {
                Ok(Some(returned)) => {
                    self.result = Some(returned);
                    Ok(ControlFlow::Return)
                }
                Ok(None) => {
                    self.state.instruction_index += 1;
                    Ok(ControlFlow::Continue)
                }
                Err(e) => Err(e),
            },
        }
    }
}
