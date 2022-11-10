use crate::{
    bls12_377::Field,
    r1cs::{LinearCombination, Variable},
};

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

/// Either a `Variable` or a `LinearCombination`.
#[derive(Clone, Debug)]
pub enum ConstraintVariable<F: Field> {
    /// A wrapper around a `LinearCombination`.
    LC(LinearCombination<F>),
    /// A wrapper around a `Variable`.
    Var(Variable),
}

impl<F: Field> From<Variable> for ConstraintVariable<F> {
    fn from(var: Variable) -> Self {
        ConstraintVariable::Var(var)
    }
}

impl<F: Field> From<(Variable, F)> for ConstraintVariable<F> {
    fn from(v: (Variable, F)) -> Self {
        ConstraintVariable::LC(v.into())
    }
}

impl<F: Field> From<LinearCombination<F>> for ConstraintVariable<F> {
    fn from(lc: LinearCombination<F>) -> Self {
        ConstraintVariable::LC(lc)
    }
}

impl<F: Field> From<(F, LinearCombination<F>)> for ConstraintVariable<F> {
    fn from((coeff, mut lc): (F, LinearCombination<F>)) -> Self {
        lc *= coeff;
        ConstraintVariable::LC(lc)
    }
}

impl<F: Field> From<(ConstraintVariable<F>, F)> for ConstraintVariable<F> {
    fn from((var, coeff): (ConstraintVariable<F>, F)) -> Self {
        match var {
            ConstraintVariable::LC(lc) => (coeff, lc).into(),
            ConstraintVariable::Var(var) => (var, coeff).into(),
        }
    }
}

impl<F: Field> ConstraintVariable<F> {
    /// Returns an empty linear combination.

    pub fn zero() -> Self {
        ConstraintVariable::LC(LinearCombination::zero())
    }

    /// Negate the coefficients of all variables in `self`.
    pub fn negate_in_place(&mut self) {
        match self {
            ConstraintVariable::LC(ref mut lc) => lc.negate_in_place(),
            ConstraintVariable::Var(var) => *self = (*var, -F::ONE).into(),
        }
    }

    /// Double the coefficients of all variables in `self`.
    pub fn double_in_place(&mut self) {
        match self {
            ConstraintVariable::LC(lc) => lc.double_in_place(),
            ConstraintVariable::Var(var) => *self = (*var, F::ONE.double()).into(),
        }
    }
}

impl<F: Field> Add<LinearCombination<F>> for ConstraintVariable<F> {
    type Output = LinearCombination<F>;

    fn add(self, other_lc: LinearCombination<F>) -> LinearCombination<F> {
        match self {
            ConstraintVariable::LC(lc) => other_lc + lc,
            ConstraintVariable::Var(var) => other_lc + var,
        }
    }
}

impl<F: Field> Sub<LinearCombination<F>> for ConstraintVariable<F> {
    type Output = LinearCombination<F>;

    fn sub(self, other_lc: LinearCombination<F>) -> LinearCombination<F> {
        let result = match self {
            ConstraintVariable::LC(lc) => other_lc - lc,
            ConstraintVariable::Var(var) => other_lc - var,
        };
        -result
    }
}

impl<F: Field> Add<LinearCombination<F>> for &ConstraintVariable<F> {
    type Output = LinearCombination<F>;

    fn add(self, other_lc: LinearCombination<F>) -> LinearCombination<F> {
        match self {
            ConstraintVariable::LC(lc) => other_lc + lc,
            ConstraintVariable::Var(var) => other_lc + *var,
        }
    }
}

impl<F: Field> Sub<LinearCombination<F>> for &ConstraintVariable<F> {
    type Output = LinearCombination<F>;

    fn sub(self, other_lc: LinearCombination<F>) -> LinearCombination<F> {
        let result = match self {
            ConstraintVariable::LC(lc) => other_lc - lc,
            ConstraintVariable::Var(var) => other_lc - *var,
        };
        -result
    }
}

impl<F: Field> Add<(F, Variable)> for ConstraintVariable<F> {
    type Output = Self;

    fn add(self, var: (F, Variable)) -> Self {
        let lc = match self {
            ConstraintVariable::LC(lc) => lc + var,
            ConstraintVariable::Var(var2) => LinearCombination::from(var2) + var,
        };
        ConstraintVariable::LC(lc)
    }
}

