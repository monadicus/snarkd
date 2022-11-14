// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use core::marker::PhantomData;

use crate::{
    fft::{
        domain::{FFTPrecomputation, IFFTPrecomputation},
        EvaluationDomain,
    },
    polycommit::sonic_pc::LabeledPolynomial,
    snark::marlin::{ahp::matrices::MatrixArithmetization, AHPForR1CS, CircuitInfo, MarlinMode, Matrix},
};
use snarkvm_fields::PrimeField;
use snarkvm_utilities::{serialize::*, SerializationError};

#[derive(Clone, Debug)]
/// The indexed version of the constraint system.
/// This struct contains three kinds of objects:
/// 1) `index_info` is information about the index, such as the size of the
///     public input
/// 2) `{a,b,c}` are the matrices defining the R1CS instance
/// 3) `{a,b,c}_star_arith` are structs containing information about A^*, B^*, and C^*,
/// which are matrices defined as `M^*(i, j) = M(j, i) * u_H(j, j)`.
pub struct Circuit<F: PrimeField, MM: MarlinMode> {
    /// Information about the indexed circuit.
    pub index_info: CircuitInfo<F>,

    /// The A matrix for the R1CS instance
    pub a: Matrix<F>,
    /// The B matrix for the R1CS instance
    pub b: Matrix<F>,
    /// The C matrix for the R1CS instance
    pub c: Matrix<F>,

    /// Joint arithmetization of the A*, B*, and C* matrices.
    pub a_arith: MatrixArithmetization<F>,
    pub b_arith: MatrixArithmetization<F>,
    pub c_arith: MatrixArithmetization<F>,

    pub fft_precomputation: FFTPrecomputation<F>,
    pub ifft_precomputation: IFFTPrecomputation<F>,

    pub(crate) mode: PhantomData<MM>,
}

impl<F: PrimeField, MM: MarlinMode> Circuit<F, MM> {
    /// The maximum degree required to represent polynomials of this index.
    pub fn max_degree(&self) -> usize {
        self.index_info.max_degree::<MM>()
    }

    /// The number of constraints in this R1CS instance.
    pub fn constraint_domain_size(&self) -> usize {
        crate::fft::EvaluationDomain::<F>::new(self.index_info.num_constraints).unwrap().size()
    }

    /// Iterate over the indexed polynomials.
    pub fn iter(&self) -> impl Iterator<Item = &LabeledPolynomial<F>> {
        // Alphabetical order
        [
            &self.a_arith.col,
            &self.b_arith.col,
            &self.c_arith.col,
            &self.a_arith.row,
            &self.b_arith.row,
            &self.c_arith.row,
            &self.a_arith.row_col,
            &self.b_arith.row_col,
            &self.c_arith.row_col,
            &self.a_arith.val,
            &self.b_arith.val,
            &self.c_arith.val,
        ]
        .into_iter()
    }
}

impl<F: PrimeField, MM: MarlinMode> CanonicalSerialize for Circuit<F, MM> {
    #[allow(unused_mut, unused_variables)]
    fn serialize_with_mode<W: Write>(&self, mut writer: W, compress: Compress) -> Result<(), SerializationError> {
        self.index_info.serialize_with_mode(&mut writer, compress)?;
        self.a.serialize_with_mode(&mut writer, compress)?;
        self.b.serialize_with_mode(&mut writer, compress)?;
        self.c.serialize_with_mode(&mut writer, compress)?;
        self.a_arith.serialize_with_mode(&mut writer, compress)?;
        self.b_arith.serialize_with_mode(&mut writer, compress)?;
        self.c_arith.serialize_with_mode(&mut writer, compress)?;
        self.mode.serialize_with_mode(&mut writer, compress)?;
        Ok(())
    }

    #[allow(unused_mut, unused_variables)]
    fn serialized_size(&self, mode: Compress) -> usize {
        let mut size = 0;
        size += self.index_info.serialized_size(mode);
        size += self.a.serialized_size(mode);
        size += self.b.serialized_size(mode);
        size += self.c.serialized_size(mode);
        size += self.a_arith.serialized_size(mode);
        size += self.b_arith.serialized_size(mode);
        size += self.c_arith.serialized_size(mode);
        size += self.mode.serialized_size(mode);
        size
    }
}
impl<F: PrimeField, MM: MarlinMode> snarkvm_utilities::Valid for Circuit<F, MM> {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }

    fn batch_check<'a>(_batch: impl Iterator<Item = &'a Self> + Send) -> Result<(), SerializationError>
    where
        Self: 'a,
    {
        Ok(())
    }
}

impl<F: PrimeField, MM: MarlinMode> CanonicalDeserialize for Circuit<F, MM> {
    #[allow(unused_mut, unused_variables)]
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let index_info: CircuitInfo<F> = CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?;
        let constraint_domain_size = EvaluationDomain::<F>::compute_size_of_domain(index_info.num_constraints)
            .ok_or(SerializationError::InvalidData)?;
        let non_zero_a_domain_size = EvaluationDomain::<F>::compute_size_of_domain(index_info.num_non_zero_a)
            .ok_or(SerializationError::InvalidData)?;
        let non_zero_b_domain_size = EvaluationDomain::<F>::compute_size_of_domain(index_info.num_non_zero_b)
            .ok_or(SerializationError::InvalidData)?;
        let non_zero_c_domain_size = EvaluationDomain::<F>::compute_size_of_domain(index_info.num_non_zero_c)
            .ok_or(SerializationError::InvalidData)?;

        let (fft_precomputation, ifft_precomputation) = AHPForR1CS::<F, MM>::fft_precomputation(
            constraint_domain_size,
            non_zero_a_domain_size,
            non_zero_b_domain_size,
            non_zero_c_domain_size,
        )
        .ok_or(SerializationError::InvalidData)?;
        Ok(Circuit {
            index_info,
            a: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            b: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            c: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            a_arith: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            b_arith: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            c_arith: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
            fft_precomputation,
            ifft_precomputation,
            mode: CanonicalDeserialize::deserialize_with_mode(&mut reader, compress, validate)?,
        })
    }
}
