use crate::bls12_377::{field::Field, LegendreSymbol};
use bitvec::prelude::*;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use rand::{distributions::Standard, Rng};
use ruint::{uint, Uint};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fq(pub Uint<384, 6>);

pub const POWERS_OF_G: &[Uint<384, 6>] = &[
    uint!(11759432984210757955515102394259421622842731805301722003778799460755806109766954778381794158916389006258470283894_U384),
    uint!(76728627332054330107215521437663829484089413964084084888354490166669576688184173341868214290394688896870888326044_U384),
    uint!(50482501805661069943992631757164962942571953595228286301055053972545162690184639599216443159769039353792100390129_U384),
    uint!(123695493714764636462668136999088991052367675418621884534703396358972537705709193652837509570803093070415960718724_U384),
    uint!(222789836298762807679998200056747413351763884219816869220101610692671189722181341890937304056155554893237932901255_U384),
    uint!(140987985019544929266141609797064264804200447090826523913714498560192170008419470179898413233763825367731542701802_U384),
    uint!(47039689189678210549109787946363672604561639283401529961735215018378230022077965962845792785428758364516999844881_U384),
    uint!(154740723738977222056526920575817910996586791732057849501698768311783113876515975339582621824185238829759268029815_U384),
    uint!(43911847545658906028162656537347105288963188951766550071991665068283895369122883095463071474596487606578049157467_U384),
    uint!(114481681270099293252842629575463736748228914932326641146589664024144094450579950126422131481316824488228191233504_U384),
    uint!(23110820532736597550505718364307504347474166758319895846776139924289425461844777907460413516708391129880171885272_U384),
    uint!(49295773124294029926949217017334072243927484154261812082469417870431924216173568887805223724939974438041107879118_U384),
    uint!(191473704634510920549461328961759513917041772995437329340862276827908002399470964262815612459743207372229185673173_U384),
    uint!(197332550480710970318841249055552690747935528495616029260228674179894094099250813642315317848933932086909796265082_U384),
    uint!(75537776578128403477541546873463464395094350677472101984032573574677844858582947846146704150240902097612677239207_U384),
    uint!(32347198654630187123231628145073975217180321394012383986290011515014898598013410435184817424519952319589513425721_U384),
    uint!(95827542307852652763760772166465420412587843111323255260390315227690226260189597117176017134351868610091019603886_U384),
    uint!(162112016113869092132934507216468582634188720679405934654621520454669892616601221972611581904503074719017835038396_U384),
    uint!(96359187626329397598231487243203555522561966206498272378534621918347838060051136083137637049555624326059186547109_U384),
    uint!(129620623012118890340064357449634008771062518666602907153865167353634975766650972340369696066719834820109618245611_U384),
    uint!(104975755195070499270359856407227994202413493711179742999132592904212330912541580719969690735757806709283254690615_U384),
    uint!(49200237170501366338805821365808976469073303157906929096913524669065324039647391839216080462749221863337839292055_U384),
    uint!(138095588182958859265694048568065496804711026332426634289297041651295790350256750768441824580159281090694506033801_U384),
    uint!(227232812349891752438547771890081723491917629976581372376881570396025688436240009348614876692655130463606748206917_U384),
    uint!(79265140731237501694793425218173186450654421055181895926991611681988068483319720755468802434058637813394390193116_U384),
    uint!(163721394595365402705453781080596970503179763151666769868246942730930892145895608749780906703671617936976646398507_U384),
    uint!(141932323099167414444561028530281853430958443040964447387116695771789139867900301953819333294152959611419920119613_U384),
    uint!(200364828116803215066115128738375079849880414729149951140694801968150111861602388655460240501857827944276981165288_U384),
    uint!(206902602080282307407872537867139499413580190224632594609821849026217339343026609604135700514491128445429423280545_U384),
    uint!(122758507831338724194795617282807082798769179705065273914600890966716311577386303180185950183349287501836811607013_U384),
    uint!(237362717562717724788000156394593924721147593435772779069434350981356826284704217174103030141741426858683847753279_U384),
    uint!(192975480444015538766397619038631542247996104726301850374820466085784366693700794701720668104005562901204393261425_U384),
    uint!(17420519987285045180618776400281717827990065101798550575337130292817566279951192386544792166668709992464691690699_U384),
    uint!(68242745112510457372492684073393005935430322726389113191092033144886302354022170165381293503760356816766120546564_U384),
    uint!(189955123805754210816110951087616182681210743528582520199412967030676842107552804407012954318091839463380258494738_U384),
    uint!(103181813677129947152193940296234760550272245449631231147124214701416204532833999301966188516951681435252657076917_U384),
    uint!(163673577358507306894966012380366695261384538120267130890007308594918875071293032901344273977979469255757291894523_U384),
    uint!(61754823887392727579018286333155378694391089947921069040751622549860207884817196028214548976644717729137018902562_U384),
    uint!(14115312151147604974062141862747353658202101915501489954988989935836325189948573016025404668832497669218677885591_U384),
    uint!(242206922837788275566049131366472977767524393039470159011183968296131693878382398006172425179143434237397941985974_U384),
    uint!(164196037447229989397315098612365381650786267343044787881473017723859118535617804642721218853696789775056713132198_U384),
    uint!(134296832008385841254860775717799001586032949361853190196237326361224570948803510842645362149538951727166889660082_U384),
    uint!(114877572632286478126447674985906730334559399281805537038690459408296984634486192544632888082065820535181276983863_U384),
    uint!(16685831635117458619713971517806615622539106529274720406206304215896756979319939250397182757549194256480151045562_U384),
    uint!(216465761340224619389371505802605247630151569547285782856803747159100223055385581585702401816380679166954762214499_U384),
];

