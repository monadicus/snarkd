use anyhow::Result;
use indexmap::IndexMap;

use crate::{
    bls12_377::Fp,
    r1cs::{
        ConstraintSynthesizer, ConstraintSystem, Index as R1csIndex,
        LinearCombination as R1csLinearCombination, Variable as R1csVariable,
    },
};

use super::{Index, LinearCombination, Variable, R1CS};

#[derive(Clone, PartialEq, Eq, Hash)]
enum AssignmentVariable {
    Constant(Fp),
    Public(Index),
    Private(Index),
}

impl From<&Variable> for AssignmentVariable {
    /// Converts a variable to an assignment variable.
    fn from(variable: &Variable) -> Self {
        match variable {
            Variable::Constant(value) => Self::Constant(**value),
            Variable::Public(index, _) => Self::Public(*index),
            Variable::Private(index, _) => Self::Private(*index),
        }
    }
}

#[derive(Clone)]
struct AssignmentLC {
    constant: Fp,
    terms: IndexMap<AssignmentVariable, Fp>,
}

impl From<&LinearCombination> for AssignmentLC {
    /// Converts a linear combination to an assignment linear combination.
    fn from(lc: &LinearCombination) -> Self {
        Self {
            constant: lc.to_constant(),
            terms: FromIterator::from_iter(
                lc.to_terms()
                    .iter()
                    .map(|(variable, coefficient)| (variable.into(), *coefficient)),
            ),
        }
    }
}

/// A struct for tracking the mapping of variables from the virtual machine (first) to the gadget constraint system (second).
#[derive(Clone)]
pub struct Assignment {
    public: IndexMap<Index, Fp>,
    private: IndexMap<Index, Fp>,
    constraints: Vec<(AssignmentLC, AssignmentLC, AssignmentLC)>,
}

impl From<R1CS> for Assignment {
    /// Converts an R1CS to an assignment.
    fn from(r1cs: R1CS) -> Self {
        Self {
            public: FromIterator::from_iter(
                r1cs.to_public_variables()
                    .iter()
                    .map(|variable| (variable.index(), variable.value())),
            ),
            private: FromIterator::from_iter(
                r1cs.to_private_variables()
                    .iter()
                    .map(|variable| (variable.index(), variable.value())),
            ),
            constraints: FromIterator::from_iter(r1cs.to_constraints().iter().map(|constraint| {
                let (a, b, c) = constraint.to_terms();
                (a.into(), b.into(), c.into())
            })),
        }
    }
}

impl Assignment {
    /// Returns the public inputs of the assignment.
    pub fn public_inputs(&self) -> Vec<Fp> {
        self.public.values().cloned().collect()
    }

    /// Returns the number of public variables in the assignment.
    pub fn num_public(&self) -> u64 {
        self.public.len() as u64
    }

    /// Returns the number of private variables in the assignment.
    pub fn num_private(&self) -> u64 {
        self.private.len() as u64
    }

    /// Returns the number of constraints in the assignment.
    pub fn num_constraints(&self) -> u64 {
        self.constraints.len() as u64
    }
}

impl ConstraintSynthesizer<Fp> for Assignment {
    /// Synthesizes the constraints from the environment into a `snarkvm_r1cs`-compliant constraint system.
    fn generate_constraints<CS: ConstraintSystem<Field = Fp>>(&self, cs: &mut CS) -> Result<()> {
        /// A struct for tracking the mapping of variables from the virtual machine (first) to the gadget constraint system (second).
        struct Converter {
            public: IndexMap<u64, R1csVariable>,
            private: IndexMap<u64, R1csVariable>,
        }

        let mut converter = Converter {
            public: Default::default(),
            private: Default::default(),
        };

        // Ensure the given `cs` is starting off clean.
        assert_eq!(1, cs.num_public_variables());
        assert_eq!(0, cs.num_private_variables());
        assert_eq!(0, cs.num_constraints());

        // Allocate the public variables.
        for (i, (index, value)) in self.public.iter().enumerate() {
            assert_eq!(
                i as u64, *index,
                "Public variables in first system must be processed in lexicographic order"
            );

            let gadget = cs.alloc_input(|| format!("Public {i}"), || Ok(*value))?;

            assert_eq!(
                R1csIndex::Public((index + 1) as usize),
                gadget.get_unchecked(),
                "Public variables in the second system must match the first system (with an off-by-1 for the public case)"
            );

            let result = converter.public.insert(*index, gadget);

            assert!(
                result.is_none(),
                "Overwrote an existing public variable in the converter"
            );
        }

        // Allocate the private variables.
        for (i, (index, value)) in self.private.iter().enumerate() {
            assert_eq!(
                i as u64, *index,
                "Private variables in first system must be processed in lexicographic order"
            );

            let gadget = cs.alloc(|| format!("Private {i}"), || Ok(*value))?;

            assert_eq!(
                R1csIndex::Private(i),
                gadget.get_unchecked(),
                "Private variables in the second system must match the first system"
            );

            let result = converter.private.insert(*index, gadget);

            assert!(
                result.is_none(),
                "Overwrote an existing private variable in the converter"
            );
        }

        // Enforce all of the constraints.
        for (i, (a, b, c)) in self.constraints.iter().enumerate() {
            // Converts terms from one linear combination in the first system to the second system.
            let convert_linear_combination = |lc: &AssignmentLC| -> R1csLinearCombination<Fp> {
                // Initialize a linear combination for the second system.
                let mut linear_combination = R1csLinearCombination::<Fp>::zero();

                // Keep an accumulator for constant values in the linear combination.
                let mut constant_accumulator = lc.constant;
                // Process every term in the linear combination.
                for (variable, coefficient) in lc.terms.iter() {
                    match variable {
                        AssignmentVariable::Constant(value) => {
                            constant_accumulator += *value;
                        }
                        AssignmentVariable::Public(index) => {
                            let gadget = converter.public.get(index).unwrap();
                            assert_eq!(
                                R1csIndex::Public((index + 1) as usize),
                                gadget.get_unchecked(),
                                "Failed during constraint translation. The public variable in the second system must match the first system (with an off-by-1 for the public case)"
                            );
                            linear_combination += (*coefficient, *gadget);
                        }
                        AssignmentVariable::Private(index) => {
                            let gadget = converter.private.get(index).unwrap();
                            assert_eq!(
                                R1csIndex::Private(*index as usize),
                                gadget.get_unchecked(),
                                "Failed during constraint translation. The private variable in the second system must match the first system"
                            );
                            linear_combination += (*coefficient, *gadget);
                        }
                    }
                }

                // Finally, add the accumulated constant value to the linear combination.
                linear_combination += (
                    constant_accumulator,
                    R1csVariable::new_unchecked(R1csIndex::Public(0)),
                );

                // Return the linear combination of the second system.
                linear_combination
            };

            cs.enforce(
                || format!("Constraint {i}"),
                |lc| lc + convert_linear_combination(a),
                |lc| lc + convert_linear_combination(b),
                |lc| lc + convert_linear_combination(c),
            );
        }

        // Ensure the given `cs` matches in size with the first system.
        assert_eq!(self.num_public() + 1, cs.num_public_variables() as u64);
        assert_eq!(self.num_private(), cs.num_private_variables() as u64);
        assert_eq!(self.num_constraints(), cs.num_constraints() as u64);

        Ok(())
    }
}
