use super::{adc, field::Field, mac_with_carry, LegendreSymbol};
use bitvec::prelude::*;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use rand::{distributions::Standard, Rng};
use ruint::{uint, Uint};

/// BLS12-377 scalar field.
///
/// Roots of unity computed from modulus and R using this sage code:
///
/// ```ignore
/// q = 8444461749428370424248824938781546531375899335154063827935233455917409239041
/// R = 6014086494747379908336260804527802945383293308637734276299549080986809532403 # Montgomery R
/// s = 47
/// o = q - 1
/// F = GF(q)
/// g = F.multiplicative_generator()
/// assert g.multiplicative_order() == o
/// g2 = g ** (o/2**s)
/// assert g2.multiplicative_order() == 2**s
/// def into_chunks(val, width, n):
///     return [int(int(val) // (2 ** (width * i)) % 2 ** width) for i in range(n)]
/// print("Gen (g % q): ", g % q)
/// print("Gen (g * R % q): ", g * R % q)
/// print("Gen into_chunks(g * R % q): ", into_chunks(g * R % q, 64, 4))
/// print("2-adic gen (g2 % q): ", g2 % q)
/// print("2-adic gen (g2 * R % q): ", g2 * R % q)
/// print("2-adic gen into_chunks(g2 * R % q): ", into_chunks(g2 * R % q, 64, 4))
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fr(pub Uint<256, 4>);

pub const POWERS_OF_G: &[Uint<256, 4>] = &[
    uint!(7550553103602024334975125493733701741804725558747959317959731134235635147227_U256),
    uint!(2426736712223096716690040454781163745329121217182105961450780095846454806933_U256),
    uint!(7262145565060860359900199261463709015779508115142628947528384917326026416094_U256),
    uint!(3808078068525049614289740472596782760038232742112406467092184053649710314798_U256),
    uint!(8053308102911176505682290454775091362575770372752331498701463662290537247815_U256),
    uint!(6273457544692162003831119556805408422297842111808582875258929546672488264429_U256),
    uint!(1908878175455172417031534663965917898181912410075901692870771313709266634658_U256),
    uint!(6430007373874177799050006496675130336369688930425688114015025566689865783171_U256),
    uint!(4565923169794395794680701488261091745754138435524711720501461627715939569405_U256),
    uint!(4698088216716092350663678034822769545746381118735185064794011634701275473477_U256),
    uint!(7107639955054323924025836477581362910123531676424209412732228091406821360591_U256),
    uint!(1143428809486502174649162785588631220233506617635138101676201306883580545245_U256),
    uint!(6661969736299690646099364120787132718931790259671978454726540531639695530804_U256),
    uint!(7494358908018005753699050867867668371079138029241646165484594969182325347974_U256),
    uint!(8023592618106794736850331059027580398177891647397700383265888807327308790994_U256),
    uint!(8167064777010489646866383058955351019363599383041041402218270315933969755456_U256),
    uint!(4968108306635867391544864815397174769014949107081063320473730990256347118931_U256),
    uint!(1583656744794762729084637539897793117917025279046896672099767742640704123169_U256),
    uint!(5895420438657019419626895090640424535157489853263080541827545912299729235333_U256),
    uint!(4740075541253033224589998562420349890539926961488613459621272966886000413066_U256),
    uint!(4372361391637922170115686270508018481727789274993041266190133512464261921507_U256),
    uint!(1929325023208003076100474221444481912739650477822489523997933708713799327071_U256),
    uint!(1277888161114191603199244350778986273492648464488507419439435896557304905444_U256),
    uint!(1108499043736799866403548949193002894217225067957145040192483460774398762483_U256),
    uint!(395836617825562047058426267249964053383069529297507125119242048124656033814_U256),
    uint!(5860443082112728571901716126618652635484760588499751147533455870417568840877_U256),
    uint!(4394477575340825254208930529903097416640033318929164913323083797337718515184_U256),
    uint!(6655010501445240092776324835106764489701610576181395798460725469229712872036_U256),
    uint!(143833216181295269350350912428884844068377551309017947960250191659665387652_U256),
    uint!(1033876875038508851968235037009578130767757090696985046145426344987751233820_U256),
    uint!(8202182930899331103167914916987210217351711145196802749080031724322811626261_U256),
    uint!(6118387923823108256753882845234939760861325266895736900343175505842688216270_U256),
    uint!(7715000358433627285717624185900955233739272726851557525530631600959054049085_U256),
    uint!(7037068887891614228326211307922481546553657918094578974026471721554702847480_U256),
    uint!(1900239651279327116489512710474311941666086647150787076325151660569921142946_U256),
    uint!(1433607442963431421316013573046465495563131275603848750869846912581024913019_U256),
    uint!(866985245235040709753013773884295437521109261361027824514873427502964982922_U256),
    uint!(4443108021371737602697596971902852132274961263160868774409420424726835935922_U256),
    uint!(7230648057878268440511611882767992485465416891455657199765901381639068805022_U256),
    uint!(3007047394428189914303468568563069309375399555829781313999234300434804999455_U256),
    uint!(2252774523953722881297536424069691740647177044028921592024459201778483644059_U256),
    uint!(6821963945538064897924581611421472144776078914389058187104145641528714456127_U256),
    uint!(2044129581458360471194413030819022067476444176988436074573508567144719427532_U256),
    uint!(6638943125734816116533668954109782743742964348031428195244516495664415882670_U256),
    uint!(3279917132858342911831074864712036382710139745724269329239664300762234227201_U256),
    uint!(880904806456922042258150504921383618666682042621506879489_U256),
];

