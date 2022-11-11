#[macro_export]
macro_rules! scope {
    ($scope_name:expr, $block:block) => {
        E::scope($scope_name, || $block)
    };
}

#[macro_export]
macro_rules! print_scope {
    () => {{
        println!(
            "Circuit::scope(Constants: {:?}, Public: {:?}, Private: {:?}, Constraints: {:?})\n",
            Circuit::num_constants_in_scope(),
            Circuit::num_public_in_scope(),
            Circuit::num_private_in_scope(),
            Circuit::num_constraints_in_scope()
        );
    }};
}

#[macro_export]
macro_rules! assert_scope {
    () => {{
        $crate::print_scope!();

        assert!(Circuit::is_satisfied_in_scope(), "(is_satisfied_in_scope)");
    }};
    ($num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr) => {{
        $crate::print_scope!();

        assert_eq!(
            $num_constants,
            Circuit::num_constants_in_scope(),
            "(num_constants)"
        );
        assert_eq!($num_public, Circuit::num_public_in_scope(), "(num_public)");
        assert_eq!(
            $num_private,
            Circuit::num_private_in_scope(),
            "(num_private)"
        );
        assert_eq!(
            $num_constraints,
            Circuit::num_constraints_in_scope(),
            "(num_constraints)"
        );
        assert!(Circuit::is_satisfied_in_scope(), "(is_satisfied_in_scope)");
    }};
    (<=$num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr) => {{
        $crate::print_scope!();

        assert!(
            Circuit::num_constants_in_scope() <= $num_constants,
            "(num_constants)"
        );
        assert_eq!($num_public, Circuit::num_public_in_scope(), "(num_public)");
        assert_eq!(
            $num_private,
            Circuit::num_private_in_scope(),
            "(num_private)"
        );
        assert_eq!(
            $num_constraints,
            Circuit::num_constraints_in_scope(),
            "(num_constraints)"
        );
        assert!(Circuit::is_satisfied_in_scope(), "(is_satisfied_in_scope)");
    }};
    (<=$num_constants:expr, <=$num_public:expr, <=$num_private:expr, <=$num_constraints:expr) => {{
        $crate::print_scope!();

        assert!(
            Circuit::num_constants_in_scope() <= $num_constants,
            "(num_constants)"
        );
        assert!(
            Circuit::num_public_in_scope() <= $num_public,
            "(num_public)"
        );
        assert!(
            Circuit::num_private_in_scope() <= $num_private,
            "(num_private)"
        );
        assert!(
            Circuit::num_constraints_in_scope() <= $num_constraints,
            "(num_constraints)"
        );
        assert!(Circuit::is_satisfied_in_scope(), "(is_satisfied_in_scope)");
    }};
    ($case:expr, $num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr) => {{
        $crate::print_scope!();

        assert_eq!(
            $num_constants,
            Circuit::num_constants_in_scope(),
            "{} (num_constants)",
            $case
        );
        assert_eq!(
            $num_public,
            Circuit::num_public_in_scope(),
            "{} (num_public)",
            $case
        );
        assert_eq!(
            $num_private,
            Circuit::num_private_in_scope(),
            "{} (num_private)",
            $case
        );
        assert_eq!(
            $num_constraints,
            Circuit::num_constraints_in_scope(),
            "{} (num_constraints)",
            $case
        );
        assert!(
            Circuit::is_satisfied_in_scope(),
            "{} (is_satisfied_in_scope)",
            $case
        );
    }};
}

#[macro_export]
macro_rules! assert_scope_fails {
    () => {{
        $crate::print_scope!();

        assert!(
            !Circuit::is_satisfied_in_scope(),
            "(!is_satisfied_in_scope)"
        );
    }};
    ($num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr) => {{
        $crate::print_scope!();

        assert_eq!(
            $num_constants,
            Circuit::num_constants_in_scope(),
            "(num_constants)"
        );
        assert_eq!($num_public, Circuit::num_public_in_scope(), "(num_public)");
        assert_eq!(
            $num_private,
            Circuit::num_private_in_scope(),
            "(num_private)"
        );
        assert_eq!(
            $num_constraints,
            Circuit::num_constraints_in_scope(),
            "(num_constraints)"
        );
        assert!(
            !Circuit::is_satisfied_in_scope(),
            "(!is_satisfied_in_scope)"
        );
    }};
    ($case:expr, $num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr) => {{
        $crate::print_scope!();

        assert_eq!(
            $num_constants,
            Circuit::num_constants_in_scope(),
            "{} (num_constants)",
            $case
        );
        assert_eq!(
            $num_public,
            Circuit::num_public_in_scope(),
            "{} (num_public)",
            $case
        );
        assert_eq!(
            $num_private,
            Circuit::num_private_in_scope(),
            "{} (num_private)",
            $case
        );
        assert_eq!(
            $num_constraints,
            Circuit::num_constraints_in_scope(),
            "{} (num_constraints)",
            $case
        );
        assert!(
            !Circuit::is_satisfied_in_scope(),
            "{} (!is_satisfied_in_scope)",
            $case
        );
    }};
}