pub const TWO_ADICITY: u32 = 46u32;

pub const TWO_ADIC_ROOT_OF_UNITY: Uint<384, 6> = uint!(146552004846884389553264564610149105174701957497228680529098805315416492923550540437026734404078567406251254115855_U384);

pub const TWO_ADIC_ROOT_OF_UNITY_AS_FIELD: Fq = Fq(
    uint!(224889470004741437790876857038605399989314902261086046762625433320979911756295853335464037764645098727193119245337_U384),
);

pub const CAPACITY: u32 = MODULUS_BITS - 1;

/// GENERATOR = -5
pub const GENERATOR: Uint<384, 6> = uint!(92261639910053574722182574790803529333160366917737991650341130812388023949653897454961487930322210790384999596794_U384);

pub const GENERATOR_AS_FIELD: Fq = Fq(
    uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458172_U384),
);

pub const INV: u64 = 9586122913090633727u64;

/// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
pub const MODULUS: Uint<384, 6> = uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177_U384);

pub const MODULUS_BITS: u32 = 377;

pub const MODULUS_MINUS_ONE_DIV_TWO: Uint<384, 6> = uint!(129332213006484547005326366847446766768196756377457330269942131333360234174170411387484444069786680062220160729088_U384);

pub const R: Uint<384, 6> = uint!(85013442423176922659824578519796707547925331718418265885885478904210582549405549618995257669764901891699128663912_U384);

pub const R2: Uint<384, 6> = uint!(66127428376872697816332570116866232405230528984664918319606315420233909940404532140033099444330447428417853902114_U384);

pub const REPR_SHAVE_BITS: u32 = 7;

/// T = (MODULUS - 1) // 2^S =
/// 3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059
pub const T: Uint<384, 6> = uint!(3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059_U384);

/// (T - 1) // 2 =
/// 1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529
pub const T_MINUS_ONE_DIV_TWO: Uint<384, 6> = uint!(1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529_U384);

impl Fq {
    pub fn legendre(&self) -> LegendreSymbol {
        // s = self^((MODULUS - 1) // 2)
        let s = self.pow(MODULUS_MINUS_ONE_DIV_TWO.as_limbs());
        if s.is_zero() {
            LegendreSymbol::Zero
        } else if s.is_one() {
            LegendreSymbol::QuadraticResidue
        } else {
            LegendreSymbol::QuadraticNonResidue
        }
    }

    pub fn half() -> Self {
        Self((MODULUS + uint!(1_U384)) >> 1)
    }

    pub fn multiplicative_generator() -> Self {
        GENERATOR_AS_FIELD
    }

    pub fn is_valid(&self) -> bool {
        self.0 < MODULUS
    }

