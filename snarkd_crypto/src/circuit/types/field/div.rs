// use super::*;

// impl<E: Environment> Div<Field<E>> for Field<E> {
//     type Output = Field<E>;

//     fn div(self, other: Field<E>) -> Self::Output {
//         self / &other
//     }
// }

// impl<E: Environment> Div<&Field<E>> for Field<E> {
//     type Output = Field<E>;

//     fn div(self, other: &Field<E>) -> Self::Output {
//         &self / other
//     }
// }

// impl<E: Environment> Div<Field<E>> for &Field<E> {
//     type Output = Field<E>;

//     fn div(self, other: Field<E>) -> Self::Output {
//         self / &other
//     }
// }

// impl<E: Environment> Div<&Field<E>> for &Field<E> {
//     type Output = Field<E>;

//     fn div(self, other: &Field<E>) -> Self::Output {
//         let mut output = self.clone();
//         output /= other;
//         output
//     }
// }

// impl<E: Environment> DivAssign<Self> for Field<E> {
//     fn div_assign(&mut self, other: Self) {
//         *self /= &other;
//     }
// }

// impl<E: Environment> DivAssign<&Self> for Field<E> {
//     #[allow(clippy::suspicious_op_assign_impl)]
//     fn div_assign(&mut self, other: &Self) {
//         match other.is_constant() {
//             // If `other` is a constant and zero, halt since the inverse of zero is undefined.
//             true if other.eject_value().is_zero() => E::halt("Attempted to divide by zero."),
//             // If `other` is a constant and non-zero, we can perform multiplication and inversion for 0 constraints.
//             // If `self` is a constant, we can perform multiplication and inversion for 1 constraint.
//             // Otherwise, we can perform multiplication and inversion for 2 constraints.
//             _ => *self *= other.inverse(),
//         }
//     }
// }

// impl<E: Environment> Metrics<dyn Div<Field<E>, Output = Field<E>>> for Field<E> {
//     type Case = (Mode, Mode);

//     fn count(case: &Self::Case) -> Count {
//         match case {
//             (Mode::Constant, Mode::Constant) | (_, Mode::Constant) => Count::is(1, 0, 0, 0),
//             (Mode::Constant, _) => Count::is(0, 0, 1, 1),
//             (_, _) => Count::is(0, 0, 2, 2),
//         }
//     }
// }

// impl<E: Environment> OutputMode<dyn Div<Field<E>, Output = Field<E>>> for Field<E> {
//     type Case = (CircuitType<Field<E>>, CircuitType<Field<E>>);

//     fn output_mode(case: &Self::Case) -> Mode {
//         match (case.0.mode(), case.1.mode()) {
//             (Mode::Constant, Mode::Constant) => Mode::Constant,
//             (Mode::Public, Mode::Constant) => match &case.1 {
//                 CircuitType::Constant(constant) => match constant.eject_value().is_one() {
//                     true => Mode::Public,
//                     false => Mode::Private,
//                 },
//                 _ => E::halt("The constant is required to determine the output mode of Public + Constant"),
//             },
//             (_, _) => Mode::Private,
//         }
//     }
// }
