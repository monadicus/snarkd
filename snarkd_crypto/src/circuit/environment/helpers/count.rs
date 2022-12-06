use core::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

pub type Constant = Measurement<u64>;
pub type Public = Measurement<u64>;
pub type Private = Measurement<u64>;
pub type Constraints = Measurement<u64>;

/// A helper struct for tracking the number of constants, public inputs, private inputs, and constraints.
#[derive(Copy, Clone, Debug)]
pub struct Count(pub Constant, pub Public, pub Private, pub Constraints);

impl Count {
    /// Returns a new `Count` whose constituent metrics are all `Exact`.
    pub const fn zero() -> Self {
        Count(
            Measurement::Exact(0),
            Measurement::Exact(0),
            Measurement::Exact(0),
            Measurement::Exact(0),
        )
    }

    /// Returns a new `Count` whose constituent metrics are all `Exact`.
    pub const fn is(
        num_constants: u64,
        num_public: u64,
        num_private: u64,
        num_constraints: u64,
    ) -> Self {
        Count(
            Measurement::Exact(num_constants),
            Measurement::Exact(num_public),
            Measurement::Exact(num_private),
            Measurement::Exact(num_constraints),
        )
    }

    /// Returns a new `Count` whose constituent metrics are all inclusive `UpperBound`.
    pub const fn less_than(
        num_constants: u64,
        num_public: u64,
        num_private: u64,
        num_constraints: u64,
    ) -> Self {
        Count(
            Measurement::UpperBound(num_constants),
            Measurement::UpperBound(num_public),
            Measurement::UpperBound(num_private),
            Measurement::UpperBound(num_constraints),
        )
    }

    /// Returns `true` if all constituent metrics match.
    pub fn matches(
        &self,
        num_constants: u64,
        num_public: u64,
        num_private: u64,
        num_constraints: u64,
    ) -> bool {
        self.0.matches(num_constants)
            && self.1.matches(num_public)
            && self.2.matches(num_private)
            && self.3.matches(num_constraints)
    }
}

impl Add for Count {
    type Output = Count;

    /// Adds the `Count` to another `Count` by summing its constituent metrics.
    fn add(self, other: Count) -> Self::Output {
        Count(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Mul<u64> for Count {
    type Output = Count;

    /// Scales the `Count` by a `u64`.
    fn mul(self, other: u64) -> Self::Output {
        Count(
            self.0 * other,
            self.1 * other,
            self.2 * other,
            self.3 * other,
        )
    }
}

impl Mul<Count> for u64 {
    type Output = Count;

    /// Scales the `Count` by a `u64`.
    fn mul(self, other: Count) -> Self::Output {
        other * self
    }
}

/// A `Measurement` is a quantity that can be measured.
/// The variants of the `Measurement` defines a condition associated with the measurable quantity.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Measurement<V: Copy + Debug + Ord + Add<Output = V> + Sub<Output = V> + Mul<Output = V>> {
    Exact(V),
    Range(V, V),
    UpperBound(V),
}

impl<V: Copy + Debug + Ord + Add<Output = V> + Sub<Output = V> + Mul<Output = V>> Measurement<V> {
    /// Returns `true` if the value matches the metric.
    ///
    /// For an `Exact` metric, `value` must be equal to the exact value defined by the metric.
    /// For a `Range` metric, `value` must be satisfy lower bound and the upper bound.
    /// For an `UpperBound` metric, `value` must be satisfy the upper bound.
    pub fn matches(&self, candidate: V) -> bool {
        let outcome = match self {
            Measurement::Exact(expected) => *expected == candidate,
            Measurement::Range(lower, upper) => candidate >= *lower && candidate <= *upper,
            Measurement::UpperBound(bound) => candidate <= *bound,
        };

        if !outcome {
            eprintln!(
                "Metrics claims the count should be {:?}, found {:?} during synthesis",
                self, candidate
            );
        }

        outcome
    }
}

impl<V: Copy + Debug + Ord + Add<Output = V> + Sub<Output = V> + Mul<Output = V>> Add
    for Measurement<V>
{
    type Output = Measurement<V>;

    /// Adds two variants of `Measurement` together, returning the newly-summed `Measurement`.
    fn add(self, other: Measurement<V>) -> Self::Output {
        match (self, other) {
            // `Exact` + `Exact` => `Exact`
            (Measurement::Exact(exact_a), Measurement::Exact(exact_b)) => {
                Measurement::Exact(exact_a + exact_b)
            }
            // `Range` + `Range` => `Range`
            (Measurement::Range(lower_a, upper_a), Measurement::Range(lower_b, upper_b)) => {
                Measurement::Range(lower_a + lower_b, upper_a + upper_b)
            }
            // `UpperBound` + `UpperBound` => `UpperBound`
            (Measurement::UpperBound(upper_a), Measurement::UpperBound(upper_b)) => {
                Measurement::UpperBound(upper_a + upper_b)
            }
            // `Exact` + `Range` => `Range`
            // `Range` + `Exact` => `Range`
            (Measurement::Exact(exact), Measurement::Range(lower, upper))
            | (Measurement::Range(lower, upper), Measurement::Exact(exact)) => {
                Measurement::Range(exact + lower, exact + upper)
            }
            // `Exact` + `UpperBound` => `UpperBound`
            // `UpperBound` + `Exact` => `UpperBound`
            (Measurement::Exact(exact), Measurement::UpperBound(upper))
            | (Measurement::UpperBound(upper), Measurement::Exact(exact)) => {
                Measurement::UpperBound(exact + upper)
            }
            // `Range` + `UpperBound` => `Range`
            // `UpperBound` + `Range` => `Range`
            (Measurement::Range(lower, upper_a), Measurement::UpperBound(upper_b))
            | (Measurement::UpperBound(upper_a), Measurement::Range(lower, upper_b)) => {
                Measurement::Range(lower, upper_a + upper_b)
            }
        }
    }
}

impl<V: Copy + Debug + Ord + Add<Output = V> + Sub<Output = V> + Mul<Output = V>> Mul<V>
    for Measurement<V>
{
    type Output = Measurement<V>;

    /// Scales the `Measurement` by a value.
    fn mul(self, other: V) -> Self::Output {
        match self {
            Measurement::Exact(value) => Measurement::Exact(value * other),
            Measurement::Range(lower, upper) => Measurement::Range(lower * other, upper * other),
            Measurement::UpperBound(bound) => Measurement::UpperBound(bound * other),
        }
    }
}