    fn reduce(&mut self) {
        while self.0 >= MODULUS {
            self.0 -= MODULUS;
        }
    }
}

impl Field for Fq {
    const PHI: Fq = Fq(
        uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
    );

    fn zero() -> Self {
        Self(uint!(0_U384))
    }

    fn is_zero(&self) -> bool {
        self.0 == Self::zero().0
    }

    fn one() -> Self {
        Self(uint!(1_U384))
    }

    fn is_one(&self) -> bool {
        self.0 == Self::one().0
    }

    fn rand() -> Self {
        let mut a = Self(rand::thread_rng().sample(Standard));
        a.reduce();
        a
    }

    fn characteristic<'a>() -> Self {
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
        self.0 = self.0.pow_mod(uint!(2_U384), MODULUS);
    }

    fn inverse(&self) -> Option<Self> {
        self.0.inv_mod(MODULUS).map(Self)
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(res) = self.0.inv_mod(MODULUS) {
            *self = Self(res);
            Some(self)
        } else {
            panic!("unhit");
            None
        }
    }

    fn sqrt(&self) -> Option<Self> {
        // https://eprint.iacr.org/2020/1407.pdf (page 4, algorithm 1)
        match self.legendre() {
            LegendreSymbol::Zero => Some(*self),
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
        *self * Self::PHI
    }
}

impl Add for Fq {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add_mod(other.0, MODULUS))
    }
}

impl AddAssign for Fq {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Fq {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if other.0 > self.0 {
            self.0 += MODULUS;
        }
        Self(self.0 - other.0)
    }
}

impl SubAssign for Fq {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Fq {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0.mul_mod(other.0, MODULUS))
    }
}

impl MulAssign for Fq {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Neg for Fq {
    type Output = Self;

    fn neg(self) -> Self {
        let mut a = Self(MODULUS - self.0);
        a.reduce();
        a
    }
}

impl Div for Fq {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0.mul_mod(other.inverse().unwrap().0, MODULUS))
    }
}

impl DivAssign for Fq {
    fn div_assign(&mut self, other: Self) {
        panic!("unhit");
        *self = *self / other;
    }
}

impl<'a> Add<&'a Self> for Fq {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self(self.0.add_mod(other.0, MODULUS))
    }
}

impl<'a> AddAssign<&'a Self> for Fq {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}

impl<'a> Sub<&'a Self> for Fq {
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self {
        if other.0 > self.0 {
            self.0 += MODULUS;
        }
        Self(self.0 - other.0)
    }
}

impl<'a> SubAssign<&'a Self> for Fq {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl<'a> Mul<&'a Self> for Fq {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        Self(self.0.mul_mod(other.0, MODULUS))
    }
}

impl<'a> MulAssign<&'a Self> for Fq {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

impl<'a> Div<&'a Self> for Fq {
    type Output = Self;

    fn div(self, other: &Self) -> Self {
        panic!("unhit");
        Self(self.0.mul_mod(other.inverse().unwrap().0, MODULUS))
    }
}

impl<'a> DivAssign<&'a Self> for Fq {
    fn div_assign(&mut self, other: &Self) {
        panic!("unhit");
        *self = *self / other;
    }
}

impl Sum<Fq> for Fq {
    /// Returns the `sum` of `self` and `other`.
    fn sum<I: Iterator<Item = Fq>>(iter: I) -> Self {
        iter.fold(Fq::zero(), |a, b| a + b)
    }
}

impl Display for Fq {
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
        let two = Fq(uint!(2_U384));

        // Compute the expected powers of G.
        let g = Fq(GENERATOR).pow(T.as_limbs());
        let powers = (0..TWO_ADICITY - 1)
            .map(|i| g.pow(two.pow(&[i as u64]).0.as_limbs()))
            .collect::<Vec<_>>();

        // Ensure the correct number of powers of G are present.
        assert_eq!(POWERS_OF_G.len() as u64, (TWO_ADICITY - 1) as u64);
        assert_eq!(POWERS_OF_G.len(), powers.len());

        // Ensure the expected and candidate powers match.
        for (expected, candidate) in powers.iter().zip(POWERS_OF_G.iter()) {
            println!("{:?} =?= {:?}", expected, candidate);
            assert_eq!(*expected, Fq(*candidate));
        }
    }
}