impl<F: Field> AddAssign<(F, Variable)> for ConstraintVariable<F> {
    fn add_assign(&mut self, var: (F, Variable)) {
        match self {
            ConstraintVariable::LC(ref mut lc) => *lc += var,
            ConstraintVariable::Var(var2) => {
                *self = ConstraintVariable::LC(LinearCombination::from(*var2) + var)
            }
        };
    }
}

impl<F: Field> Neg for ConstraintVariable<F> {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.negate_in_place();
        self
    }
}

impl<F: Field> Mul<F> for ConstraintVariable<F> {
    type Output = Self;

    fn mul(self, scalar: F) -> Self {
        match self {
            ConstraintVariable::LC(lc) => ConstraintVariable::LC(lc * scalar),
            ConstraintVariable::Var(var) => (var, scalar).into(),
        }
    }
}

impl<F: Field> MulAssign<F> for ConstraintVariable<F> {
    fn mul_assign(&mut self, scalar: F) {
        match self {
            ConstraintVariable::LC(lc) => *lc *= scalar,
            ConstraintVariable::Var(var) => *self = (*var, scalar).into(),
        }
    }
}

impl<F: Field> Sub<(F, Variable)> for ConstraintVariable<F> {
    type Output = Self;

    fn sub(self, (coeff, var): (F, Variable)) -> Self {
        self + (-coeff, var)
    }
}

impl<F: Field> Add<Variable> for ConstraintVariable<F> {
    type Output = Self;

    fn add(self, other: Variable) -> Self {
        self + (F::ONE, other)
    }
}

impl<F: Field> Sub<Variable> for ConstraintVariable<F> {
    type Output = Self;

    fn sub(self, other: Variable) -> Self {
        self - (F::ONE, other)
    }
}

impl<'a, F: Field> Add<&'a Self> for ConstraintVariable<F> {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        let lc = match self {
            ConstraintVariable::LC(lc2) => lc2,
            ConstraintVariable::Var(var) => var.into(),
        };
        let lc2 = match other {
            ConstraintVariable::LC(lc2) => lc + lc2,
            ConstraintVariable::Var(var) => lc + *var,
        };
        ConstraintVariable::LC(lc2)
    }
}

impl<'a, F: Field> Sub<&'a Self> for ConstraintVariable<F> {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self {
        let lc = match self {
            ConstraintVariable::LC(lc2) => lc2,
            ConstraintVariable::Var(var) => var.into(),
        };
        let lc2 = match other {
            ConstraintVariable::LC(lc2) => lc - lc2,
            ConstraintVariable::Var(var) => lc - *var,
        };
        ConstraintVariable::LC(lc2)
    }
}

impl<F: Field> Add<&ConstraintVariable<F>> for &ConstraintVariable<F> {
    type Output = ConstraintVariable<F>;

    fn add(self, other: &ConstraintVariable<F>) -> Self::Output {
        (ConstraintVariable::zero() + self) + other
    }
}

impl<F: Field> Sub<&ConstraintVariable<F>> for &ConstraintVariable<F> {
    type Output = ConstraintVariable<F>;

    fn sub(self, other: &ConstraintVariable<F>) -> Self::Output {
        (ConstraintVariable::zero() + self) - other
    }
}

impl<'a, F: Field> Add<(F, &'a Self)> for ConstraintVariable<F> {
    type Output = Self;

    fn add(self, (coeff, other): (F, &'a Self)) -> Self {
        let mut lc = match self {
            ConstraintVariable::LC(lc2) => lc2,
            ConstraintVariable::Var(var) => LinearCombination::zero() + var,
        };

        lc = match other {
            ConstraintVariable::LC(lc2) => lc + (coeff, lc2),
            ConstraintVariable::Var(var) => lc + (coeff, *var),
        };
        ConstraintVariable::LC(lc)
    }
}

impl<'a, F: Field> Sub<(F, &'a Self)> for ConstraintVariable<F> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, (coeff, other): (F, &'a Self)) -> Self {
        let mut lc = match self {
            ConstraintVariable::LC(lc2) => lc2,
            ConstraintVariable::Var(var) => LinearCombination::zero() + var,
        };
        lc = match other {
            ConstraintVariable::LC(lc2) => lc - (coeff, lc2),
            ConstraintVariable::Var(var) => lc - (coeff, *var),
        };
        ConstraintVariable::LC(lc)
    }
}
