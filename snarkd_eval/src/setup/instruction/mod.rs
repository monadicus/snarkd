// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkd_crypto::*;
use snarkd_ir::Instruction;

use super::*;

impl<'a, F: Field, G: Group> EvaluatorState<'a, F, G> {
    /// Evaluates a single instruction in the local [`EvaluatorState`] context. Panics if `instruction` is a control instruction.
    pub(super) fn evaluate_instruction<'b, CS: ConstraintSystem<F>>(
        &mut self,
        instruction: &'b Instruction,
        branch_condition: bool,
        cs: &mut CS,
    ) -> Result<Option<ConstrainedValue<F, G>>> {
        match instruction {
            Instruction::Abs(data) => todo!(),
            Instruction::AbsWrapped(data) => todo!(),
            Instruction::Add(data) => todo!(),
            Instruction::AddWrapped(data) => todo!(),
            Instruction::And(data) => todo!(),
            Instruction::AssertEq(data) => todo!(),
            Instruction::AssertNeq(data) => todo!(),
            Instruction::CommitBhp256(data) => todo!(),
            Instruction::CommitBhp512(data) => todo!(),
            Instruction::CommitBhp768(data) => todo!(),
            Instruction::CommitBhp1024(data) => todo!(),
            Instruction::CommitPed64(data) => todo!(),
            Instruction::CommitPed128(data) => todo!(),
            Instruction::Div(data) => todo!(),
            Instruction::DivWrapped(data) => todo!(),
            Instruction::Double(data) => todo!(),
            Instruction::Gt(data) => todo!(),
            Instruction::Gte(data) => todo!(),
            Instruction::HashBhp256(data) => todo!(),
            Instruction::HashBhp512(data) => todo!(),
            Instruction::HashBhp768(data) => todo!(),
            Instruction::HashBhp1024(data) => todo!(),
            Instruction::HashPed64(data) => todo!(),
            Instruction::HashPed128(data) => todo!(),
            Instruction::HashPsd2(data) => todo!(),
            Instruction::HashPsd4(data) => todo!(),
            Instruction::HashPsd8(data) => todo!(),
            Instruction::Inv(data) => todo!(),
            Instruction::IsEq(data) => todo!(),
            Instruction::IsNeq(data) => todo!(),
            Instruction::Lt(data) => todo!(),
            Instruction::Lte(data) => todo!(),
            Instruction::Mod(data) => todo!(),
            Instruction::Mul(data) => todo!(),
            Instruction::MulWrapped(data) => todo!(),
            Instruction::Nand(data) => todo!(),
            Instruction::Neg(data) => todo!(),
            Instruction::Nor(data) => todo!(),
            Instruction::Not(data) => todo!(),
            Instruction::Or(data) => todo!(),
            Instruction::Pow(data) => todo!(),
            Instruction::PowWrapped(data) => todo!(),
            Instruction::Rem(data) => todo!(),
            Instruction::RemWrapped(data) => todo!(),
            Instruction::Shl(data) => todo!(),
            Instruction::ShlWrapped(data) => todo!(),
            Instruction::Shr(data) => todo!(),
            Instruction::ShrWrapped(data) => todo!(),
            Instruction::Sqrt(data) => todo!(),
            Instruction::Square(data) => todo!(),
            Instruction::Sub(data) => todo!(),
            Instruction::SubWrapped(data) => todo!(),
            Instruction::Ternary(data) => todo!(),
            Instruction::Xor(data) => todo!(),
        }
        Ok(None)
    }
}
