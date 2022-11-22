use crate::{bls12_377::Scalar, marlin::ahp, polycommit::sonic_pc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commitments {
    pub witness_commitments: Vec<WitnessCommitments>,
    /// Commitment to the masking polynomial.
    pub mask_poly: Option<sonic_pc::Commitment>,
    /// Commitment to the `g_1` polynomial.
    pub g_1: sonic_pc::Commitment,
    /// Commitment to the `h_1` polynomial.
    pub h_1: sonic_pc::Commitment,
    /// Commitment to the `g_a` polynomial.
    pub g_a: sonic_pc::Commitment,
    /// Commitment to the `g_b` polynomial.
    pub g_b: sonic_pc::Commitment,
    /// Commitment to the `g_c` polynomial.
    pub g_c: sonic_pc::Commitment,
    /// Commitment to the `h_2` polynomial.
    pub h_2: sonic_pc::Commitment,
}

/// Commitments to the `w`, `z_a`, and `z_b` polynomials.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WitnessCommitments {
    /// Commitment to the `w` polynomial.
    pub w: sonic_pc::Commitment,
    /// Commitment to the `z_a` polynomial.
    pub z_a: sonic_pc::Commitment,
    /// Commitment to the `z_b` polynomial.
    pub z_b: sonic_pc::Commitment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluations {
    /// Evaluation of `z_b_i`'s at `beta`.
    pub z_b_evals: Vec<Scalar>,
    /// Evaluation of `g_1` at `beta`.
    pub g_1_eval: Scalar,
    /// Evaluation of `g_a` at `beta`.
    pub g_a_eval: Scalar,
    /// Evaluation of `g_b` at `gamma`.
    pub g_b_eval: Scalar,
    /// Evaluation of `g_c` at `gamma`.
    pub g_c_eval: Scalar,
}

impl Evaluations {
    pub(crate) fn from_map(
        map: &std::collections::BTreeMap<String, Scalar>,
        batch_size: usize,
    ) -> Self {
        let z_b_evals = map
            .iter()
            .filter_map(|(k, v)| k.starts_with("z_b_").then_some(*v))
            .collect::<Vec<_>>();
        assert_eq!(z_b_evals.len(), batch_size);
        Self {
            z_b_evals,
            g_1_eval: map["g_1"],
            g_a_eval: map["g_a"],
            g_b_eval: map["g_b"],
            g_c_eval: map["g_c"],
        }
    }

    pub(crate) fn get(&self, label: &str) -> Option<Scalar> {
        if let Some(index) = label.strip_prefix("z_b_") {
            self.z_b_evals.get(index.parse::<usize>().unwrap()).copied()
        } else {
            match label {
                "g_1" => Some(self.g_1_eval),
                "g_a" => Some(self.g_a_eval),
                "g_b" => Some(self.g_b_eval),
                "g_c" => Some(self.g_c_eval),
                _ => None,
            }
        }
    }
}

impl Evaluations {
    pub fn to_field_elements(&self) -> Vec<Scalar> {
        let mut result = self.z_b_evals.clone();
        result.extend([self.g_1_eval, self.g_a_eval, self.g_b_eval, self.g_c_eval]);
        result
    }
}

/// A zkSNARK proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof {
    /// The number of instances being proven in this proof.
    batch_size: usize,

    /// Commitments to prover polynomials.
    pub commitments: Commitments,

    /// Evaluations of some of the committed polynomials.
    pub evaluations: Evaluations,

    /// Prover message: sum_a, sum_b, sum_c
    pub msg: ahp::prover::ThirdMessage,

    /// An evaluation proof from the polynomial commitment.
    pub pc_proof: sonic_pc::BatchLCProof,
}

impl Proof {
    /// Construct a new proof.
    pub fn new(
        batch_size: usize,
        commitments: Commitments,
        evaluations: Evaluations,
        msg: ahp::prover::ThirdMessage,
        pc_proof: sonic_pc::BatchLCProof,
    ) -> Self {
        Self {
            batch_size,
            commitments,
            evaluations,
            msg,
            pc_proof,
        }
    }
}
