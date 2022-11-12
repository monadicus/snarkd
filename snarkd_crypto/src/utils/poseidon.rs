use crate::bls12_377::{fp, Field, Fp};
use smallvec::SmallVec;
use std::{
    ops::{Index, IndexMut},
    sync::Arc,
};

/// The mode structure for duplex sponges
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DuplexSpongeMode {
    /// The sponge is currently absorbing data.
    Absorbing {
        /// next position of the state to be XOR-ed when absorbing.
        next_absorb_index: usize,
    },
    /// The sponge is currently squeezing data out.
    Squeezing {
        /// next position of the state to be outputted when squeezing.
        next_squeeze_index: usize,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct State<const RATE: usize, const CAPACITY: usize> {
    capacity_state: [Fp; CAPACITY],
    rate_state: [Fp; RATE],
}

impl<const RATE: usize, const CAPACITY: usize> Default for State<RATE, CAPACITY> {
    fn default() -> Self {
        Self {
            capacity_state: [Fp::ZERO; CAPACITY],
            rate_state: [Fp::ZERO; RATE],
        }
    }
}

impl<const RATE: usize, const CAPACITY: usize> State<RATE, CAPACITY> {
    /// Returns an immutable iterator over the state.
    pub fn iter(&self) -> impl Iterator<Item = &Fp> + Clone {
        self.capacity_state.iter().chain(self.rate_state.iter())
    }

    /// Returns an mutable iterator over the state.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Fp> {
        self.capacity_state
            .iter_mut()
            .chain(self.rate_state.iter_mut())
    }
}

impl<const RATE: usize, const CAPACITY: usize> Index<usize> for State<RATE, CAPACITY> {
    type Output = Fp;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < RATE + CAPACITY,
            "Index out of bounds: index is {} but length is {}",
            index,
            RATE + CAPACITY
        );
        if index < CAPACITY {
            &self.capacity_state[index]
        } else {
            &self.rate_state[index - CAPACITY]
        }
    }
}

