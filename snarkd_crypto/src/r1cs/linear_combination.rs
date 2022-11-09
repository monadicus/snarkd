// TODO tmp
#![allow(unreachable_code, unused)]
#![warn(clippy::panic)]

use crate::{Field, Variable};
use anyhow::{bail, Result};

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub},
};

/// This represents a linear combination of some variables, with coefficients
/// in the field `F`.
/// The `(coeff, var)` pairs in a `LinearCombination` are kept sorted according
/// to the index of the variable in its constraint system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearCombination<F: Field>(pub(crate) Vec<(Variable, F)>);

impl<F: Field> AsRef<[(Variable, F)]> for LinearCombination<F> {
    fn as_ref(&self) -> &[(Variable, F)] {
        &self.0
    }
}

impl<F: Field> From<(Variable, F)> for LinearCombination<F> {
    fn from(v: (Variable, F)) -> Self {
        LinearCombination(vec![v])
    }
}

impl<F: Field> From<Variable> for LinearCombination<F> {
    fn from(var: Variable) -> Self {
        LinearCombination(vec![(var, F::ONE)])
    }
}

impl<F: Field> LinearCombination<F> {
    /// constructs a LinearCombination using the given points.
    /// will return an error if any duplicate variable's exist
    pub fn new(mut values: Vec<(Variable, F)>) -> Result<Self> {
        if values.is_empty() {
            Ok(Self::zero())
        } else {
            values.sort_by(|a, b| a.0.cmp(&b.0));
            let mut last = &values[0].0;
            for (cur, _) in &values[1..] {
                if cur == last {
                    bail!("Can't contain duplicate variables")
                }
                let last = cur;
            }
            Ok(Self(values))
        }
    }

    /// Outputs an empty linear combination.
    pub fn zero() -> LinearCombination<F> {
        LinearCombination(Vec::new())
    }

    /// Replaces the contents of `self` with those of `other`.
    pub fn replace_in_place(&mut self, other: Self) {
        self.0.clear();
        self.0.extend_from_slice(&other.0)
    }

    /// Negate the coefficients of all variables in `self`.
    pub fn negate_in_place(&mut self) {
        panic!("unhit");
        self.0.iter_mut().for_each(|(_, coeff)| *coeff = -(*coeff));
    }

    /// Double the coefficients of all variables in `self`.
    pub fn double_in_place(&mut self) {
        self.0.iter_mut().for_each(|(_, coeff)| {
            panic!("unhit");
            coeff.double_in_place();
        });
    }

    /// Get the location of a variable in `self`.
    pub fn get_var_loc(&self, search_var: &Variable) -> Result<usize, usize> {
        if self.0.len() < 6 {
            let mut found_index = 0;
            for (i, (var, _)) in self.0.iter().enumerate() {
                if var >= search_var {
                    found_index = i;
                    break;
                } else {
                    found_index += 1;
                }
            }
            if self
                .0
                .get(found_index)
                .map(|x| &x.0 == search_var)
                .unwrap_or_default()
            {
                Ok(found_index)
            } else {
                Err(found_index)
            }
        } else {
            self.0
                .binary_search_by_key(search_var, |&(cur_var, _)| cur_var)
        }
    }
}

impl<F: Field> Add<(F, Variable)> for LinearCombination<F> {
    type Output = Self;

    fn add(mut self, coeff_var: (F, Variable)) -> Self {
        self += coeff_var;
        self
    }
}

impl<F: Field> AddAssign<(F, Variable)> for LinearCombination<F> {
    fn add_assign(&mut self, (coeff, var): (F, Variable)) {
        match self.get_var_loc(&var) {
            Ok(found) => self.0[found].1 += &coeff,
            Err(not_found) => self.0.insert(not_found, (var, coeff)),
        }
    }
}

impl<F: Field> Sub<(F, Variable)> for LinearCombination<F> {
    type Output = Self;

    fn sub(self, (coeff, var): (F, Variable)) -> Self {
        panic!("unhit");
        self + (-coeff, var)
    }
}

