use snarkvm_fields::PrimeField;

use crate::snark::marlin::{witness_label, MarlinMode};

/// First message of the verifier.
#[derive(Clone, Debug)]
pub struct FirstMessage<F> {
    /// Query for the random polynomial.
    pub alpha: F,
    /// Randomizer for the lincheck for `B`.
    pub eta_b: F,
    /// Randomizer for the lincheck for `C`.
    pub eta_c: F,
    /// Randomizers for combining vectors from the batch.
    pub batch_combiners: Vec<F>,
}

/// Second verifier message.
#[derive(Copy, Clone, Debug)]
pub struct SecondMessage<F> {
    /// Query for the second round of polynomials.
    pub beta: F,
}

/// Third message of the verifier.
#[derive(Copy, Clone, Debug)]
pub struct ThirdMessage<F> {
    /// Randomizer for the h-polynomial for `B`.
    pub r_b: F,
    /// Randomizer for the h-polynomial for `C`.
    pub r_c: F,
}

/// Query set of the verifier.
#[derive(Clone, Debug)]
pub struct QuerySet<F> {
    pub batch_size: usize,
    pub g_1_query: (String, F),
    pub z_b_query: (String, F),
    pub lincheck_sumcheck_query: (String, F),

    pub g_a_query: (String, F),
    pub g_b_query: (String, F),
    pub g_c_query: (String, F),
    pub matrix_sumcheck_query: (String, F),
}

impl<F: PrimeField> QuerySet<F> {
    pub fn new<MM: MarlinMode>(state: &super::State<F, MM>) -> Self {
        let beta = state.second_round_message.unwrap().beta;
        let gamma = state.gamma.unwrap();
        // For the first linear combination
        // Lincheck sumcheck test:
        //   s(beta) + r(alpha, beta) * (sum_M eta_M z_M(beta)) - t(beta) * z(beta)
        // = h_1(beta) * v_H(beta) + beta * g_1(beta)
        //
        // Note that z is the interpolation of x || w, so it equals x + v_X * w
        // We also use an optimization: instead of explicitly calculating z_c, we
        // use the "virtual oracle" z_a * z_b
        Self {
            batch_size: state.batch_size,
            g_1_query: ("beta".into(), beta),
            z_b_query: ("beta".into(), beta),
            lincheck_sumcheck_query: ("beta".into(), beta),

            g_a_query: ("gamma".into(), gamma),
            g_b_query: ("gamma".into(), gamma),
            g_c_query: ("gamma".into(), gamma),
            matrix_sumcheck_query: ("gamma".into(), gamma),
        }
    }

    /// Returns a `BTreeSet` containing elements of the form
    /// `(polynomial_label, (query_label, query))`.
    pub fn to_set(&self) -> crate::polycommit::sonic_pc::QuerySet<'_, F> {
        let mut query_set = crate::polycommit::sonic_pc::QuerySet::new();
        for i in 0..self.batch_size {
            query_set.insert((witness_label("z_b", i), self.z_b_query.clone()));
        }
        query_set.insert(("g_1".into(), self.g_1_query.clone()));
        query_set.insert((
            "lincheck_sumcheck".into(),
            self.lincheck_sumcheck_query.clone(),
        ));

        query_set.insert(("g_a".into(), self.g_a_query.clone()));
        query_set.insert(("g_b".into(), self.g_b_query.clone()));
        query_set.insert(("g_c".into(), self.g_c_query.clone()));
        query_set.insert(("matrix_sumcheck".into(), self.matrix_sumcheck_query.clone()));
        query_set
    }
}
