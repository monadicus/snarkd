use core::{
    fmt,
    ops::{Add, AddAssign, Mul, Neg, Sub},
};
use indexmap::{map::Entry, IndexMap};

use crate::bls12_377::{Field, Fp};

use super::{Mode, Variable};

#[derive(Clone)]
pub struct LinearCombination {
    pub(crate) constant: Fp,
    pub(crate) terms: IndexMap<Variable, Fp>,
    /// The value of this linear combination, defined as the sum of the `terms` and `constant`.
    value: Fp,
}

impl LinearCombination {
    /// Returns the `zero` constant.
    pub(crate) fn zero() -> Self {
        Self {
            constant: Fp::ZERO,
            terms: Default::default(),
            value: Fp::ZERO,
        }
    }

    /// Returns the `one` constant.
    pub(crate) fn one() -> Self {
        Self {
            constant: Fp::ONE,
            terms: Default::default(),
            value: Fp::ONE,
        }
    }

    /// Returns `true` if there are no terms in the linear combination.
    pub fn is_constant(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns `true` if there is exactly one term with a coefficient of one,
    /// and the term contains a public variable.
    pub fn is_public(&self) -> bool {
        self.constant.is_zero()
            && self.terms.len() == 1
            && match self.terms.iter().next() {
                Some((Variable::Public(..), coefficient)) => *coefficient == Fp::ONE,
                _ => false,
            }
    }

    /// Returns `true` if the linear combination is not constant or public.
    pub fn is_private(&self) -> bool {
        !self.is_constant() && !self.is_public()
    }

    /// Returns the mode of this linear combination.
    pub fn mode(&self) -> Mode {
        if self.is_constant() {
            Mode::Constant
        } else if self.is_public() {
            Mode::Public
        } else {
            Mode::Private
        }
    }

    /// Returns the computed value of the linear combination.
    pub fn value(&self) -> Fp {
        self.value
    }

    ///
    /// Returns `true` if the linear combination represents a `Boolean` type,
    /// and is well-formed.
    ///
    /// Properties:
    /// 1. Either `constant` or `terms` is utilized, however never both.
    /// 2. Every individual variable in the linear combination must always be either `0` or `1`.
    /// 3. The value of the linear combination must always be either `0` or `1`.
    ///
    pub fn is_boolean_type(&self) -> bool {
        // Constant case (enforce Property 1)
        if self.terms.is_empty() {
            self.constant.is_zero() || self.constant.is_one()
        }
        // Public and private cases (enforce Property 1)
        else if self.constant.is_zero() {
            // Enforce property 2.
            if self
                .terms
                .iter()
                .all(|(v, _)| !(v.value().is_zero() || v.value().is_one()))
            {
                eprintln!("Property 2 of the `Boolean` type was violated in {self}");
                return false;
            }

            // Enforce property 3.
            if !(self.value.is_zero() || self.value.is_one()) {
                eprintln!("Property 3 of the `Boolean` type was violated");
                return false;
            }

            true
        } else {
            // Property 1 of the `Boolean` type was violated.
            // Both self.constant and self.terms contain elements.
            eprintln!("Both LC::constant and LC::terms contain elements, which is a violation");
            false
        }
    }

    /// Returns only the constant value (excluding the terms) in the linear combination.
    pub(super) fn to_constant(&self) -> Fp {
        self.constant
    }

    /// Returns the terms (excluding the constant value) in the linear combination.
    pub(super) fn to_terms(&self) -> &IndexMap<Variable, Fp> {
        &self.terms
    }

    /// Returns the number of addition gates in the linear combination.
    pub(super) fn num_additions(&self) -> u64 {
        // Increment by one if the constant is nonzero and the number of terms is nonzero.
        match !self.constant.is_zero() && !self.terms.is_empty() {
            true => self.terms.len() as u64,
            false => (self.terms.len() as u64).saturating_sub(1),
        }
    }
}

impl From<Variable> for LinearCombination {
    fn from(variable: Variable) -> Self {
        Self::from(&variable)
    }
}

impl From<&Variable> for LinearCombination {
    fn from(variable: &Variable) -> Self {
        Self::from(&[variable.clone()])
    }
}

impl<const N: usize> From<[Variable; N]> for LinearCombination {
    fn from(variables: [Variable; N]) -> Self {
        Self::from(&variables[..])
    }
}

impl<const N: usize> From<&[Variable; N]> for LinearCombination {
    fn from(variables: &[Variable; N]) -> Self {
        Self::from(&variables[..])
    }
}

impl From<Vec<Variable>> for LinearCombination {
    fn from(variables: Vec<Variable>) -> Self {
        Self::from(variables.as_slice())
    }
}

impl From<&Vec<Variable>> for LinearCombination {
    fn from(variables: &Vec<Variable>) -> Self {
        Self::from(variables.as_slice())
    }
}

impl From<&[Variable]> for LinearCombination {
    fn from(variables: &[Variable]) -> Self {
        let mut output = Self::zero();
        for variable in variables {
            match variable.is_constant() {
                true => output.constant += variable.value(),
                false => {
                    match output.terms.entry(variable.clone()) {
                        Entry::Occupied(mut entry) => {
                            // Increment the existing coefficient by 1.
                            *entry.get_mut() += Fp::ONE;
                            // If the coefficient of the term is now zero, remove the entry.
                            if entry.get().is_zero() {
                                entry.remove_entry();
                            }
                        }
                        Entry::Vacant(entry) => {
                            // Insert the variable and a coefficient of 1 as a new term.
                            entry.insert(Fp::ONE);
                        }
                    }
                }
            }
            // Increment the value of the linear combination by the variable.
            output.value += variable.value();
        }
        output
    }
}

impl Neg for LinearCombination {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut output = self;
        output.constant = -output.constant;
        output
            .terms
            .iter_mut()
            .for_each(|(_, coefficient)| *coefficient = -(*coefficient));
        output.value = -output.value;
        output
    }
}

