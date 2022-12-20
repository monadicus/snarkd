/// The `witness!` macro is a closure that takes in a list of circuits,
/// eject the value of each circuit, and uses it in the subsequent code block.
///
/// This macro requires `Inject` to be implemented on the return type,
/// and `Eject` to be implemented on all inputs.
#[macro_export]
macro_rules! witness {
    (| $($circuit:ident),* | $block:block) => {{
        // Determine the witness mode, by checking if all given circuits are constant.
        let mode = $crate::witness_mode!($( $circuit ),*);

        use $crate::circuit::circuit::Circuit;
        Circuit::new_witness(mode, || {
            // Reassign each circuit to its primitive type.
            $( let $circuit = $circuit.eject_value(); )*
            // Execute the code block, returning the primitive to be injected.
            $block
        })
    }};
    (| $($circuit:ident),* | $logic:expr) => {{
        witness!(| $($circuit),* | { $logic })
    }};
}

/// The `witness_mode!` macro returns the expected mode given a list of circuits.
#[macro_export]
macro_rules! witness_mode {
    ($($circuit:ident),*) => {{
        // Determine the witness mode, by checking if all given circuits are constant.
        match $( $circuit.is_constant() & )* true {
            true => $crate::circuit::helpers::Mode::Constant,
            false => $crate::circuit::helpers::Mode::Private
        }
    }}
}

#[cfg(test)]
mod tests {
    use crate::circuit::helpers::Mode;
    use crate::circuit::traits::{Eject, Inject};
    use crate::circuit::Environment;

    /// A dummy struct for testing `witness!`.
    pub struct Foo(Mode, u8);

    impl Inject for Foo {
        type Primitive = u8;

        fn new(mode: Mode, value: Self::Primitive) -> Self {
            Self(mode, value)
        }
    }

    impl Eject for Foo {
        type Primitive = u8;

        fn eject_mode(&self) -> Mode {
            self.0
        }

        fn eject_value(&self) -> Self::Primitive {
            self.1
        }
    }

    fn check_witness(expected_mode: Mode, mode_a: Mode, mode_b: Mode) {
        let x = Foo::new(mode_a, 3);
        let y = Foo::new(mode_b, 5);
        let z: Foo = witness!(|x, y| { x + y });
        assert_eq!(expected_mode, z.eject_mode());
        assert_eq!(8, z.eject_value());
    }

    #[test]
    fn test_witness_constant() {
        check_witness(Mode::Constant, Mode::Constant, Mode::Constant);
    }

    #[test]
    fn test_witness_private() {
        check_witness(Mode::Private, Mode::Constant, Mode::Private);
    }
}
