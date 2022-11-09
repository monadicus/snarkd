use std::fmt;

use ruint::uint;

use crate::{Fp, Index, Variable};

use super::*;

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

macro_rules! test_op {
    (
        $op:tt,
        $(
            [
                $name:expr,
                $lhs:expr,
                $rhs:expr,
                $out:expr,
                $flipped_out:expr
            ]
        ),+
    ) => {
        $(
            println!("{}: ", $name);
            let lhs = create_multi(&$lhs);
            let rhs = create_multi::<u64>(&$rhs);
            let out = create_multi(&$out);
            let flipped_out = create_multi(&$flipped_out);
            let str_op = stringify!($op);

            print!("    lhs {str_op} rhs:   ");
            test_op!(@test $op, lhs.clone(), rhs.clone(), out);
            print!("    rhs {str_op} lhs:   ");
            test_op!(@test $op, rhs.clone(), lhs.clone(), flipped_out);

            print!("    &lhs {str_op} rhs:  ");
            test_op!(@test $op, &lhs, rhs.clone(), out);
            print!("    &rhs {str_op} lhs:  ");
            test_op!(@test $op, &rhs, lhs.clone(), flipped_out);

            print!("    lhs {str_op} &rhs:  ");
            test_op!(@test $op, lhs.clone(), &rhs, out);
            print!("    rhs {str_op} &lhs:  ");
            test_op!(@test $op, rhs.clone(), &lhs, flipped_out);

            print!("    &lhs {str_op} &rhs: ");
            test_op!(@test $op, &lhs, &rhs, out);
            print!("    &rhs {str_op} &lhs: ");
            test_op!(@test $op, &rhs, &lhs, flipped_out);

            println!();
        )+
    };
    (@test $op:tt, $lhs:expr, $rhs:expr, $out:expr) => {
        let res = $lhs $op $rhs;
        assert_eq!(res, $out);
        println!("ok!");
    };
}

#[test]
fn add_combo() {
    test_op!(+,
        [
            "Add single, same index, same values",
            [(0, 1)],
            [(0, 1)],
            [(0, 2)],
            [(0, 2)]
        ],
        [
            "Add single, same index, different values",
            [(0, 1)],
            [(0, 2)],
            [(0, 3)],
            [(0, 3)]
        ],
        [
            "Add single, same index, empty value",
            [(0, 1)],
            [],
            [(0, 1)],
            [(0, 1)]
        ],
        [
            "Add single, different indexes, same values",
            [(0, 1)],
            [(1, 1)],
            [(0, 1), (1, 1)],
            [(0, 1), (1, 1)]
        ],
        [
            "Add single, different indexes, different values",
            [(0, 1)],
            [(1, 2)],
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)]
        ],
        [
            "Add single, different indexes, empty value",
            [(1, 1)],
            [],
            [(1, 1)],
            [(1, 1)]
        ],
        [
            "Add multi, same index, same values",
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            [(0, 2), (1, 4)],
            [(0, 2), (1, 4)]
        ],
        [
            "Add multi, same index, different values",
            [(0, 1), (1, 2)],
            [(0, 2), (1, 1)],
            [(0, 3), (1, 3)],
            [(0, 3), (1, 3)]
        ],
        [
            "Add multi, same index, empty value",
            [(0, 1), (1, 2)],
            [],
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)]
        ],
        [
            "Add multi, different indexes, same values",
            [(0, 1), (1, 1)],
            [(1, 1), (2, 1)],
            [(0, 1), (1, 2), (2, 1)],
            [(0, 1), (1, 2), (2, 1)]
        ],
        [
            "Add multi, different indexes, different values",
            [(0, 1), (1, 3)],
            [(1, 2), (2, 4)],
            [(0, 1), (1, 5), (2, 4)],
            [(0, 1), (1, 5), (2, 4)]
        ],
        [
            "Add multi, different indexes, empty value",
            [(1, 1), (2, 3)],
            [],
            [(1, 1), (2, 3)],
            [(1, 1), (2, 3)]
        ]
    );
}

#[test]
fn sub_combo() {
    let underflowed_by_one = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000000_U384);
    let underflowed_by_two = uint!(0x01ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508bfffffffffff_U384);
    let zero = uint!(0_U384);
    let one = uint!(1_U384);
    let two = uint!(2_U384);
    test_op!(-,
        [
            "Sub single, same index, same values",
            [(0, 1)],
            [(0, 1)],
            [(0, 0)],
            [(0, 0)]
        ],
        [
            "Sub single, same index, different values",
            [(0, 2)],
            [(0, 1)],
            [(0, 1)],
            [(0, underflowed_by_one)]
        ],
        [
            "Sub single, same index, empty value",
            [(0, 1)],
            [],
            [(0, 1)],
            [(0, underflowed_by_one)]
        ],
        [
            "Sub single, different indexes, same values",
            [(0, 1)],
            [(1, 1)],
            [(0, one), (1, underflowed_by_one)],
            [(0, underflowed_by_one), (1, one)]
        ],
        [
            "Sub single, different indexes, different values",
            [(0, 1)],
            [(1, 2)],
            [(0, one), (1, underflowed_by_two)],
            [(0, underflowed_by_one), (1, two)]
        ],
        [
            "Sub single, different indexes, empty value",
            [(1, 1)],
            [],
            [(1, 1)],
            [(1, underflowed_by_one)]
        ],
        [
            "Sub multi, same index, same values",
            [(0, 1), (1, 2)],
            [(0, 1), (1, 2)],
            [(0, 0), (1, 0)],
            [(0, 0), (1, 0)]
        ],
        [
            "Sub multi, same index, different values",
            [(0, 2), (1, 2)],
            [(0, 1), (1, 1)],
            [(0, 1), (1, 1)],
            [(0, underflowed_by_one), (1, underflowed_by_one)]
        ],
        [
            "Sub multi, same index, empty value",
            [(0, 1), (1, 2)],
            [],
            [(0, 1), (1, 2)],
            [(0, underflowed_by_one), (1, underflowed_by_two)]
        ],
        [
            "Sub multi, different indexes, same values",
            [(0, 1), (1, 1)],
            [(1, 1), (2, 1)],
            [(0, one), (1, zero), (2, underflowed_by_one)],
            [(0, underflowed_by_one), (1, zero), (2, one)]
        ],
        [
            "Sub multi, different indexes, different values",
            [(0, 1), (1, 3)],
            [(1, 2), (2, 2)],
            [(0, one), (1, one), (2, underflowed_by_two)],
            [(0, underflowed_by_one), (1, underflowed_by_one), (2, two)]
        ],
        [
            "Sub multi, different indexes, empty value",
            [(1, 1), (2, 2)],
            [],
            [(1, 1), (2, 2)],
            [(1, underflowed_by_one), (2, underflowed_by_two)]
        ]
    );
}