impl Neg for &LinearCombination {
    type Output = LinearCombination;

    #[inline]
    fn neg(self) -> Self::Output {
        -(self.clone())
    }
}

impl Add<Variable> for LinearCombination {
    type Output = Self;

    #[allow(clippy::op_ref)]
    fn add(self, other: Variable) -> Self::Output {
        self + &other
    }
}

impl Add<&Variable> for LinearCombination {
    type Output = Self;

    fn add(self, other: &Variable) -> Self::Output {
        self + Self::from(other)
    }
}

impl Add<Variable> for &LinearCombination {
    type Output = LinearCombination;

    #[allow(clippy::op_ref)]
    fn add(self, other: Variable) -> Self::Output {
        self.clone() + &other
    }
}

impl Add<LinearCombination> for LinearCombination {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self + &other
    }
}

impl Add<&LinearCombination> for LinearCombination {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        &self + other
    }
}

impl Add<LinearCombination> for &LinearCombination {
    type Output = LinearCombination;

    fn add(self, other: LinearCombination) -> Self::Output {
        self + &other
    }
}

impl Add<&LinearCombination> for &LinearCombination {
    type Output = LinearCombination;

    fn add(self, other: &LinearCombination) -> Self::Output {
        if self.constant.is_zero() && self.terms.is_empty() {
            other.clone()
        } else if other.constant.is_zero() && other.terms.is_empty() {
            self.clone()
        } else if self.terms.len() > other.terms.len() {
            let mut output = self.clone();
            output += other;
            output
        } else {
            let mut output = other.clone();
            output += self;
            output
        }
    }
}

