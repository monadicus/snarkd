use std::fmt;

use ruint::uint;

use crate::{Fp, Index, Variable};

use super::*;

macro_rules! test_op {
    (
        $op:tt,
        $(
            [
                $name:expr,
                $lhs:expr,
                $rhs:expr,
                $unswapped_out:expr,
                $swapped_out:expr,
                $coeff:expr,
                $unswapped_coeff_out:expr,
                $swapped_coeff_out:expr
            ]
        ),+
    ) => {
        $(
            println!("{}: ", $name);
            let lhs = create_multi(&$lhs);
            let rhs = create_multi::<u64>(&$rhs);
            let unswapped_out = create_multi(&$unswapped_out);
            let swapped_out = create_multi(&$swapped_out);
            let unswapped_coeff_out = create_multi(&$unswapped_coeff_out);
            let swapped_coeff_out = create_multi(&$swapped_coeff_out);
            let str_op = stringify!($op);

            print!("    lhs {str_op} rhs:   ");
            test_op!(@op $op, lhs.clone(), rhs.clone(), unswapped_out);
            print!("    rhs {str_op} lhs:   ");
            test_op!(@op $op, rhs.clone(), lhs.clone(), swapped_out);

            print!("    &lhs {str_op} rhs:  ");
            test_op!(@op $op, &lhs, rhs.clone(), unswapped_out);
            print!("    &rhs {str_op} lhs:  ");
            test_op!(@op $op, &rhs, lhs.clone(), swapped_out);

            print!("    lhs {str_op} &rhs:  ");
            test_op!(@op $op, lhs.clone(), &rhs, unswapped_out);
            print!("    rhs {str_op} &lhs:  ");
            test_op!(@op $op, rhs.clone(), &lhs, swapped_out);

            print!("    &lhs {str_op} &rhs: ");
            test_op!(@op $op, &lhs, &rhs, unswapped_out);
            print!("    &rhs {str_op} &lhs: ");
            test_op!(@op $op, &rhs, &lhs, swapped_out);



            print!("    lhs {str_op} (coeff, rhs):   ");
            test_op!(@coeff $op, lhs.clone(), rhs.clone(), $coeff, unswapped_coeff_out);
            print!("    rhs {str_op} (coeff, lhs):   ");
            test_op!(@coeff $op, rhs.clone(), lhs.clone(), $coeff, swapped_coeff_out);

            print!("    &lhs {str_op} (coeff, rhs):  ");
            test_op!(@coeff $op, &lhs, rhs.clone(), $coeff, unswapped_coeff_out);
            print!("    &rhs {str_op} (coeff, lhs):  ");
            test_op!(@coeff $op, &rhs, lhs.clone(), $coeff, swapped_coeff_out);

            print!("    lhs {str_op} (coeff, &rhs):  ");
            test_op!(@coeff $op, lhs.clone(), &rhs, $coeff, unswapped_coeff_out);
            print!("    rhs {str_op} (coeff, &lhs):  ");
            test_op!(@coeff $op, rhs.clone(), &lhs, $coeff, swapped_coeff_out);

            print!("    &lhs {str_op} (coeff, &rhs): ");
            test_op!(@coeff $op, &lhs, &rhs, $coeff, unswapped_coeff_out);
            print!("    &rhs {str_op} (coeff, &lhs): ");
            test_op!(@coeff $op, &rhs, &lhs, $coeff, swapped_coeff_out);

            println!();
        )+
    };
    (@op $op:tt, $lhs:expr, $rhs:expr, $unswapped_out:expr) => {
        let res = $lhs $op $rhs;
        assert_eq!(res, $unswapped_out);
        println!("ok!");
    };
    (@coeff $op:tt, $lhs:expr, $rhs:expr, $coeff:expr, $unswapped_out:expr) => {
        // TODO flip argument order in coeff math to match everything else
        let res = $lhs $op ($coeff, $rhs);
        assert_eq!(res, $unswapped_out);
        println!("ok!");
    }
}

const FP_TWO: Fp = Fp(uint!(2_U384));

impl fmt::Display for LinearCombination<Fp> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for (i, fp) in self.0.iter() {
            writeln!(
                f,
                "({}, {})",
                match i.get_unchecked() {
                    Index::Public(i) => i,
                    Index::Private(i) => i,
                },
                fp.0
            )?;
        }
        writeln!(f, "]")
    }
}

fn empty_combo() -> LinearCombination<Fp> {
    LinearCombination::zero()
}

fn create_combo<V: Into<Fp>>(idx: usize, v: V) -> LinearCombination<Fp> {
    LinearCombination::new(vec![(
        Variable::new_unchecked(Index::Public(idx)),
        v.into(),
    )])
    .unwrap()
}

fn create_multi<V: Into<Fp> + Copy>(vals: &[(usize, V)]) -> LinearCombination<Fp> {
    LinearCombination::new(
        vals.iter()
            .copied()
            .map(|(idx, v)| (Variable::new_unchecked(Index::Public(idx)), v.into()))
            .collect(),
    )
    .unwrap()
}