pub const TWO_ADICITY: u32 = 47;

/// TWO_ADIC_ROOT_OF_UNITY = 8065159656716812877374967518403273466521432693661810619979959746626482506078
/// Encoded in Montgomery form, the value is
/// (8065159656716812877374967518403273466521432693661810619979959746626482506078 * R % q) =
/// 7039866554349711480672062101017509031917008525101396696252683426045173093960
pub const TWO_ADIC_ROOT_OF_UNITY: Uint<256, 4> =
    uint!(8065159656716812877374967518403273466521432693661810619979959746626482506078_U256);

pub const CAPACITY: u32 = MODULUS_BITS - 1;

/// GENERATOR = 22
/// Encoded in Montgomery form, so the value is
/// (22 * R) % q = 5642976643016801619665363617888466827793962762719196659561577942948671127251
pub const GENERATOR: Uint<256, 4> =
    uint!(5642976643016801619665363617888466827793962762719196659561577942948671127251_U256);

pub const INV: u64 = 725501752471715839u64;

/// MODULUS = 8444461749428370424248824938781546531375899335154063827935233455917409239041
pub const MODULUS: Uint<256, 4> =
    uint!(8444461749428370424248824938781546531375899335154063827935233455917409239041_U256);

pub const MODULUS_BITS: u32 = 253;

/// (r - 1)/2 =
/// 4222230874714185212124412469390773265687949667577031913967616727958704619520
pub const MODULUS_MINUS_ONE_DIV_TWO: Uint<256, 4> =
    uint!(4222230874714185212124412469390773265687949667577031913967616727958704619520_U256);

pub const R: Uint<256, 4> =
    uint!(6014086494747379908336260804527802945383293308637734276299549080986809532403_U256);

pub const R2: Uint<256, 4> =
    uint!(508595941311779472113692600146818027278633330499214071737745792929336755579_U256);

pub const REPR_SHAVE_BITS: u32 = 3;

/// t = (r - 1) / 2^s =
/// 60001509534603559531609739528203892656505753216962260608619555
pub const T: Uint<256, 4> =
    uint!(60001509534603559531609739528203892656505753216962260608619555_U256);

/// (t - 1) / 2 =
/// 30000754767301779765804869764101946328252876608481130304309777
pub const T_MINUS_ONE_DIV_TWO: Uint<256, 4> =
    uint!(30000754767301779765804869764101946328252876608481130304309777_U256);

impl Fr {
    pub fn legendre(&self) -> LegendreSymbol {
        // s = self^((MODULUS - 1) // 2)
        let mut s = self.pow(MODULUS_MINUS_ONE_DIV_TWO.as_limbs());
        s.reduce();

        if s.is_zero() {
            panic!("unhit");
            LegendreSymbol::Zero
        } else if s.is_one() {
            LegendreSymbol::QuadraticResidue
        } else {
            LegendreSymbol::QuadraticNonResidue
        }
    }

