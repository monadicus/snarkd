use crate::circuit::{
    helpers::Measurement,
    rng_test_struct::{TestRng, Uniform},
};

const ITERATIONS: u64 = 1024;

#[test]
fn test_exact_matches() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        // Generate a random `Measurement` and candidate value.
        let value = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;
        let metric = Measurement::Exact(value);

        // Check that the metric is only satisfied if the candidate is equal to the value.
        assert!(metric.matches(value));
        if candidate == value {
            assert!(metric.matches(candidate));
        } else {
            assert!(!metric.matches(candidate));
        }
    }
}

#[test]
fn test_upper_matches() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        // Generate a random `Measurement::UpperBound` and candidate value.
        let upper = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;
        let metric = Measurement::UpperBound(upper);

        // Check that the metric is only satisfied if the candidate is less than upper.
        assert!(metric.matches(upper));
        if candidate <= upper {
            assert!(metric.matches(candidate));
        } else {
            assert!(!metric.matches(candidate));
        }
    }
}

#[test]
fn test_range_matches() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        // Generate a random `Measurement::UpperBound` and candidate value.
        let first_bound = u32::rand(&mut rng) as u64;
        let second_bound = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;
        let (metric, lower, upper) = if first_bound <= second_bound {
            (
                Measurement::Range(first_bound, second_bound),
                first_bound,
                second_bound,
            )
        } else {
            (
                Measurement::Range(second_bound, first_bound),
                second_bound,
                first_bound,
            )
        };

        // Check that the metric is only satisfied if the candidate is less than upper.
        assert!(metric.matches(lower));
        assert!(metric.matches(upper));
        if lower <= candidate && candidate <= upper {
            assert!(metric.matches(candidate));
        } else {
            assert!(!metric.matches(candidate));
        }
    }
}

// Test addition.

#[test]
fn test_exact_plus_exact() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let a = Measurement::Exact(first);
        let b = Measurement::Exact(second);
        let c = a + b;

        assert!(c.matches(first + second));
        if candidate == first + second {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_exact_plus_upper() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let a = Measurement::Exact(first);
        let b = Measurement::UpperBound(second);
        let c = a + b;

        assert!(c.matches(first + second));
        if candidate <= first + second {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_exact_plus_range() {
    let mut rng = TestRng::default();

    let value = u32::rand(&mut rng) as u64;
    let first_bound = u32::rand(&mut rng) as u64;
    let second_bound = u32::rand(&mut rng) as u64;
    let candidate = u32::rand(&mut rng) as u64;

    let a = Measurement::Exact(value);
    let (b, lower, upper) = if first_bound <= second_bound {
        (
            Measurement::Range(first_bound, second_bound),
            first_bound,
            second_bound,
        )
    } else {
        (
            Measurement::Range(second_bound, first_bound),
            second_bound,
            first_bound,
        )
    };
    let c = a + b;

    assert!(c.matches(value + lower));
    assert!(c.matches(value + upper));
    if value + lower <= candidate && candidate <= value + upper {
        assert!(c.matches(candidate));
    } else {
        assert!(!c.matches(candidate));
    }
}

#[test]
fn test_range_plus_exact() {
    let mut rng = TestRng::default();

    let value = u32::rand(&mut rng) as u64;
    let first_bound = u32::rand(&mut rng) as u64;
    let second_bound = u32::rand(&mut rng) as u64;
    let candidate = u32::rand(&mut rng) as u64;

    let (a, lower, upper) = if first_bound <= second_bound {
        (
            Measurement::Range(first_bound, second_bound),
            first_bound,
            second_bound,
        )
    } else {
        (
            Measurement::Range(second_bound, first_bound),
            second_bound,
            first_bound,
        )
    };
    let b = Measurement::Exact(value);
    let c = a + b;

    assert!(c.matches(value + lower));
    assert!(c.matches(value + upper));
    if value + lower <= candidate && candidate <= value + upper {
        assert!(c.matches(candidate));
    } else {
        assert!(!c.matches(candidate));
    }
}

#[test]
fn test_range_plus_range() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let third = u32::rand(&mut rng) as u64;
        let fourth = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let (a, first_lower, first_upper) = if first <= second {
            (Measurement::Range(first, second), first, second)
        } else {
            (Measurement::Range(second, first), second, first)
        };
        let (b, second_lower, second_upper) = if third <= fourth {
            (Measurement::Range(third, fourth), third, fourth)
        } else {
            (Measurement::Range(fourth, third), fourth, third)
        };
        let c = a + b;

        assert!(c.matches(first_lower + second_lower));
        assert!(c.matches(first_upper + second_upper));
        if first_lower + second_lower <= candidate && candidate <= first_upper + second_upper {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_range_plus_upper() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let third = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let (a, lower, upper) = if second <= third {
            (Measurement::Range(second, third), second, third)
        } else {
            (Measurement::Range(third, second), third, second)
        };
        let b = Measurement::UpperBound(first);
        let c = a + b;

        assert!(c.matches(lower));
        assert!(c.matches(first + upper));
        if lower <= candidate && candidate <= first + upper {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_upper_plus_exact() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let a = Measurement::UpperBound(second);
        let b = Measurement::Exact(first);
        let c = a + b;

        assert!(c.matches(first + second));
        if candidate <= first + second {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_upper_plus_range() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let third = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let a = Measurement::UpperBound(first);
        let (b, lower, upper) = if second <= third {
            (Measurement::Range(second, third), second, third)
        } else {
            (Measurement::Range(third, second), third, second)
        };
        let c = a + b;

        assert!(c.matches(lower));
        assert!(c.matches(first + upper));
        if lower <= candidate && candidate <= first + upper {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

#[test]
fn test_upper_plus_upper() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let first = u32::rand(&mut rng) as u64;
        let second = u32::rand(&mut rng) as u64;
        let candidate = u32::rand(&mut rng) as u64;

        let a = Measurement::UpperBound(second);
        let b = Measurement::UpperBound(first);
        let c = a + b;

        assert!(c.matches(first + second));
        if candidate <= first + second {
            assert!(c.matches(candidate));
        } else {
            assert!(!c.matches(candidate));
        }
    }
}

// Test multiplication.

#[test]
fn test_exact_mul() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let start = u32::rand(&mut rng) as u64;
        let scalar = u32::rand(&mut rng) as u64;

        let expected = Measurement::Exact(start * scalar);
        let candidate = Measurement::Exact(start) * scalar;
        assert_eq!(candidate, expected);
    }
}

#[test]
fn test_upper_bound_mul() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let start = u32::rand(&mut rng) as u64;
        let scalar = u32::rand(&mut rng) as u64;

        let expected = Measurement::UpperBound(start * scalar);
        let candidate = Measurement::UpperBound(start) * scalar;
        assert_eq!(candidate, expected);
    }
}

#[test]
fn test_range_mul() {
    let mut rng = TestRng::default();

    for _ in 0..ITERATIONS {
        let start = u32::rand(&mut rng) as u64;
        let end = u32::rand(&mut rng) as u64;
        let scalar = u32::rand(&mut rng) as u64;

        let expected = Measurement::Range(start * scalar, end * scalar);
        let candidate = Measurement::Range(start, end) * scalar;
        assert_eq!(candidate, expected);
    }
}