// TODO test exact error
#[test]
fn duplicate_combos() {
    assert!(LinearCombination::new(vec![
        (Variable::new_unchecked(Index::Public(1)), Fp::from(1)),
        (Variable::new_unchecked(Index::Public(1)), Fp::from(67)),
    ])
    .is_err());
}

#[test]
fn replace_in_place() {
    let mut og = LinearCombination::<Fp>::zero();
    let other = create_combo(0, 1);
    og.replace_in_place(other.clone());
    assert_eq!(og, other);
}

#[test]
fn linear_combination_append() {
    let mut create = empty_combo();
    for i in 0..100u64 {
        create += (i.into(), Variable::new_unchecked(Index::Public(0)));
    }
    assert_eq!(create, create_combo(0, 0x1356));
}

#[test]
fn linear_combination_append_100() {
    let mut create = LinearCombination::<Fp>::zero();
    for i in 0..100u64 {
        create += (i.into(), Variable::new_unchecked(Index::Public(i as usize)));
    }
    for i in 0..100u64 {
        create += (i.into(), Variable::new_unchecked(Index::Public(i as usize)));
    }
    assert_eq!(create.0.len(), 100);
}

#[test]
fn stray_math() {
    let fp = FP_TWO;
    let var = Variable::new_unchecked(Index::Public(1));
    let combo = create_multi(&[(0, 1), (1, 5), (2, 0)]);

    // add (fp, var)
    let out = combo.clone() + (fp, var);
    assert_eq!(out, create_multi(&[(0, 1), (1, 7), (2, 0)]));
    // sub (fp, var)
    let out = combo.clone() - (fp, var);
    assert_eq!(out, create_multi(&[(0, 1), (1, 3), (2, 0)]));
    // mul Fp
    let out = combo.clone() * fp;
    assert_eq!(out, create_multi(&[(0, 2), (1, 10), (2, 0)]));
    // add var
    let out = combo.clone() + var;
    assert_eq!(out, create_multi(&[(0, 1), (1, 6), (2, 0)]));
    // sub var
    let out = combo - var;
    assert_eq!(out, create_multi(&[(0, 1), (1, 4), (2, 0)]));
}

#[test]
fn add_combo() {
    test_op!(+,
        [
            "Add single, same index, same values",
            [(0, 1)],
            [(0, 1)],
            [(0, 2)],
            [(0, 2)],
            FP_TWO,
            [(0, 3)],
            [(0, 3)]
        ],
        [
            "Add single, same index, different values",
            [(0, 1)],
            [(0, 2)],
            [(0, 3)],
            [(0, 3)],
            FP_TWO,
            [(0, 5)],
            [(0, 4)]
        ],
        [
            "Add single, same index, empty value",
            [(0, 1)],
            [],
            [(0, 1)],
            [(0, 1)],
            FP_TWO,
            [(0, 1)],
            [(0, 2)]
        ],
        [
            "Add single, different indexes, same values",
            [(0, 1)],
            [(1, 1)],
            [(0, 1), (1, 1)],
            [(0, 1), (1, 1)],
            FP_TWO,
            [(0, 1), (1, 2)],
            [(0, 2), (1, 1)]
        ],
        [
            "Add single, different indexes, different values",
            [(0, 1)],
            [(1, 2)],
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            FP_TWO,
            [(0, 1), (1, 4)],
            [(0, 2), (1, 2)]
        ],
        [
            "Add single, different indexes, empty value",
            [(1, 1)],
            [],
            [(1, 1)],
            [(1, 1)],
            FP_TWO,
            [(1, 1)],
            [(1, 2)]
        ],
        [
            "Add multi, same index, same values",
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            [(0, 2), (1, 4)],
            [(0, 2), (1, 4)],
            FP_TWO,
            [(0, 3), (1, 6)],
            [(0, 3), (1, 6)]
        ],
        [
            "Add multi, same index, different values",
            [(0, 1), (1, 2)],
            [(0, 2), (1, 1)],
            [(0, 3), (1, 3)],
            [(0, 3), (1, 3)],
            FP_TWO,
            [(0, 5), (1, 4)],
            [(0, 4), (1, 5)]
        ],
        [
            "Add multi, same index, empty value",
            [(0, 1), (1, 2)],
            [],
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            FP_TWO,
            [(0, 1), (1, 2)],
            [(0, 2), (1, 4)]
        ],
        [
            "Add multi, different indexes, same values",
            [(0, 1), (1, 1)],
            [(1, 1), (2, 1)],
            [(0, 1), (1, 2), (2, 1)],
            [(0, 1), (1, 2), (2, 1)],
            FP_TWO,
            [(0, 1), (1, 3), (2, 2)],
            [(0, 2), (1, 3), (2, 1)]
        ],
        [
            "Add multi, different indexes, different values",
            [(0, 1), (1, 3)],
            [(1, 2), (2, 4)],
            [(0, 1), (1, 5), (2, 4)],
            [(0, 1), (1, 5), (2, 4)],
            FP_TWO,
            [(0, 1), (1, 7), (2, 8)],
            [(0, 2), (1, 8), (2, 4)]
        ],
        [
            "Add multi, different indexes, empty value",
            [(1, 1), (2, 3)],
            [],
            [(1, 1), (2, 3)],
            [(1, 1), (2, 3)],
            FP_TWO,
            [(1, 1), (2, 3)],
            [(1, 2), (2, 6)]
        ]
    );
}