    pub fn decompose(
        &self,
        q1: &[u64; 4],
        q2: &[u64; 4],
        b1: Self,
        b2: Self,
        r128: Self,
        half_r: &[u64; 8],
    ) -> (Self, Self, bool, bool) {
        let mul_short = |a: &[u64; 4], b: &[u64; 4]| -> [u64; 8] {
            // Schoolbook multiplication
            let mut carry = 0;
            let r0 = mac_with_carry(0, a[0], b[0], &mut carry);
            let r1 = mac_with_carry(0, a[0], b[1], &mut carry);
            let r2 = mac_with_carry(0, a[0], b[2], &mut carry);
            let r3 = carry;

            let mut carry = 0;
            let r1 = mac_with_carry(r1, a[1], b[0], &mut carry);
            let r2 = mac_with_carry(r2, a[1], b[1], &mut carry);
            let r3 = mac_with_carry(r3, a[1], b[2], &mut carry);
            let r4 = carry;

            let mut carry = 0;
            let r2 = mac_with_carry(r2, a[2], b[0], &mut carry);
            let r3 = mac_with_carry(r3, a[2], b[1], &mut carry);
            let r4 = mac_with_carry(r4, a[2], b[2], &mut carry);
            let r5 = carry;

            let mut carry = 0;
            let r3 = mac_with_carry(r3, a[3], b[0], &mut carry);
            let r4 = mac_with_carry(r4, a[3], b[1], &mut carry);
            let r5 = mac_with_carry(r5, a[3], b[2], &mut carry);
            let r6 = carry;

            [r0, r1, r2, r3, r4, r5, r6, 0]
        };

        let round = |a: &mut [u64; 8]| -> Self {
            let mut carry = 0;
            // NOTE: can the first 4 be omitted?
            carry = adc(&mut a[0], half_r[0], carry);
            carry = adc(&mut a[1], half_r[1], carry);
            carry = adc(&mut a[2], half_r[2], carry);
            carry = adc(&mut a[3], half_r[3], carry);
            carry = adc(&mut a[4], half_r[4], carry);
            carry = adc(&mut a[5], half_r[5], carry);
            carry = adc(&mut a[6], half_r[6], carry);
            _ = adc(&mut a[7], half_r[7], carry);
            Self(Uint::from_limbs([a[4], a[5], a[6], a[7]]))
        };

        let alpha = |x: &Self, q: &[u64; 4]| -> Self {
            let mut a = mul_short(x.0.as_limbs(), q);
            round(&mut a)
        };

        let alpha1 = alpha(self, q1);
        let alpha2 = alpha(self, q2);
        let z1 = alpha1 * b1;
        let z2 = alpha2 * b2;

        let mut k1 = *self - z1 - alpha2;
        let mut k2 = z2 - alpha1;
        let mut k1_neg = false;
        let mut k2_neg = false;

        if k1 > r128 {
            k1 = -k1;
            k1_neg = true;
        }

        if k2 > r128 {
            k2 = -k2;
            k2_neg = true;
        }

        (k1, k2, k1_neg, k2_neg)
    }

    fn reduce(&mut self) {
        while self.0 >= MODULUS {
            self.0 -= MODULUS;
        }
    }
}

impl Field for Fr {
    // We don't need GLV endomorphisms for the scalar field.
    const PHI: Self = Self(uint!(0_U256));

    fn zero() -> Self {
        Self(uint!(0_U256))
    }

    fn is_zero(&self) -> bool {
        self.0 == Self::zero().0
    }

    fn one() -> Self {
        Self(uint!(1_U256))
    }

    fn is_one(&self) -> bool {
        self.0 == Self::one().0
    }

    fn rand() -> Self {
        let mut a = Self(rand::thread_rng().sample(Standard));
        a.reduce();
        a
    }

    fn characteristic() -> Self {
        Self(MODULUS)
    }

    fn double(&self) -> Self {
        let mut res = *self;
        res.double_in_place();
        res
    }

    fn double_in_place(&mut self) {
        *self = Self(self.0.add_mod(self.0, MODULUS));
    }

