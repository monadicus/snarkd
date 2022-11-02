use crate::bls12_377::{group::Group, Fq, Fr};
use ruint::uint;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct G1Parameters;

impl Group for G1Parameters {
    type BaseField = Fq;

    /// COFACTOR = (x - 1)^2 / 3  = 30631250834960419227450344600217059328
    const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    ///              = 5285428838741532253824584287042945485047145357130994810877
    const COFACTOR_INV: Fr =
        Fr(uint!(5285428838741532253824584287042945485047145357130994810877_U256));

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    /// WEIERSTRASS_A = 0
    const A: Fq = Fq(uint!(0_U384));

    /// WEIERSTRASS_B = 1
    const B: Fq = Fq(uint!(1_U384));
}

///
/// G1_GENERATOR_X =
/// 89363714989903307245735717098563574705733591463163614225748337416674727625843187853442697973404985688481508350822
///
/// See `snarkvm_algorithms::hash_to_curve::tests::bls12_377` for tests.
///
const G1_GENERATOR_X: Fq = Fq(
    uint!(89363714989903307245735717098563574705733591463163614225748337416674727625843187853442697973404985688481508350822_U384),
);

///
/// G1_GENERATOR_Y =
/// 3702177272937190650578065972808860481433820514072818216637796320125658674906330993856598323293086021583822603349
///
/// See `snarkvm_algorithms::hash_to_curve::tests::bls12_377` for tests.
///
const G1_GENERATOR_Y: Fq = Fq(
    uint!(3702177272937190650578065972808860481433820514072818216637796320125658674906330993856598323293086021583822603349_U384),
);

#[cfg(test)]
mod tests {
    use super::{super::G1Affine, *};
    use crate::bls12_377::field::Field;
    use rand::Rng;

    #[test]
    fn test_subgroup_membership() {
        let rng = &mut TestRng::default();

        for _ in 0..1000 {
            let p = G1Affine::rand(rng);
            assert!(Bls12_377G1Parameters::is_in_correct_subgroup_assuming_on_curve(&p));
            let x = Fq::rand(rng);
            let greatest = rng.gen();

            if let Some(p) = G1Affine::from_x_coordinate(x, greatest) {
                assert_eq!(
                    Bls12_377G1Parameters::is_in_correct_subgroup_assuming_on_curve(&p),
                    p.mul_bits(Fr::characteristic().to_be_bytes().into())
                        .is_zero(),
                );
            }
        }
    }
}