#[test]
fn sub_combo() {
    let underflowed_by_one = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000000_U384);
    let underflowed_by_two = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508bfffffffffff_U384);
    let underflowed_by_three = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508bffffffffffe_U384);
    let underflowed_by_four = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508bffffffffffd_U384);
    let zero = uint!(0_U384);
    let one = uint!(1_U384);
    let two = uint!(2_U384);
    test_op!(-,
        [
            "Sub single, same index, same values",
            [(0, 1)],
            [(0, 1)],
            [(0, 0)],
            [(0, 0)],
            FP_TWO,
            [(0, underflowed_by_one)],
            [(0, underflowed_by_one)]
        ],
        [
            "Sub single, same index, different values",
            [(0, 2)],
            [(0, 1)],
            [(0, 1)],
            [(0, underflowed_by_one)],
            FP_TWO,
            [(0, 0)],
            [(0, underflowed_by_three)]
        ],
        [
            "Sub single, same index, empty value",
            [(0, 1)],
            [],
            [(0, 1)],
            [(0, underflowed_by_one)],
            FP_TWO,
            [(0, 1)],
            [(0, underflowed_by_two)]
        ],
        [
            "Sub single, different indexes, same values",
            [(0, 1)],
            [(1, 1)],
            [(0, one), (1, underflowed_by_one)],
            [(0, underflowed_by_one), (1, one)],
            FP_TWO,
            [(0, one), (1, underflowed_by_two)],
            [(0, underflowed_by_two), (1, one)]
        ],
        [
            "Sub single, different indexes, different values",
            [(0, 1)],
            [(1, 2)],
            [(0, one), (1, underflowed_by_two)],
            [(0, underflowed_by_one), (1, two)],
            FP_TWO,
            [(0, one), (1, underflowed_by_four)],
            [(0, underflowed_by_two), (1, two)]
        ],
        [
            "Sub single, different indexes, empty value",
            [(1, 1)],
            [],
            [(1, 1)],
            [(1, underflowed_by_one)],
            FP_TWO,
            [(1, 1)],
            [(1, underflowed_by_two)]
        ],
        [
            "Sub multi, same index, same values",
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            [(0, 0), (1, 0)],
            [(0, 0), (1, 0)],
            FP_TWO,
            [(0, underflowed_by_one), (1, underflowed_by_two)],
            [(0, underflowed_by_one), (1, underflowed_by_two)]
        ],
        [
            "Sub multi, same index, different values",
            [(0, 2), (1, 2)],
            [(0, 1), (1, 1)],
            [(0, 1), (1, 1)],
            [(0, underflowed_by_one), (1, underflowed_by_one)],
            FP_TWO,
            [(0, 0), (1, 0)],
            [(0, underflowed_by_three), (1, underflowed_by_three)]
        ],
        [
            "Sub multi, same index, empty value",
            [(0, 1), (1, 2)],
            [],
            [(0, 1), (1, 2)],
            [(0, underflowed_by_one), (1, underflowed_by_two)],
            FP_TWO,
            [(0, 1), (1, 2)],
            [(0, underflowed_by_two), (1, underflowed_by_four)]
        ],
        [
            "Sub multi, different indexes, same values",
            [(0, 1), (1, 1)],
            [(1, 1), (2, 1)],
            [(0, one), (1, zero), (2, underflowed_by_one)],
            [(0, underflowed_by_one), (1, zero), (2, one)],
            FP_TWO,
            [(0, one), (1, underflowed_by_one), (2, underflowed_by_two)],
            [(0, underflowed_by_two), (1, underflowed_by_one), (2, one)]
        ],
        [
            "Sub multi, different indexes, different values",
            [(0, 1), (1, 3)],
            [(1, 2), (2, 2)],
            [(0, one), (1, one), (2, underflowed_by_two)],
            [(0, underflowed_by_one), (1, underflowed_by_one), (2, two)],
            FP_TWO,
            [(0, one), (1, underflowed_by_one), (2, underflowed_by_four)],
            [(0, underflowed_by_two), (1, underflowed_by_four), (2, two)]
        ],
        [
            "Sub multi, different indexes, empty value",
            [(1, 1), (2, 2)],
            [],
            [(1, 1), (2, 2)],
            [(1, underflowed_by_one), (2, underflowed_by_two)],
            FP_TWO,
            [(1, 1), (2, 2)],
            [(1, underflowed_by_two), (2, underflowed_by_four)]
        ]
    );
}