    fn square(&self) -> Self {
        let mut res = *self;
        res.square_in_place();
        res
    }

    fn square_in_place(&mut self) {
        *self = Self(self.0.mul_mod(self.0, MODULUS));
    }

    fn inverse(&self) -> Option<Self> {
        self.0.inv_mod(MODULUS).map(Self)
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inv) = self.0.inv_mod(MODULUS) {
            panic!("unhit");
            *self = Self(inv);
            Some(self)
        } else {
            panic!("unhit");
            None
        }
    }

    fn sqrt(&self) -> Option<Self> {
        // https://eprint.iacr.org/2020/1407.pdf (page 4, algorithm 1)
        match self.legendre() {
            LegendreSymbol::Zero => panic!("unhit"), // Some(*self),
            LegendreSymbol::QuadraticNonResidue => None,
            LegendreSymbol::QuadraticResidue => {
                let n = TWO_ADICITY as u64;
                // `T` is equivalent to `m` in the paper.
                let v = self.pow(T_MINUS_ONE_DIV_TWO.as_limbs());
                let x = *self * v.square();

                let k = ((n - 1) as f64).sqrt().floor() as u64;
                // It's important that k_2 results in a number which makes `l_minus_one_times_k`
                // divisible by `k`, because the native arithmetic will not match the field
                // arithmetic otherwise (native numbers will divide and round down, but field
                // elements will end up nowhere near the native number).
                let k_2 = if n % 2 == 0 { k / 2 } else { (n - 1) % k };
                let k_1 = k - k_2;
                let l_minus_one_times_k = n - 1 - k_2;
                let l_minus_one = l_minus_one_times_k / k;
                let l = l_minus_one + 1;
                let mut l_s: Vec<u64> = Vec::with_capacity(k as usize);
                l_s.resize(l_s.len() + k_1 as usize, l_minus_one);
                l_s.resize(l_s.len() + k_2 as usize, l);

                let mut x_s: Vec<Self> = Vec::with_capacity(k as usize);
                let mut l_sum = 0;
                l_s.iter().take((k as usize) - 1).for_each(|l| {
                    l_sum += l;
                    let x = x.pow(&[2u64.pow((n - 1 - l_sum) as u32)]);
                    x_s.push(x);
                });
                x_s.push(x);

                let find = |delta: Self| -> u64 {
                    let mut mu = delta;
                    let mut i = 0;
                    while mu != -Self::one() {
                        mu.square_in_place();
                        i += 1;
                    }
                    i
                };

                let eval = |mut delta: Self| -> u64 {
                    let mut s = 0u64;
                    while delta != Self::one() {
                        let i = find(delta);
                        let n_minus_one_minus_i = n - 1 - i;
                        s += 2u64.pow(n_minus_one_minus_i as u32);
                        if i > 0 {
                            delta *= Self(POWERS_OF_G[n_minus_one_minus_i as usize])
                        } else {
                            delta = -delta;
                        }
                    }
                    s
                };

                let calc_kappa = |i: usize, j: usize, l_s: &[u64]| -> u64 {
                    l_s.iter().take(j).sum::<u64>() + 1 + l_s.iter().skip(i + 1).sum::<u64>()
                };

                let calc_gamma =
                    |i: usize, q_s: &Vec<BitVec>, last: bool| -> Self {
                        let mut gamma = Self::one();
                        if i != 0 {
                            q_s.iter()
                                .zip(l_s.iter())
                                .enumerate()
                                .for_each(|(j, (q_bits, l))| {
                                    let mut kappa = calc_kappa(i, j, &l_s);
                                    if last {
                                        kappa -= 1;
                                    }
                                    q_bits.iter().enumerate().take(*l as usize).for_each(
                                        |(k, bit)| {
                                            if *bit {
                                                gamma *= Self(POWERS_OF_G[(kappa as usize) + k])
                                            }
                                        },
                                    );
                                });
                        }
                        gamma
                    };

                let mut q_s = Vec::<BitVec>::with_capacity(k as usize);
                let two_to_n_minus_l = 2u64.pow((n - l) as u32);
                let two_to_n_minus_l_minus_one = 2u64.pow((n - l_minus_one) as u32);
                x_s.iter().enumerate().for_each(|(i, x)| {
                    // Calculate g^t.
                    // This algorithm deviates from the standard description in the paper, and is
                    // explained in detail in page 6, in section 2.1.
                    let gamma = calc_gamma(i, &q_s, false);
                    let alpha = *x * gamma;
                    q_s.push(BitVec::from_bitslice(
                        ((eval(alpha)
                            / if i < k_1 as usize {
                                two_to_n_minus_l_minus_one
                            } else {
                                two_to_n_minus_l
                            }) as usize)
                            .view_bits::<Lsb0>(),
                    ));
                });

                // Calculate g^{t/2}.
                let gamma = calc_gamma(k as usize, &q_s, true);
                Some(*self * v * gamma)
            }
        }
    }

    fn frobenius_map(&mut self, _: usize) {
        panic!("unhit");
        // No-op
    }

    fn glv_endomorphism(&self) -> Self {
        panic!("unhit");
        Self::zero()
    }
}