impl<const RATE: usize, const CAPACITY: usize> IndexMut<usize> for State<RATE, CAPACITY> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < RATE + CAPACITY,
            "Index out of bounds: index is {} but length is {}",
            index,
            RATE + CAPACITY
        );
        if index < CAPACITY {
            &mut self.capacity_state[index]
        } else {
            &mut self.rate_state[index - CAPACITY]
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poseidon<const RATE: usize> {
    parameters: Arc<PoseidonParameters<RATE, 1>>,
}

impl<const RATE: usize> Poseidon<RATE> {
    /// Initializes a new instance of the cryptographic hash function.
    pub fn setup() -> Self {
        Self {
            parameters: Arc::new(Fp::default_poseidon_parameters::<RATE>().unwrap()),
        }
    }

    /// Evaluate the cryptographic hash function over a list of field elements as input.
    pub fn evaluate(&self, input: &[Fp]) -> Fp {
        self.evaluate_many(input, 1)[0]
    }

    /// Evaluate the cryptographic hash function over a list of field elements as input,
    /// and returns the specified number of field elements as output.
    pub fn evaluate_many(&self, input: &[Fp], num_outputs: usize) -> Vec<Fp> {
        let mut sponge = PoseidonSponge::<RATE, 1>::new_with_parameters(&self.parameters);
        sponge.absorb_native_field_elements(input);
        sponge.squeeze_native_field_elements(num_outputs).to_vec()
    }

    /// Evaluate the cryptographic hash function over a non-fixed-length vector,
    /// in which the length also needs to be hashed.
    pub fn evaluate_with_len(&self, input: &[Fp]) -> Fp {
        self.evaluate(&[vec![F::from(input.len() as u128)], input.to_vec()].concat())
    }

    pub fn parameters(&self) -> &Arc<PoseidonParameters<RATE, 1>> {
        &self.parameters
    }
}

/// A duplex sponge based using the Poseidon permutation.
///
/// This implementation of Poseidon is entirely from Fractal's implementation in [COS20][cos]
/// with small syntax changes.
///
/// [cos]: https://eprint.iacr.org/2019/1076
#[derive(Clone, Debug)]
pub struct PoseidonSponge<const RATE: usize, const CAPACITY: usize> {
    /// Sponge Parameters
    parameters: Arc<PoseidonParameters<RATE, CAPACITY>>,
    /// Current sponge's state (current elements in the permutation block)
    state: State<RATE, CAPACITY>,
    /// Current mode (whether its absorbing or squeezing)
    pub mode: DuplexSpongeMode,
}

impl<const RATE: usize> PoseidonSponge<RATE, 1> {
    type Parameters = Arc<PoseidonParameters<RATE, 1>>;

    fn sample_parameters() -> Self::Parameters {
        Arc::new(Fp::default_poseidon_parameters::<RATE>().unwrap())
    }

    fn new_with_parameters(parameters: &Self::Parameters) -> Self {
        Self {
            parameters: parameters.clone(),
            state: State::default(),
            mode: DuplexSpongeMode::Absorbing {
                next_absorb_index: 0,
            },
        }
    }

    /// Takes in field elements.
    fn absorb_native_field_elements<T: ToConstraintField<F>>(&mut self, elements: &[T]) {
        let input = elements
            .iter()
            .flat_map(|e| e.to_field_elements().unwrap())
            .collect::<Vec<_>>();
        if !input.is_empty() {
            match self.mode {
                DuplexSpongeMode::Absorbing {
                    mut next_absorb_index,
                } => {
                    if next_absorb_index == RATE {
                        self.permute();
                        next_absorb_index = 0;
                    }
                    self.absorb_internal(next_absorb_index, &input);
                }
                DuplexSpongeMode::Squeezing {
                    next_squeeze_index: _,
                } => {
                    self.permute();
                    self.absorb_internal(0, &input);
                }
            }
        }
    }

    /// Takes in field elements.
    fn absorb_nonnative_field_elements(&mut self, elements: impl IntoIterator<Item = Fp>) {
        Self::push_elements_to_sponge(self, elements, OptimizationType::Weight);
    }

    fn squeeze_nonnative_field_elements(&mut self, num: usize) -> SmallVec<[Fp; 10]> {
        self.get_fe(num, false)
    }

    fn squeeze_native_field_elements(&mut self, num_elements: usize) -> SmallVec<[Fp; 10]> {
        if num_elements == 0 {
            return SmallVec::<[Fp; 10]>::new();
        }
        let mut output = if num_elements <= 10 {
            smallvec::smallvec_inline![Fp::ZERO; 10]
        } else {
            smallvec::smallvec![Fp::ZERO; num_elements]
        };

        match self.mode {
            DuplexSpongeMode::Absorbing {
                next_absorb_index: _,
            } => {
                self.permute();
                self.squeeze_internal(0, &mut output[..num_elements]);
            }
            DuplexSpongeMode::Squeezing {
                mut next_squeeze_index,
            } => {
                if next_squeeze_index == RATE {
                    self.permute();
                    next_squeeze_index = 0;
                }
                self.squeeze_internal(next_squeeze_index, &mut output[..num_elements]);
            }
        }

        output.truncate(num_elements);
        output
    }

    /// Takes out field elements of 168 bits.
    fn squeeze_short_nonnative_field_elements(&mut self, num: usize) -> SmallVec<[Fp; 10]> {
        self.get_fe(num, true)
    }
}

impl<const RATE: usize> PoseidonSponge<RATE, 1> {
    #[inline]
    fn apply_ark(&mut self, round_number: usize) {
        for (state_elem, ark_elem) in self
            .state
            .iter_mut()
            .zip(&self.parameters.ark[round_number])
        {
            *state_elem += ark_elem;
        }
    }

    #[inline]
    fn apply_s_box(&mut self, is_full_round: bool) {
        if is_full_round {
            // Full rounds apply the S Box (x^alpha) to every element of state
            for elem in self.state.iter_mut() {
                *elem = elem.pow([self.parameters.alpha]);
            }
        } else {
            // Partial rounds apply the S Box (x^alpha) to just the first element of state
            self.state[0] = self.state[0].pow([self.parameters.alpha]);
        }
    }

    #[inline]
    fn apply_mds(&mut self) {
        let mut new_state = State::default();
        new_state
            .iter_mut()
            .zip(&self.parameters.mds)
            .for_each(|(new_elem, mds_row)| {
                *new_elem = F::sum_of_products(self.state.iter(), mds_row.iter());
            });
        self.state = new_state;
    }

    #[inline]
    fn permute(&mut self) {
        // Determine the partial rounds range bound.
        let partial_rounds = self.parameters.partial_rounds;
        let full_rounds = self.parameters.full_rounds;
        let full_rounds_over_2 = full_rounds / 2;
        let partial_round_range = full_rounds_over_2..(full_rounds_over_2 + partial_rounds);

        // Iterate through all rounds to permute.
        for i in 0..(partial_rounds + full_rounds) {
            let is_full_round = !partial_round_range.contains(&i);
            self.apply_ark(i);
            self.apply_s_box(is_full_round);
            self.apply_mds();
        }
    }

    /// Absorbs everything in elements, this does not end in an absorption.
    #[inline]
    fn absorb_internal(&mut self, mut rate_start: usize, input: &[Fp]) {
        if !input.is_empty() {
            let first_chunk_size = std::cmp::min(RATE - rate_start, input.len());
            let num_elements_remaining = input.len() - first_chunk_size;
            let (first_chunk, rest_chunk) = input.split_at(first_chunk_size);
            let rest_chunks = rest_chunk.chunks(RATE);
            // The total number of chunks is `elements[num_elements_remaining..].len() / RATE`, plus 1
            // for the remainder.
            let total_num_chunks = 1 + // 1 for the first chunk
                // We add all the chunks that are perfectly divisible by `RATE`
                (num_elements_remaining / RATE) +
                // And also add 1 if the last chunk is non-empty
                // (i.e. if `num_elements_remaining` is not a multiple of `RATE`)
                usize::from((num_elements_remaining % RATE) != 0);

            // Absorb the input elements, `RATE` elements at a time, except for the first chunk, which
            // is of size `RATE - rate_start`.
            for (i, chunk) in std::iter::once(first_chunk).chain(rest_chunks).enumerate() {
                for (element, state_elem) in
                    chunk.iter().zip(&mut self.state.rate_state[rate_start..])
                {
                    *state_elem += element;
                }
                // Are we in the last chunk?
                // If so, let's wrap up.
                if i == total_num_chunks - 1 {
                    self.mode = DuplexSpongeMode::Absorbing {
                        next_absorb_index: rate_start + chunk.len(),
                    };
                    return;
                } else {
                    self.permute();
                }
                rate_start = 0;
            }
        }
    }

    /// Squeeze |output| many elements. This does not end in a squeeze
    #[inline]
    fn squeeze_internal(&mut self, mut rate_start: usize, output: &mut [Fp]) {
        let output_size = output.len();
        if output_size != 0 {
            let first_chunk_size = std::cmp::min(RATE - rate_start, output.len());
            let num_output_remaining = output.len() - first_chunk_size;
            let (first_chunk, rest_chunk) = output.split_at_mut(first_chunk_size);
            assert_eq!(rest_chunk.len(), num_output_remaining);
            let rest_chunks = rest_chunk.chunks_mut(RATE);
            // The total number of chunks is `output[num_output_remaining..].len() / RATE`, plus 1
            // for the remainder.
            let total_num_chunks = 1 + // 1 for the first chunk
                // We add all the chunks that are perfectly divisible by `RATE`
                (num_output_remaining / RATE) +
                // And also add 1 if the last chunk is non-empty
                // (i.e. if `num_output_remaining` is not a multiple of `RATE`)
                usize::from((num_output_remaining % RATE) != 0);

            // Absorb the input output, `RATE` output at a time, except for the first chunk, which
            // is of size `RATE - rate_start`.
            for (i, chunk) in std::iter::once(first_chunk).chain(rest_chunks).enumerate() {
                let range = rate_start..(rate_start + chunk.len());
                debug_assert_eq!(
                    chunk.len(),
                    self.state.rate_state[range.clone()].len(),
                    "failed with squeeze {} at rate {} and rate_start {}",
                    output_size,
                    RATE,
                    rate_start
                );
                chunk.copy_from_slice(&self.state.rate_state[range]);
                // Are we in the last chunk?
                // If so, let's wrap up.
                if i == total_num_chunks - 1 {
                    self.mode = DuplexSpongeMode::Squeezing {
                        next_squeeze_index: (rate_start + chunk.len()),
                    };
                    return;
                } else {
                    self.permute();
                }
                rate_start = 0;
            }
        }
    }

    /// Compress every two elements if possible.
    /// Provides a vector of (limb, num_of_additions), both of which are F.
    pub fn compress_elements(src_limbs: &[(Fp, Fp)], ty: OptimizationType) -> Vec<Fp> {
        let capacity = fp::MODULUS_BITS - 1;
        let mut dest_limbs = Vec::<Fp>::new();

        let params = get_params(TargetField::size_in_bits(), F::size_in_bits(), ty);

        let adjustment_factor_lookup_table = {
            let mut table = Vec::<F>::new();

            let mut cur = F::one();
            for _ in 1..=capacity {
                table.push(cur);
                cur.double_in_place();
            }

            table
        };

        let mut i = 0;
        let src_len = src_limbs.len();
        while i < src_len {
            let first = &src_limbs[i];
            let second = if i + 1 < src_len {
                Some(&src_limbs[i + 1])
            } else {
                None
            };

            let first_max_bits_per_limb =
                params.bits_per_limb + crate::overhead!(first.1 + F::one());
            let second_max_bits_per_limb = if let Some(second) = second {
                params.bits_per_limb + crate::overhead!(second.1 + F::one())
            } else {
                0
            };

            if let Some(second) = second {
                if first_max_bits_per_limb + second_max_bits_per_limb <= capacity {
                    let adjustment_factor =
                        &adjustment_factor_lookup_table[second_max_bits_per_limb];

                    dest_limbs.push(first.0 * adjustment_factor + second.0);
                    i += 2;
                } else {
                    dest_limbs.push(first.0);
                    i += 1;
                }
            } else {
                dest_limbs.push(first.0);
                i += 1;
            }
        }

        dest_limbs
    }

    /// Convert a `TargetField` element into limbs (not constraints)
    /// This is an internal function that would be reused by a number of other functions
    pub fn get_limbs_representations<TargetField: PrimeField>(
        elem: &TargetField,
        optimization_type: OptimizationType,
    ) -> SmallVec<[F; 10]> {
        Self::get_limbs_representations_from_big_integer::<TargetField>(
            &elem.to_repr(),
            optimization_type,
        )
    }

    /// Obtain the limbs directly from a big int
    pub fn get_limbs_representations_from_big_integer<TargetField: PrimeField>(
        elem: &<TargetField as PrimeField>::BigInteger,
        optimization_type: OptimizationType,
    ) -> SmallVec<[F; 10]> {
        let params = get_params(
            TargetField::size_in_bits(),
            F::size_in_bits(),
            optimization_type,
        );

        // Push the lower limbs first
        let mut limbs: SmallVec<[F; 10]> = SmallVec::new();
        let mut cur = *elem;
        for _ in 0..params.num_limbs {
            let cur_bits = cur.to_bits_be(); // `to_bits` is big endian
            let cur_mod_r = <F as PrimeField>::BigInteger::from_bits_be(
                &cur_bits[cur_bits.len() - params.bits_per_limb..],
            )
            .unwrap(); // therefore, the lowest `bits_per_non_top_limb` bits is what we want.
            limbs.push(F::from_repr(cur_mod_r).unwrap());
            cur.divn(params.bits_per_limb as u32);
        }

        // then we reserve, so that the limbs are ``big limb first''
        limbs.reverse();

        limbs
    }

    /// Push elements to sponge, treated in the non-native field representations.
    pub fn push_elements_to_sponge<TargetField: PrimeField>(
        &mut self,
        src: impl IntoIterator<Item = TargetField>,
        ty: OptimizationType,
    ) {
        let mut src_limbs = Vec::<(F, F)>::new();

        for elem in src {
            let limbs = Self::get_limbs_representations(&elem, ty);
            for limb in limbs.iter() {
                src_limbs.push((*limb, F::one()));
                // specifically set to one, since most gadgets in the constraint world would not have zero noise (due to the relatively weak normal form testing in `alloc`)
            }
        }

        let dest_limbs = Self::compress_elements::<TargetField>(&src_limbs, ty);
        self.absorb_native_field_elements(&dest_limbs);
    }

    /// obtain random bits from hashchain.
    /// not guaranteed to be uniformly distributed, should only be used in certain situations.
    pub fn get_bits(&mut self, num_bits: usize) -> Vec<bool> {
        let bits_per_element = F::size_in_bits() - 1;
        let num_elements = (num_bits + bits_per_element - 1) / bits_per_element;

        let src_elements = self.squeeze_native_field_elements(num_elements);
        let mut dest_bits = Vec::<bool>::with_capacity(num_elements * bits_per_element);

        let skip = (F::Parameters::REPR_SHAVE_BITS + 1) as usize;
        for elem in src_elements.iter() {
            // discard the highest bit
            let elem_bits = elem.to_repr().to_bits_be();
            dest_bits.extend_from_slice(&elem_bits[skip..]);
        }
        dest_bits.truncate(num_bits);

        dest_bits
    }

    /// obtain random field elements from hashchain.
    /// not guaranteed to be uniformly distributed, should only be used in certain situations.
    pub fn get_fe<TargetField: PrimeField>(
        &mut self,
        num_elements: usize,
        outputs_short_elements: bool,
    ) -> SmallVec<[TargetField; 10]> {
        let num_bits_per_nonnative = if outputs_short_elements {
            168
        } else {
            TargetField::size_in_bits() - 1 // also omit the highest bit
        };
        let bits = self.get_bits(num_bits_per_nonnative * num_elements);

        let mut lookup_table = Vec::<TargetField>::new();
        let mut cur = TargetField::one();
        for _ in 0..num_bits_per_nonnative {
            lookup_table.push(cur);
            cur.double_in_place();
        }

        let dest_elements = bits
            .chunks_exact(num_bits_per_nonnative)
            .map(|per_nonnative_bits| {
                // technically, this can be done via BigInterger::from_bits; here, we use this method for consistency with the gadget counterpart
                let mut res = TargetField::zero();

                for (i, bit) in per_nonnative_bits.iter().rev().enumerate() {
                    if *bit {
                        res += &lookup_table[i];
                    }
                }
                res
            })
            .collect::<SmallVec<_>>();
        debug_assert_eq!(dest_elements.len(), num_elements);

        dest_elements
    }
}