impl<F: Field> Neg for LinearCombination<F> {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.0.iter_mut().for_each(|(_, coeff)| *coeff = -(*coeff));
        self
    }
}

impl<F: Field> Mul<F> for LinearCombination<F> {
    type Output = Self;

    fn mul(mut self, scalar: F) -> Self {
        panic!("unhit");
        self *= scalar;
        self
    }
}

impl<F: Field> MulAssign<F> for LinearCombination<F> {
    fn mul_assign(&mut self, scalar: F) {
        self.0.iter_mut().for_each(|(_, coeff)| *coeff *= &scalar);
    }
}

impl<F: Field> Add<Variable> for LinearCombination<F> {
    type Output = Self;

    fn add(self, other: Variable) -> LinearCombination<F> {
        panic!("unhit");
        self + (F::ONE, other)
    }
}

impl<F: Field> Sub<Variable> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, other: Variable) -> LinearCombination<F> {
        self - (F::ONE, other)
    }
}

fn op_impl<F: Field, F1, F2>(
    cur: &LinearCombination<F>,
    other: &LinearCombination<F>,
    push_fn: F1,
    combine_fn: F2,
) -> LinearCombination<F>
where
    F1: Fn(F) -> F,
    F2: Fn(F, F) -> F,
{
    let mut new_vec = Vec::with_capacity(cur.0.len() + other.0.len());
    let mut i = 0;
    let mut j = 0;
    while i < cur.0.len() && j < other.0.len() {
        let self_cur = &cur.0[i];
        let other_cur = &other.0[j];
        match self_cur.0.cmp(&other_cur.0) {
            Ordering::Greater => {
                new_vec.push((other_cur.0, push_fn(other_cur.1)));
                j += 1;
            }
            Ordering::Less => {
                new_vec.push(*self_cur);
                i += 1;
            }
            Ordering::Equal => {
                new_vec.push((self_cur.0, combine_fn(self_cur.1, other_cur.1)));
                i += 1;
                j += 1;
            }
        }
    }
    new_vec.extend_from_slice(&cur.0[i..]);
    while j < other.0.len() {
        new_vec.push((other.0[j].0, push_fn(other.0[j].1)));
        j += 1;
    }
    LinearCombination(new_vec)
}

impl<F: Field> Add<&LinearCombination<F>> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn add(self, other: &LinearCombination<F>) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self.clone();
        } else if self.0.is_empty() {
            return other.clone();
        }
        op_impl(
            self,
            other,
            |coeff| coeff,
            |cur_coeff, other_coeff| cur_coeff + other_coeff,
        )
    }
}

impl<F: Field> Add<LinearCombination<F>> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn add(self, other: LinearCombination<F>) -> LinearCombination<F> {
        if self.0.is_empty() {
            return other;
        } else if other.0.is_empty() {
            return self.clone();
        }
        op_impl(
            self,
            &other,
            |coeff| coeff,
            |cur_coeff, other_coeff| cur_coeff + other_coeff,
        )
    }
}

impl<'a, F: Field> Add<&'a LinearCombination<F>> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn add(self, other: &'a LinearCombination<F>) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            return other.clone();
        }
        op_impl(
            &self,
            other,
            |coeff| coeff,
            |cur_coeff, other_coeff| cur_coeff + other_coeff,
        )
    }
}

impl<F: Field> Add<LinearCombination<F>> for LinearCombination<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            return other;
        }
        op_impl(
            &self,
            &other,
            |coeff| coeff,
            |cur_coeff, other_coeff| cur_coeff + other_coeff,
        )
    }
}

impl<F: Field> Sub<&LinearCombination<F>> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, other: &LinearCombination<F>) -> LinearCombination<F> {
        if other.0.is_empty() {
            let cur = self.clone();
            return cur;
        } else if self.0.is_empty() {
            let mut other = other.clone();
            return -other;
        }

