use crate::{
    fft::{
        domain::{FFTPrecomputation, IFFTPrecomputation},
        EvaluationDomain,
    },
    marlin::{ahp::matrices::MatrixArithmetization, AHPForR1CS, CircuitInfo, MarlinMode, Matrix},
    polycommit::sonic_pc::LabeledPolynomial,
};

#[derive(Clone, Debug)]
/// The indexed version of the constraint system.
/// This struct contains three kinds of objects:
/// 1) `index_info` is information about the index, such as the size of the
///     public input
/// 2) `{a,b,c}` are the matrices defining the R1CS instance
/// 3) `{a,b,c}_star_arith` are structs containing information about A^*, B^*, and C^*,
/// which are matrices defined as `M^*(i, j) = M(j, i) * u_H(j, j)`.
pub struct Circuit {
    /// Information about the indexed circuit.
    pub index_info: CircuitInfo,

    /// The A matrix for the R1CS instance
    pub a: Matrix,
    /// The B matrix for the R1CS instance
    pub b: Matrix,
    /// The C matrix for the R1CS instance
    pub c: Matrix,

    /// Joint arithmetization of the A*, B*, and C* matrices.
    pub a_arith: MatrixArithmetization,
    pub b_arith: MatrixArithmetization,
    pub c_arith: MatrixArithmetization,

    pub fft_precomputation: FFTPrecomputation,
    pub ifft_precomputation: IFFTPrecomputation,

    pub(crate) zk: bool,
}

impl Circuit {
    /// The maximum degree required to represent polynomials of this index.
    pub fn max_degree(&self) -> usize {
        self.index_info.max_degree(self.zk)
    }

    /// The number of constraints in this R1CS instance.
    pub fn constraint_domain_size(&self) -> usize {
        crate::fft::EvaluationDomain::new(self.index_info.num_constraints)
            .unwrap()
            .size()
    }

    /// Iterate over the indexed polynomials.
    pub fn iter(&self) -> impl Iterator<Item = &LabeledPolynomial> {
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