impl Add for Fr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add_mod(other.0, MODULUS))
    }
}

impl AddAssign for Fr {
    fn add_assign(&mut self, other: Self) {
        panic!("unhit");
        *self = *self + other
    }
}

impl Sub for Fr {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if other.0 > self.0 {
            self.0 += MODULUS;
        }
        Self(self.0 - other.0)
    }
}

impl SubAssign for Fr {
    fn sub_assign(&mut self, other: Self) {
        panic!("unhit");
        *self = *self - other;
    }
}

impl Mul for Fr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0.mul_mod(other.0, MODULUS))
    }
}

impl MulAssign for Fr {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Neg for Fr {
    type Output = Self;

    fn neg(self) -> Self {
        let mut a = Self(MODULUS - self.0);
        a.reduce();
        a
    }
}

impl Div for Fr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        panic!("unhit");
        Self(self.0.mul_mod(other.inverse().unwrap().0, MODULUS))
    }
}

impl DivAssign for Fr {
    fn div_assign(&mut self, other: Self) {
        panic!("unhit");
        *self = *self / other;
    }
}

impl<'a> Add<&'a Self> for Fr {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self(self.0.add_mod(other.0, MODULUS))
    }
}

impl<'a> AddAssign<&'a Self> for Fr {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other
    }
}

impl<'a> Sub<&'a Self> for Fr {
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self {
        if other.0 > self.0 {
            self.0 += MODULUS;
        }
        Self(self.0 - other.0)
    }
}

impl<'a> SubAssign<&'a Self> for Fr {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl<'a> Mul<&'a Self> for Fr {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        Self(self.0.mul_mod(other.0, MODULUS))
    }
}

impl<'a> MulAssign<&'a Self> for Fr {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

impl<'a> Div<&'a Self> for Fr {
    type Output = Self;

    fn div(self, other: &Self) -> Self {
        panic!("unhit");
        Self(self.0.mul_mod(other.inverse().unwrap().0, MODULUS))
    }
}

impl<'a> DivAssign<&'a Self> for Fr {
    fn div_assign(&mut self, other: &Self) {
        panic!("unhit");
        *self = *self / other;
    }
}

impl Sum<Fr> for Fr {
    /// Returns the `sum` of `self` and `other`.
    fn sum<I: Iterator<Item = Fr>>(iter: I) -> Self {
        iter.fold(Fr::zero(), |a, b| a + b)
    }
}

impl Display for Fr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        panic!("unhit");
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powers_of_g() {
        let two = Fr(uint!(2_U256));

        // Compute the expected powers of G.
        let g = Fr(GENERATOR).pow(T.as_limbs());
        let powers = (0..TWO_ADICITY - 1)
            .map(|i| g.pow(two.pow(&[i as u64]).0.as_limbs()))
            .collect::<Vec<_>>();

        // Ensure the correct number of powers of G are present.
        assert_eq!(POWERS_OF_G.len() as u64, (TWO_ADICITY - 1) as u64);
        assert_eq!(POWERS_OF_G.len(), powers.len());

        // Ensure the expected and candidate powers match.
        for (expected, candidate) in powers.iter().zip(POWERS_OF_G.iter()) {
            println!("{:?} =?= {:?}", expected, candidate);
            assert_eq!(*expected, Fr(*candidate));
        }
    }
}