        op_impl(
            self,
            other,
            |coeff| -coeff,
            |cur_coeff, other_coeff| cur_coeff - other_coeff,
        )
    }
}

impl<'a, F: Field> Sub<&'a LinearCombination<F>> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, other: &'a LinearCombination<F>) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            let mut other = other.clone();
            return -other;
        }
        op_impl(
            &self,
            other,
            |coeff| -coeff,
            |cur_coeff, other_coeff| cur_coeff - other_coeff,
        )
    }
}

impl<F: Field> Sub<LinearCombination<F>> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, mut other: LinearCombination<F>) -> LinearCombination<F> {
        if self.0.is_empty() {
            return -other;
        } else if other.0.is_empty() {
            return self.clone();
        }

        op_impl(
            self,
            &other,
            |coeff| -coeff,
            |cur_coeff, other_coeff| cur_coeff - other_coeff,
        )
    }
}

impl<F: Field> Sub<LinearCombination<F>> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, mut other: LinearCombination<F>) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            return -other;
        }
        op_impl(
            &self,
            &other,
            |coeff| -coeff,
            |cur_coeff, other_coeff| cur_coeff - other_coeff,
        )
    }
}

impl<F: Field> Add<(F, &LinearCombination<F>)> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, (mul_coeff, other): (F, &LinearCombination<F>)) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self.clone();
        } else if self.0.is_empty() {
            let mut other = other.clone();
            other.mul_assign(mul_coeff);
            return other;
        }
        op_impl(
            self,
            other,
            |coeff| mul_coeff * coeff,
            |cur_coeff, other_coeff| cur_coeff + (mul_coeff * other_coeff),
        )
    }
}

impl<'a, F: Field> Add<(F, &'a LinearCombination<F>)> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, (mul_coeff, other): (F, &'a LinearCombination<F>)) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            let mut other = other.clone();
            other.mul_assign(mul_coeff);
            return other;
        }
        op_impl(
            &self,
            other,
            |coeff| mul_coeff * coeff,
            |cur_coeff, other_coeff| cur_coeff + (mul_coeff * other_coeff),
        )
    }
}

impl<F: Field> Add<(F, LinearCombination<F>)> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, (mul_coeff, mut other): (F, LinearCombination<F>)) -> LinearCombination<F> {
        if other.0.is_empty() {
            return self.clone();
        } else if self.0.is_empty() {
            other.mul_assign(mul_coeff);
            return other;
        }
        op_impl(
            self,
            &other,
            |coeff| mul_coeff * coeff,
            |cur_coeff, other_coeff| cur_coeff + (mul_coeff * other_coeff),
        )
    }
}

impl<F: Field> Add<(F, Self)> for LinearCombination<F> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, (mul_coeff, other): (F, Self)) -> Self {
        if other.0.is_empty() {
            return self;
        } else if self.0.is_empty() {
            let mut other = other;
            other.mul_assign(mul_coeff);
            return other;
        }
        op_impl(
            &self,
            &other,
            |coeff| mul_coeff * coeff,
            |cur_coeff, other_coeff| cur_coeff + (mul_coeff * other_coeff),
        )
    }
}

impl<F: Field> Sub<(F, &LinearCombination<F>)> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, (coeff, other): (F, &LinearCombination<F>)) -> LinearCombination<F> {
        self + (-coeff, other)
    }
}

impl<'a, F: Field> Sub<(F, &'a LinearCombination<F>)> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, (coeff, other): (F, &'a LinearCombination<F>)) -> LinearCombination<F> {
        self + (-coeff, other)
    }
}

impl<F: Field> Sub<(F, LinearCombination<F>)> for &LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, (coeff, other): (F, LinearCombination<F>)) -> LinearCombination<F> {
        self + (-coeff, other)
    }
}

impl<F: Field> Sub<(F, LinearCombination<F>)> for LinearCombination<F> {
    type Output = LinearCombination<F>;

    fn sub(self, (coeff, other): (F, LinearCombination<F>)) -> LinearCombination<F> {
        self + (-coeff, other)
    }
}