impl AddAssign<LinearCombination> for LinearCombination {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl AddAssign<&LinearCombination> for LinearCombination {
    fn add_assign(&mut self, other: &Self) {
        // If `other` is empty, return immediately.
        if other.constant.is_zero() && other.terms.is_empty() {
            return;
        }

        if self.constant.is_zero() && self.terms.is_empty() {
            *self = other.clone();
        } else {
            // Add the constant value from `other` to `self`.
            self.constant += other.constant;

            // Add the terms from `other` to the terms of `self`.
            for (variable, coefficient) in other.terms.iter() {
                match variable.is_constant() {
                    true => panic!("Malformed linear combination found"),
                    false => {
                        match self.terms.entry(variable.clone()) {
                            Entry::Occupied(mut entry) => {
                                // Add the coefficient to the existing coefficient for this term.
                                *entry.get_mut() += *coefficient;
                                // If the coefficient of the term is now zero, remove the entry.
                                if entry.get().is_zero() {
                                    entry.remove_entry();
                                }
                            }
                            Entry::Vacant(entry) => {
                                // Insert the variable and coefficient as a new term.
                                entry.insert(*coefficient);
                            }
                        }
                    }
                }
            }

            // Add the value from `other` to `self`.
            self.value += other.value;
        }
    }
}

impl Sub<Variable> for LinearCombination {
    type Output = Self;

    #[allow(clippy::op_ref)]
    fn sub(self, other: Variable) -> Self::Output {
        self - &other
    }
}

impl Sub<&Variable> for LinearCombination {
    type Output = Self;

    fn sub(self, other: &Variable) -> Self::Output {
        self - Self::from(other)
    }
}

impl Sub<Variable> for &LinearCombination {
    type Output = LinearCombination;

    #[allow(clippy::op_ref)]
    fn sub(self, other: Variable) -> Self::Output {
        self.clone() - &other
    }
}

impl Sub<LinearCombination> for LinearCombination {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - &other
    }
}

impl Sub<&LinearCombination> for LinearCombination {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        &self - other
    }
}

impl Sub<LinearCombination> for &LinearCombination {
    type Output = LinearCombination;

    fn sub(self, other: LinearCombination) -> Self::Output {
        self - &other
    }
}

impl Sub<&LinearCombination> for &LinearCombination {
    type Output = LinearCombination;

    fn sub(self, other: &LinearCombination) -> Self::Output {
        self + &(-other)
    }
}

impl Mul<Fp> for LinearCombination {
    type Output = Self;

    #[allow(clippy::op_ref)]
    fn mul(self, coefficient: Fp) -> Self::Output {
        self * &coefficient
    }
}

impl Mul<&Fp> for LinearCombination {
    type Output = Self;

    fn mul(self, coefficient: &Fp) -> Self::Output {
        let mut output = self;
        output.constant *= coefficient;
        output
            .terms
            .iter_mut()
            .for_each(|(_, current_coefficient)| *current_coefficient *= coefficient);
        output.value *= coefficient;
        output
    }
}

impl Mul<Fp> for &LinearCombination {
    type Output = LinearCombination;

    #[allow(clippy::op_ref)]
    fn mul(self, coefficient: Fp) -> Self::Output {
        self * &coefficient
    }
}

impl Mul<&Fp> for &LinearCombination {
    type Output = LinearCombination;

    fn mul(self, coefficient: &Fp) -> Self::Output {
        self.clone() * coefficient
    }
}

impl fmt::Debug for LinearCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut output = format!("Constant({})", self.constant);

        // Sort the terms.
        let mut terms = self.terms.clone();
        terms.sort_keys();

        for (variable, coefficient) in &terms {
            output += &match (variable.mode(), coefficient.is_one()) {
                (Mode::Constant, _) => panic!(
                    "Malformed linear combination at: ({} * {:?})",
                    coefficient, variable
                ),
                (_, true) => format!(" + {:?}", variable),
                _ => format!(" + {} * {:?}", coefficient, variable),
            };
        }
        write!(f, "{}", output)
    }
}

impl fmt::Display for LinearCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
