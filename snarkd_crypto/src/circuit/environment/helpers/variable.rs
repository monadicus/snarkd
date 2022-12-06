use core::{
    cmp::Ordering,
    fmt,
    ops::{Add, Sub},
};
use std::rc::Rc;

use crate::bls12_377::Fp;

use super::{LinearCombination, Mode};

pub type Index = u64;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Variable {
    Constant(Rc<Fp>),
    Public(Index, Rc<Fp>),
    Private(Index, Rc<Fp>),
}

impl Variable {
    ///
    /// Returns `true` if the variable is a constant.
    ///
    pub fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(..))
    }

    ///
    /// Returns `true` if the variable is public.
    ///
    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public(..))
    }

    ///
    /// Returns `true` if the variable is private.
    ///
    pub fn is_private(&self) -> bool {
        matches!(self, Self::Private(..))
    }

    ///
    /// Returns the mode of the variable.
    ///
    pub fn mode(&self) -> Mode {
        match self {
            Self::Constant(..) => Mode::Constant,
            Self::Public(..) => Mode::Public,
            Self::Private(..) => Mode::Private,
        }
    }

    ///
    /// Returns the relative index of the variable.
    ///
    pub fn index(&self) -> Index {
        match self {
            Self::Constant(..) => 0,
            Self::Public(index, ..) => *index,
            Self::Private(index, ..) => *index,
        }
    }

    ///
    /// Returns the value of the variable.
    ///
    pub fn value(&self) -> Fp {
        match self {
            Self::Constant(value) => **value,
            Self::Public(_, value) => **value,
            Self::Private(_, value) => **value,
        }
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Constant(v1), Self::Constant(v2)) => v1.cmp(v2),
            (Self::Constant(..), Self::Public(..)) => Ordering::Less,
            (Self::Constant(..), Self::Private(..)) => Ordering::Less,
            (Self::Public(..), Self::Constant(..)) => Ordering::Greater,
            (Self::Private(..), Self::Constant(..)) => Ordering::Greater,
            (Self::Public(i1, ..), Self::Public(i2, ..)) => i1.cmp(i2),
            (Self::Private(i1, ..), Self::Private(i2, ..)) => i1.cmp(i2),
            (Self::Public(..), Self::Private(..)) => Ordering::Less,
            (Self::Private(..), Self::Public(..)) => Ordering::Greater,
        }
    }
}

#[allow(clippy::op_ref)]
impl Add<Variable> for Variable {
    type Output = LinearCombination;

    fn add(self, other: Variable) -> Self::Output {
        self + &other
    }
}

#[allow(clippy::op_ref)]
impl Add<Variable> for &Variable {
    type Output = LinearCombination;

    fn add(self, other: Variable) -> Self::Output {
        self + &other
    }
}

#[allow(clippy::op_ref)]
impl Add<&Variable> for Variable {
    type Output = LinearCombination;

    fn add(self, other: &Variable) -> Self::Output {
        &self + other
    }
}

impl Add<&Variable> for &Variable {
    type Output = LinearCombination;

    fn add(self, other: &Variable) -> Self::Output {
        match (self, other) {
            (Variable::Constant(a), Variable::Constant(b)) => {
                Variable::Constant(Rc::new(**a + **b)).into()
            }
            (first, second) => LinearCombination::from([first.clone(), second.clone()]),
        }
    }
}

#[allow(clippy::op_ref)]
impl Add<LinearCombination> for Variable {
    type Output = LinearCombination;

    fn add(self, other: LinearCombination) -> Self::Output {
        self + &other
    }
}

#[allow(clippy::op_ref)]
impl Add<LinearCombination> for &Variable {
    type Output = LinearCombination;

    fn add(self, other: LinearCombination) -> Self::Output {
        self + &other
    }
}

#[allow(clippy::op_ref)]
impl Add<&LinearCombination> for Variable {
    type Output = LinearCombination;

    fn add(self, other: &LinearCombination) -> Self::Output {
        &self + other
    }
}

impl Add<&LinearCombination> for &Variable {
    type Output = LinearCombination;

    fn add(self, other: &LinearCombination) -> Self::Output {
        LinearCombination::from(self) + other
    }
}

#[allow(clippy::op_ref)]
impl Sub<Variable> for Variable {
    type Output = LinearCombination;

    fn sub(self, other: Variable) -> Self::Output {
        self - &other
    }
}

#[allow(clippy::op_ref)]
impl Sub<Variable> for &Variable {
    type Output = LinearCombination;

    fn sub(self, other: Variable) -> Self::Output {
        self - &other
    }
}

#[allow(clippy::op_ref)]
impl Sub<&Variable> for Variable {
    type Output = LinearCombination;

    fn sub(self, other: &Variable) -> Self::Output {
        &self - other
    }
}

impl Sub<&Variable> for &Variable {
    type Output = LinearCombination;

    fn sub(self, other: &Variable) -> Self::Output {
        match (self, other) {
            (Variable::Constant(a), Variable::Constant(b)) => {
                Variable::Constant(Rc::new(**a - **b)).into()
            }
            (first, second) => LinearCombination::from(first) - second,
        }
    }
}

#[allow(clippy::op_ref)]
impl Sub<LinearCombination> for Variable {
    type Output = LinearCombination;

    fn sub(self, other: LinearCombination) -> Self::Output {
        self - &other
    }
}

#[allow(clippy::op_ref)]
impl Sub<LinearCombination> for &Variable {
    type Output = LinearCombination;

    fn sub(self, other: LinearCombination) -> Self::Output {
        self - &other
    }
}

#[allow(clippy::op_ref)]
impl Sub<&LinearCombination> for Variable {
    type Output = LinearCombination;

    fn sub(self, other: &LinearCombination) -> Self::Output {
        &self - other
    }
}

impl Sub<&LinearCombination> for &Variable {
    type Output = LinearCombination;

    fn sub(self, other: &LinearCombination) -> Self::Output {
        LinearCombination::from(self) - other
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Constant(value) => format!("Constant({})", value),
                Self::Public(index, value) => format!("Public({}, {})", index, value),
                Self::Private(index, value) => format!("Private({}, {})", index, value),
            }
        )
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
