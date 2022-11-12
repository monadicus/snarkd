// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::bls12_377::{fp, scalar, Field, Fp, Scalar};
use bitvec::prelude::*;
use ruint::{uint, Uint};
use smallvec::SmallVec;
use std::{
    ops::{Deref, Index, IndexMut},
    sync::Arc,
};

const RATE: usize = 2;
const CAPACITY: usize = 1;

/// A function for computing ceil(log2(x))+1 for a field element x
fn overhead(num: Fp) -> usize {
    let num_bits = num
        .0
        .as_limbs()
        .iter()
        .flat_map(|limb| limb.view_bits::<Lsb0>())
        .map(|b| *b.deref())
        .rev()
        .collect::<Vec<_>>();
    let mut skipped_bits = 0;
    for b in num_bits.iter() {
        if *b == false {
            skipped_bits += 1;
        } else {
            break;
        }
    }

    let mut is_power_of_2 = true;
    for b in num_bits.iter().skip(skipped_bits + 1) {
        if *b == true {
            is_power_of_2 = false;
        }
    }

    if is_power_of_2 {
        num_bits.len() - skipped_bits
    } else {
        num_bits.len() - skipped_bits + 1
    }
}

/// Obtain the parameters from a `ConstraintSystem`'s cache or generate a new one
pub const fn get_params(
    target_field_size: usize,
    base_field_size: usize,
    optimization_type: OptimizationType,
) -> (usize, usize) {
    find_parameters(base_field_size, target_field_size, optimization_type)
}

/// A function to search for parameters for nonnative field gadgets
pub const fn find_parameters(
    base_field_prime_length: usize,
    target_field_prime_bit_length: usize,
    optimization_type: OptimizationType,
) -> (usize, usize) {
    let mut found = false;
    let mut min_cost = 0usize;
    let mut min_cost_limb_size = 0usize;
    let mut min_cost_num_of_limbs = 0usize;

    let surfeit = 10;
    let mut max_limb_size = (base_field_prime_length - 1 - surfeit - 1) / 2 - 1;
    if max_limb_size > target_field_prime_bit_length {
        max_limb_size = target_field_prime_bit_length;
    }
    let mut limb_size = 1;

    while limb_size <= max_limb_size {
        let num_of_limbs = (target_field_prime_bit_length + limb_size - 1) / limb_size;

        let group_size =
            (base_field_prime_length - 1 - surfeit - 1 - 1 - limb_size + limb_size - 1) / limb_size;
        let num_of_groups = (2 * num_of_limbs - 1 + group_size - 1) / group_size;

        let mut this_cost = 0;

        match optimization_type {
            OptimizationType::Constraints => {
                this_cost += 2 * num_of_limbs - 1;
            }
            OptimizationType::Weight => {
                this_cost += 6 * num_of_limbs * num_of_limbs;
            }
        };

        match optimization_type {
            OptimizationType::Constraints => {
                this_cost += target_field_prime_bit_length; // allocation of k
                this_cost += target_field_prime_bit_length + num_of_limbs; // allocation of r
                                                                           //this_cost += 2 * num_of_limbs - 1; // compute kp
                this_cost += num_of_groups + (num_of_groups - 1) * (limb_size * 2 + surfeit) + 1;
                // equality check
            }
            OptimizationType::Weight => {
                this_cost += target_field_prime_bit_length * 3 + target_field_prime_bit_length; // allocation of k
                this_cost += target_field_prime_bit_length * 3
                    + target_field_prime_bit_length
                    + num_of_limbs; // allocation of r
                this_cost += num_of_limbs * num_of_limbs + 2 * (2 * num_of_limbs - 1); // compute kp
                this_cost += num_of_limbs
                    + num_of_groups
                    + 6 * num_of_groups
                    + (num_of_groups - 1) * (2 * limb_size + surfeit) * 4
                    + 2; // equality check
            }
        };

        if !found || this_cost < min_cost {
            found = true;
            min_cost = this_cost;
            min_cost_limb_size = limb_size;
            min_cost_num_of_limbs = num_of_limbs;
        }

        limb_size += 1;
    }

    (min_cost_num_of_limbs, min_cost_limb_size)
}

pub enum OptimizationType {
    Weight,
    Constraints,
}

/// The mode structure for duplex sponges.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DuplexSpongeMode {
    /// The sponge is currently absorbing data.
    Absorbing {
        /// The next position of the state to be XOR-ed when absorbing.
        next_absorb_index: usize,
    },
    /// The sponge is currently squeezing data out.
    Squeezing {
        /// The next position of the state to be outputted when squeezing.
        next_squeeze_index: usize,
    },
}

/// Parameters and RNG used
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoseidonParameters {
    /// number of rounds in a full-round operation
    pub full_rounds: usize,
    /// number of rounds in a partial-round operation
    pub partial_rounds: usize,
    /// Exponent used in S-boxes
    pub alpha: u64,
    /// Additive Round keys. These are added before each MDS matrix application to make it an affine shift.
    /// They are indexed by `ark[round_num][state_element_index]`
    pub ark: Vec<Vec<Fp>>,
    /// Maximally Distance Separating Matrix.
    pub mds: Vec<Vec<Fp>>,
}

impl Default for PoseidonParameters {
    fn default() -> Self {
        Self {
            full_rounds: 8,
            partial_rounds: 31,
            alpha: 17,
            ark: vec![
                vec![
                    Fp(
                        uint!(123249878756453098914639601843199176451997132612914162343590671120179979107846114348064675842753496966502226470504_U384),
                    ),
                    Fp(
                        uint!(53905766173893895260794869709237214385817399454448711667759505042599362214601718682151848385057179500606557721647_U384),
                    ),
                    Fp(
                        uint!(69894258097921391480299485244196654800484322255007476002974737215216019155108287854575586445309048623300976500186_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(199269668121875174262570566455689951380204776091573924798659006690560053061129973379838694860592058095048653996886_U384),
                    ),
                    Fp(
                        uint!(238380155638054426865611280966399840311283670977656700124343990049337832223435242290330416091629395326468367200694_U384),
                    ),
                    Fp(
                        uint!(212599814638151740594239938840408336056840064513659388805072396583467200575230295920880684207605497942975271963482_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(168914555474650585865140636066457509664822869961119817085015902784107763611497575013588473080216753361935154707010_U384),
                    ),
                    Fp(
                        uint!(53776337623194839368137436133474167179306472987260969806083684345990583528478024243778418311781192352786333037262_U384),
                    ),
                    Fp(
                        uint!(248867522100291115924418017563087071912585010573958563496624003376931076896846052799391847772671448846373554213551_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(35631741058397496924366231156673935881565943972602937094522045989256363839293709188784238224940964750407897277330_U384),
                    ),
                    Fp(
                        uint!(7156811532468409927576845751990203959972794842929038664826572233020786824205198784067484739611297952558975673525_U384),
                    ),
                    Fp(
                        uint!(15979461281492123433122857594463244790261784547146673175073000444677214597242748768087325039007316516299176001509_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(49754305211992756860291736450940496115500536038609822532547985566439150683219315252172063528174877535028115611426_U384),
                    ),
                    Fp(
                        uint!(216949553183571701463265648286619401188451882876550757881148401346730830975776784112086074385527611896268776861443_U384),
                    ),
                    Fp(
                        uint!(154083689848809196835533626226861291475925228965341568449375421928198779718328545609801450631059855774468437183675_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(29502137502944860067241987089780210878930586459241857665873534227953181087265906053919742367508518196418106799806_U384),
                    ),
                    Fp(
                        uint!(132373035808136518827992049261301947450498154936614023679388954300081661784851944028690271115929087672833323628947_U384),
                    ),
                    Fp(
                        uint!(215747065685210104280208334912564361804699328020235674942496660758226155688200145092731052953352829033676863042630_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(199648585544625597282043439398719700409246757664428471828724582419530290323495031580337339234017647369916547108958_U384),
                    ),
                    Fp(
                        uint!(249575928844995465269738608819476286372884074177639142297081916221358214871660642843838074316560663218386973740173_U384),
                    ),
                    Fp(
                        uint!(74982114655706235696493453220768307411520767156884132118410225505977592728838652389837915751053304413004683265639_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(106210893563839260576304917669354671677368166928359922623554581531406088660838991706361575276657684361659801532597_U384),
                    ),
                    Fp(
                        uint!(11585440423875492387746565618452234080951922019833673083821688269701182965167436520603220148800340540649190539129_U384),
                    ),
                    Fp(
                        uint!(37259364694251003983990539546703073907090415386678577600390274977885009271501265285951467194762590248232970812844_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(55837576930986823158863800928077105077853280536700135646766922885911998320579725325719074294029609849816879406734_U384),
                    ),
                    Fp(
                        uint!(116196118812458208678900768001429737210506949071720002979523997962887466062064707950742955679705357069634209515723_U384),
                    ),
                    Fp(
                        uint!(24815444638034932833671809997597970940772642987124330190627003560135207315166813788012165972582101193880572012425_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(8273799170260651595038492091530332589844019793817674372861920239816475852471908767091347071442643736888815451573_U384),
                    ),
                    Fp(
                        uint!(136990111822759715389631392741048451444971543778803264358207793191138912342988121207664006283186301023235486962908_U384),
                    ),
                    Fp(
                        uint!(18927153358572748727167231887593945930709178220781358813059367890606662567925981344966823750216495960065937779382_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(14853717798346258618706074618305350695598054492875071420312670809589654546598863746625188519698040835608660556159_U384),
                    ),
                    Fp(
                        uint!(176244718044988586163620753193829773891006448729185890339575543133809251309372861124810944047181141986328457412271_U384),
                    ),
                    Fp(
                        uint!(110233743777966819273995158642051347290508079434162581354613179685804039325709118867348142870653771761630005888307_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(161450408187838611032909671423510614052593225149162808041124828019352169325631782682210492475825053268732766729188_U384),
                    ),
                    Fp(
                        uint!(98500573657597535150392453836987141880178711694344573271124963035313026654066107879785978599420939724454330812177_U384),
                    ),
                    Fp(
                        uint!(215876031358183401857867635719035351422270130594078940310356834104879903855422762837568172975859284057413791888463_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(204131296462104965227188513952192358580161695271034405933269755582850293970852406144296664401269366372941792250467_U384),
                    ),
                    Fp(
                        uint!(249055944105228847655227995674839790690527612872758434023675475202902983562708467495202781909125241976893640769485_U384),
                    ),
                    Fp(
                        uint!(229583286868130259500413761228235662329364304128164289006746728927752301094007770574061957905615623121952293733410_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(97517137752483519086795583001379387731583152856232248443468839338330057977841917349007821334306740790291136905974_U384),
                    ),
                    Fp(
                        uint!(123488479251161582154755930609622851433258511862463208593787895860046694339616550157942520077460765622263030118175_U384),
                    ),
                    Fp(
                        uint!(71432639825611523000280189495110508914555485498103026713477936527348359478511563831157563324853527351478004088468_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(91036072174315573792700064386146501824720160045153964050728880763049550271037560479809028105202996773568857740730_U384),
                    ),
                    Fp(
                        uint!(22543564450401763754262340909190687557385187274502421381039682479049063587284520644182139148382788770792136350730_U384),
                    ),
                    Fp(
                        uint!(142332951471076179551307567596387601171650552060403080229506160329597397458669457278907083453911143048367692807957_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(132220734042377172239294549962805515211262743615319266088172915692615455860531484953442975677793502323549653807013_U384),
                    ),
                    Fp(
                        uint!(93545141080589996877640088231346264823743396787843686206971590288437291906435217842171096954488932034021955982341_U384),
                    ),
                    Fp(
                        uint!(240853888813002049402641151657197764532471620278969626757294146309548064471722973918761650243980940919903584631021_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(213503951761453329038225269663723790274543267128942326856880800168236861547603473591480303861374397603917184363409_U384),
                    ),
                    Fp(
                        uint!(89903237953544441905563167047407202265037317870234905464628470820413104873403912116742106741939288646681955585592_U384),
                    ),
                    Fp(
                        uint!(227121824801807544842683518849178395477499272684097761652696447845872786929195257751449337349649535876783186356932_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(146971666607035715052553690155718843961663952406456998981945817009558492075030732771578449344145496025583596767529_U384),
                    ),
                    Fp(
                        uint!(134089029253068479750825302615074040106242441439845487647903191411265000857473209669062720892950980761449114307448_U384),
                    ),
                    Fp(
                        uint!(240876825504060088346683291079269022914405381209699533928214418428379986520457497863030431018122239809907227823545_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(198679995161578152944752940670334322637799809857648522826858388680172266023884005933440419287476164086770000386213_U384),
                    ),
                    Fp(
                        uint!(80453254513068178946616210391952329341738228131537630777936072121633132376974015675425930731821852982135052772824_U384),
                    ),
                    Fp(
                        uint!(51768068183070369841309308465744117994964313769378589398952388439182600629247824076033474616974680361718264496789_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(243786304512064454489442716415645262128218178912097043568997297339729319251009514316482045843945939785873311024862_U384),
                    ),
                    Fp(
                        uint!(132173037488875105639933852791191619959134471035456041997878656537714362390384670197604289467581846432000497395848_U384),
                    ),
                    Fp(
                        uint!(138604002173172705882182745730007697550901886293221788738303534900559003963900219115006541529324886578352274293799_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(81783919742603431816536303551235523053319325628680028340677898253811841771498386894771134375242031554657528159968_U384),
                    ),
                    Fp(
                        uint!(89996400559826291686063370272745776928773053585174906250124744120004601102635457051808673956966087024872962073778_U384),
                    ),
                    Fp(
                        uint!(12344123991576028812375373502965766640863831483294590816896451707123374600150201588149068234468387476695336142872_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(126658015711577921340966771989453650175842088716604137911295183366663485409400992312567694787591845600554914654115_U384),
                    ),
                    Fp(
                        uint!(164573749458837881364642242529191795392373682411872943164652677729048094673511958737424619008331062199862267652935_U384),
                    ),
                    Fp(
                        uint!(143664707544522749631081019060087611028964440272897357239195964754781588855456478370128855886667526444876450715220_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(190063502426458192727293662114673159337018305482738016082798402909947909388433256561924969169284825978832455579368_U384),
                    ),
                    Fp(
                        uint!(200570271046622734241692574928890759512247601848653772722076665026354776331148830989844078413438205377226077381532_U384),
                    ),
                    Fp(
                        uint!(138002415082423685424410551811447526297743243297262932785520614237184932570821640271043572260989269814779470761461_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(16788676705864143878780230479765282864054741033672656690224477402805235181341884274547412331727211099012342081859_U384),
                    ),
                    Fp(
                        uint!(204290600886783875333612666138119904239583082229871768433568000092203989815186589303588884701205693229512519768754_U384),
                    ),
                    Fp(
                        uint!(87038987841167673770859932175226012933997089943393502222169060963262863845214906568997443646438042896398425595517_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(36339730403510893807656584803012696279422432037251730189319369181711761371163710757212065138707754369092877655154_U384),
                    ),
                    Fp(
                        uint!(23719136079159372599286451744989936807954969964666516807332295420486880070514166596679589399139358707568583760908_U384),
                    ),
                    Fp(
                        uint!(56393335057571631799160728164218189604902690263179612889078150181027528679320914138536210530501845237163318197428_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(205825956035491267343111682188790766922328411605275469211275484195313659964988531094479492782154028581379936224444_U384),
                    ),
                    Fp(
                        uint!(14251323509232608512846002255486393977548730149242264667463070512925839406395836441387775340864744223546556498715_U384),
                    ),
                    Fp(
                        uint!(78428895560820169309169428677090706087502853851935641954584167534512067284012881590143110425966068532035695668777_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(75494383501361595510879099604200999089073272552094921752996800680267084650818676639185519371499429119407927521694_U384),
                    ),
                    Fp(
                        uint!(71654751419236499966546173490894599834311797714598165686807217633186393301928260640596079166780877531085221325785_U384),
                    ),
                    Fp(
                        uint!(200578082042519003217027186194032673613554519507662494009516442239977006673663941756393116663841297396793491871200_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(120280384146306862951854508424447098979618461682025441151850969362942271625861150381428890843919546149633622105768_U384),
                    ),
                    Fp(
                        uint!(227475425496153223669855864055613669014065977392917058770175352117179491094064142348157299350182313499504389083442_U384),
                    ),
                    Fp(
                        uint!(251127263423734302912203519333198755054413799582445749881827904612771493287021107263113755730642765378206506332728_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(83702595636895308967051271584382753663326775825724154674461807131791275318302215831042606082449545102374950849149_U384),
                    ),
                    Fp(
                        uint!(72457985217378059985209058682320070298806205003882947360107581077425648268857982638575115120572096951096305132848_U384),
                    ),
                    Fp(
                        uint!(12116600973201943572988978934130839409963908949941838392365368398743958008280031214900074753572240221871297157796_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(240872572144156225455781664515486127362275317595363215915293841253269790726868349873274949641462036923410553640448_U384),
                    ),
                    Fp(
                        uint!(145005621445512968320023394688234446061157047306027479183225589915851108312974841851900985683181027983777819469749_U384),
                    ),
                    Fp(
                        uint!(223934906758737028193582875327881601162900418521869327818828928797111524239009182764598636421899745113893918838102_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(16041135858962966773434394701665023495889307936385789447051685789622713476233465453520183391926457145978975456780_U384),
                    ),
                    Fp(
                        uint!(100995326650741809373350376300291093265611246694300366918949313510272548230989953212376186670081618363334860819266_U384),
                    ),
                    Fp(
                        uint!(198113061836041953087296741499457296947901762958345262407373960882722071735229745555760175641534017765920249851403_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(160310964282803191210156178260469498817686363777861893444880694842806130876775742977058740003184226096711472502332_U384),
                    ),
                    Fp(
                        uint!(188713129639597187156378905515616933770775761286091242780337441914651066308540192205692023798450603034519453279164_U384),
                    ),
                    Fp(
                        uint!(144177371846162732968346932904974285173557315948314203099016729242538001323624139665700501564547696462348047085475_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(79270873425284875854185620699109770597921524227459035513636651263949603822776268395349011178814936290604749327216_U384),
                    ),
                    Fp(
                        uint!(66634508562919326060253106867724866375414704994924403132729353386392729560099833340809999504328104294822126690206_U384),
                    ),
                    Fp(
                        uint!(153929451747036516277146884279088023240503545576502622475104547924498837499332163003522743849174380874173903478589_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(65951591639970943843478787167093376292485300299245482252716091066831460583153445126961516774641242644059740963631_U384),
                    ),
                    Fp(
                        uint!(218283324593072992330537678366612521138133713936527225314279366375484764183384762101590493464257294993736058798003_U384),
                    ),
                    Fp(
                        uint!(255801326343293104028075157882719596846119525365262151647658801094843254475907908556215545683201236013153654096091_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(226255389453600272835601278226928175590352392261397636954040403683064727971365284972741836048745971086673805312770_U384),
                    ),
                    Fp(
                        uint!(30094566584570359029617856208266980210102789615056943080637739339632299082666382408767896640283618386400863011377_U384),
                    ),
                    Fp(
                        uint!(171014403954507192635907791911496156579477488568451453501143540559952206171633891640382019016227963532953321760176_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(166057204219683871752892448206953243424627338207417177280506199576386068200127812837156087933305873775343563022702_U384),
                    ),
                    Fp(
                        uint!(189980739384556361714711372786771245267076300911771323385655044819119270337048535106665515768517077503660696853087_U384),
                    ),
                    Fp(
                        uint!(160509966668023670725615598656132311085788181242287915812481624013950278259314541983309947248633680202474798784113_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(121604680206118278311858973633579806987780447456690173958929756615242378735587345162043644789250322132552405934838_U384),
                    ),
                    Fp(
                        uint!(162490787868836358365957714904092588505217178719637049967797863955517541278871433068812149053958672871873339777657_U384),
                    ),
                    Fp(
                        uint!(186725839885149672835245872626306502017366920295670132626156737796246154714707858273955752031344539280320214023217_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(1595442381035683601009655514607864917155264882908420917897267779293136954609652688808389170558528873507396022657_U384),
                    ),
                    Fp(
                        uint!(136133658372771228168254201060050291177683595113705517331628662542619211285959494716428905546778127973286832435248_U384),
                    ),
                    Fp(
                        uint!(235707281471584662954139438770000959801075760015072690205031932435280838811659817426504701946918628382850116491607_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(218394064516331833020386245120198448098388776182164066507039096886784654454748249393443008924076322437744672962940_U384),
                    ),
                    Fp(
                        uint!(171630003249069743969583651512237853143542592922081517495872510118379411011409238640358871094120884164999614012_U384),
                    ),
                    Fp(
                        uint!(106352495811714591674517100311841383873861724084673517408579093193910563925812357978278276551276192431523493134802_U384),
                    ),
                ],
            ],
            mds: vec![
                vec![
                    Fp(
                        uint!(35463799792750972803746014831251318629433070651916394903137949221437209577677273605833717469941575569104741526451_U384),
                    ),
                    Fp(
                        uint!(18525374364661750307440824350340771293424609245159218207409253749617918442029080961367157063966182839254983576724_U384),
                    ),
                    Fp(
                        uint!(96313611821735511449591580163083975587347120205529218061849469348716252837177987500111192232021055962542059542412_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(184610826894298373826952030256215485452556494530798726246415694794196222735666067140505346074672032818873376193660_U384),
                    ),
                    Fp(
                        uint!(169170114062164939552104715979827042386033829996509029655899361104098442853225147615546393356393444238242438049980_U384),
                    ),
                    Fp(
                        uint!(24177241132903335121524689415818818107920151023402250200813429563196326173884815770339346817801446861279643703952_U384),
                    ),
                ],
                vec![
                    Fp(
                        uint!(17228430949886884828033371768349883299641066192821547195081333400086665473981454169936377873256566147576607049992_U384),
                    ),
                    Fp(
                        uint!(35113533023170247280272066588387614578863541036869539331927201531038853371598133096624809442419922813566246641442_U384),
                    ),
                    Fp(
                        uint!(225762263795139846379155325981635321549752796953252150370574780810431415761301654496442331322761087421338650655933_U384),
                    ),
                ],
            ],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct State {
    capacity_state: [Fp; CAPACITY],
    rate_state: [Fp; RATE],
}

impl Default for State {
    fn default() -> Self {
        Self {
            capacity_state: [Fp::ZERO; CAPACITY],
            rate_state: [Fp::ZERO; RATE],
        }
    }
}

impl State {
    /// Returns an immutable iterator over the state.
    pub fn iter(&self) -> impl Iterator<Item = &Fp> + Clone {
        self.capacity_state.iter().chain(self.rate_state.iter())
    }

    /// Returns an immutable iterator over the state.
    pub fn into_iter(&self) -> impl Iterator<Item = Fp> + Clone {
        self.capacity_state
            .into_iter()
            .chain(self.rate_state.into_iter())
    }

    /// Returns an mutable iterator over the state.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Fp> {
        self.capacity_state
            .iter_mut()
            .chain(self.rate_state.iter_mut())
    }
}

impl Index<usize> for State {
    type Output = Fp;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < RATE + CAPACITY,
            "Index out of bounds: index is {} but length is {}",
            index,
            RATE + CAPACITY
        );
        if index < CAPACITY {
            &self.capacity_state[index]
        } else {
            &self.rate_state[index - CAPACITY]
        }
    }
}

impl IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < RATE + CAPACITY,
            "Index out of bounds: index is {} but length is {}",
            index,
            RATE + CAPACITY
        );
        if index < CAPACITY {
            &mut self.capacity_state[index]
        } else {
            &mut self.rate_state[index - CAPACITY]
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poseidon {
    parameters: Arc<PoseidonParameters>,
}

impl Poseidon {
    /// Initializes a new instance of the cryptographic hash function.
    pub fn setup() -> Self {
        Self {
            parameters: Arc::new(PoseidonParameters::default()),
        }
    }

    /// Evaluate the cryptographic hash function over a list of field elements as input.
    pub fn evaluate(&self, input: &[Fp]) -> Fp {
        self.evaluate_many(input, 1)[0]
    }

    /// Evaluate the cryptographic hash function over a list of field elements as input,
    /// and returns the specified number of field elements as output.
    pub fn evaluate_many(&self, input: &[Fp], num_outputs: usize) -> Vec<Fp> {
        let mut sponge = PoseidonSponge::new(self.parameters);
        sponge.absorb_native_field_elements(input);
        sponge.squeeze_native_field_elements(num_outputs).to_vec()
    }

    /// Evaluate the cryptographic hash function over a non-fixed-length vector,
    /// in which the length also needs to be hashed.
    pub fn evaluate_with_len(&self, input: &[Fp]) -> Fp {
        self.evaluate(&[vec![Fp::from(input.len() as u64)], input.to_vec()].concat())
    }

    pub fn parameters(&self) -> &Arc<PoseidonParameters> {
        &self.parameters
    }
}

/// A duplex sponge based using the Poseidon permutation.
///
/// This implementation of Poseidon is entirely from Fractal's implementation in [COS20][cos]
/// with small syntax changes.
///
/// [cos]: https://eprint.iacr.org/2019/1076
#[derive(Clone, Debug)]
pub struct PoseidonSponge {
    /// Sponge Parameters
    parameters: Arc<PoseidonParameters>,
    /// Current sponge's state (current elements in the permutation block)
    state: State,
    /// Current mode (whether its absorbing or squeezing)
    pub mode: DuplexSpongeMode,
}

impl PoseidonSponge {
    pub fn new(parameters: Arc<PoseidonParameters>) -> Self {
        Self {
            parameters: parameters.clone(),
            state: State::default(),
            mode: DuplexSpongeMode::Absorbing {
                next_absorb_index: 0,
            },
        }
    }

    /// Takes in field elements.
    pub fn absorb_native_field_elements(&mut self, elements: &[Fp]) {
        let input = elements.to_vec();
        if !input.is_empty() {
            match self.mode {
                DuplexSpongeMode::Absorbing {
                    mut next_absorb_index,
                } => {
                    if next_absorb_index == RATE {
                        self.permute();
                        next_absorb_index = 0;
                    }
                    self.absorb_internal(next_absorb_index, &input);
                }
                DuplexSpongeMode::Squeezing {
                    next_squeeze_index: _,
                } => {
                    self.permute();
                    self.absorb_internal(0, &input);
                }
            }
        }
    }

    /// Takes in field elements.
    pub fn absorb_nonnative_field_elements(&mut self, elements: impl IntoIterator<Item = Scalar>) {
        Self::push_elements_to_sponge(self, elements, OptimizationType::Weight);
    }

    pub fn squeeze_nonnative_field_elements(&mut self, num: usize) -> SmallVec<[Scalar; 10]> {
        self.get_fe(num, false)
    }

    pub fn squeeze_native_field_elements(&mut self, num_elements: usize) -> SmallVec<[Fp; 10]> {
        if num_elements == 0 {
            return SmallVec::<[Fp; 10]>::new();
        }
        let mut output = smallvec::smallvec![Fp::ZERO; num_elements];

        match self.mode {
            DuplexSpongeMode::Absorbing {
                next_absorb_index: _,
            } => {
                self.permute();
                self.squeeze_internal(0, &mut output[..num_elements]);
            }
            DuplexSpongeMode::Squeezing {
                mut next_squeeze_index,
            } => {
                if next_squeeze_index == RATE {
                    self.permute();
                    next_squeeze_index = 0;
                }
                self.squeeze_internal(next_squeeze_index, &mut output[..num_elements]);
            }
        }

        output.truncate(num_elements);
        output
    }

    /// Takes out field elements of 168 bits.
    pub fn squeeze_short_nonnative_field_elements(&mut self, num: usize) -> SmallVec<[Scalar; 10]> {
        self.get_fe(num, true)
    }

    /// Takes out a field element of 168 bits.
    pub fn squeeze_short_nonnative_field_element(&mut self) -> Scalar {
        self.squeeze_short_nonnative_field_elements(1)[0]
    }
}

impl PoseidonSponge {
    #[inline]
    fn apply_ark(&mut self, round_number: usize) {
        for (state_elem, ark_elem) in self
            .state
            .iter_mut()
            .zip(&self.parameters.ark[round_number])
        {
            *state_elem += ark_elem;
        }
    }

    #[inline]
    fn apply_s_box(&mut self, is_full_round: bool) {
        if is_full_round {
            // Full rounds apply the S Box (x^alpha) to every element of state
            for elem in self.state.iter_mut() {
                *elem = elem.pow(&[self.parameters.alpha]);
            }
        } else {
            // Partial rounds apply the S Box (x^alpha) to just the first element of state
            self.state[0] = self.state[0].pow(&[self.parameters.alpha]);
        }
    }

    #[inline]
    fn apply_mds(&mut self) {
        let mut new_state = State::default();
        new_state
            .iter_mut()
            .zip(&self.parameters.mds)
            .for_each(|(new_elem, mds_row)| {
                *new_elem = Fp::sum_of_products(self.state.into_iter(), (*mds_row).into_iter());
            });
        self.state = new_state;
    }

    #[inline]
    fn permute(&mut self) {
        // Determine the partial rounds range bound.
        let partial_rounds = self.parameters.partial_rounds;
        let full_rounds = self.parameters.full_rounds;
        let full_rounds_over_2 = full_rounds / 2;
        let partial_round_range = full_rounds_over_2..(full_rounds_over_2 + partial_rounds);

        // Iterate through all rounds to permute.
        for i in 0..(partial_rounds + full_rounds) {
            let is_full_round = !partial_round_range.contains(&i);
            self.apply_ark(i);
            self.apply_s_box(is_full_round);
            self.apply_mds();
        }
    }

    /// Absorbs everything in elements, this does not end in an absorption.
    #[inline]
    fn absorb_internal(&mut self, mut rate_start: usize, input: &[Fp]) {
        if !input.is_empty() {
            let first_chunk_size = std::cmp::min(RATE - rate_start, input.len());
            let num_elements_remaining = input.len() - first_chunk_size;
            let (first_chunk, rest_chunk) = input.split_at(first_chunk_size);
            let rest_chunks = rest_chunk.chunks(RATE);
            // The total number of chunks is `elements[num_elements_remaining..].len() / RATE`, plus 1
            // for the remainder.
            let total_num_chunks = 1 + // 1 for the first chunk
                // We add all the chunks that are perfectly divisible by `RATE`
                (num_elements_remaining / RATE) +
                // And also add 1 if the last chunk is non-empty
                // (i.e. if `num_elements_remaining` is not a multiple of `RATE`)
                usize::from((num_elements_remaining % RATE) != 0);

            // Absorb the input elements, `RATE` elements at a time, except for the first chunk, which
            // is of size `RATE - rate_start`.
            for (i, chunk) in std::iter::once(first_chunk).chain(rest_chunks).enumerate() {
                for (element, state_elem) in
                    chunk.iter().zip(&mut self.state.rate_state[rate_start..])
                {
                    *state_elem += element;
                }
                // Are we in the last chunk?
                // If so, let's wrap up.
                if i == total_num_chunks - 1 {
                    self.mode = DuplexSpongeMode::Absorbing {
                        next_absorb_index: rate_start + chunk.len(),
                    };
                    return;
                } else {
                    self.permute();
                }
                rate_start = 0;
            }
        }
    }

    /// Squeeze |output| many elements. This does not end in a squeeze
    #[inline]
    fn squeeze_internal(&mut self, mut rate_start: usize, output: &mut [Fp]) {
        let output_size = output.len();
        if output_size != 0 {
            let first_chunk_size = std::cmp::min(RATE - rate_start, output.len());
            let num_output_remaining = output.len() - first_chunk_size;
            let (first_chunk, rest_chunk) = output.split_at_mut(first_chunk_size);
            assert_eq!(rest_chunk.len(), num_output_remaining);
            let rest_chunks = rest_chunk.chunks_mut(RATE);
            // The total number of chunks is `output[num_output_remaining..].len() / RATE`, plus 1
            // for the remainder.
            let total_num_chunks = 1 + // 1 for the first chunk
                // We add all the chunks that are perfectly divisible by `RATE`
                (num_output_remaining / RATE) +
                // And also add 1 if the last chunk is non-empty
                // (i.e. if `num_output_remaining` is not a multiple of `RATE`)
                usize::from((num_output_remaining % RATE) != 0);

            // Absorb the input output, `RATE` output at a time, except for the first chunk, which
            // is of size `RATE - rate_start`.
            for (i, chunk) in std::iter::once(first_chunk).chain(rest_chunks).enumerate() {
                let range = rate_start..(rate_start + chunk.len());
                debug_assert_eq!(
                    chunk.len(),
                    self.state.rate_state[range.clone()].len(),
                    "failed with squeeze {} at rate {} and rate_start {}",
                    output_size,
                    RATE,
                    rate_start
                );
                chunk.copy_from_slice(&self.state.rate_state[range]);
                // Are we in the last chunk?
                // If so, let's wrap up.
                if i == total_num_chunks - 1 {
                    self.mode = DuplexSpongeMode::Squeezing {
                        next_squeeze_index: (rate_start + chunk.len()),
                    };
                    return;
                } else {
                    self.permute();
                }
                rate_start = 0;
            }
        }
    }

    /// Compress every two elements if possible.
    /// Provides a vector of (limb, num_of_additions), both of which are F.
    pub fn compress_elements(src_limbs: &[(Fp, Fp)], ty: OptimizationType) -> Vec<Fp> {
        let capacity = (fp::MODULUS_BITS - 1) as usize;
        let mut dest_limbs = Vec::<Fp>::new();

        let (_, bits_per_limb) =
            get_params(scalar::MODULUS_BITS as usize, fp::MODULUS_BITS as usize, ty);

        let adjustment_factor_lookup_table = {
            let mut table = Vec::<Fp>::new();

            let mut cur = Fp::ONE;
            for _ in 1..=capacity {
                table.push(cur);
                cur.double_in_place();
            }

            table
        };

        let mut i = 0;
        let src_len = src_limbs.len();
        while i < src_len {
            let first = &src_limbs[i];
            let second = if i + 1 < src_len {
                Some(&src_limbs[i + 1])
            } else {
                None
            };

            let first_max_bits_per_limb = bits_per_limb + overhead(first.1 + Fp::ONE);
            let second_max_bits_per_limb = if let Some(second) = second {
                bits_per_limb + overhead(second.1 + Fp::ONE)
            } else {
                0
            };

            if let Some(second) = second {
                if first_max_bits_per_limb + second_max_bits_per_limb <= capacity {
                    let adjustment_factor =
                        &adjustment_factor_lookup_table[second_max_bits_per_limb];

                    dest_limbs.push(first.0 * adjustment_factor + second.0);
                    i += 2;
                } else {
                    dest_limbs.push(first.0);
                    i += 1;
                }
            } else {
                dest_limbs.push(first.0);
                i += 1;
            }
        }

        dest_limbs
    }

    /// Convert a `TargetField` element into limbs (not constraints)
    /// This is an internal function that would be reused by a number of other functions
    fn get_limbs_representations(
        elem: &Scalar,
        optimization_type: OptimizationType,
    ) -> SmallVec<[Fp; 10]> {
        Self::get_limbs_representations_from_big_integer(&elem.0, optimization_type)
    }

    /// Obtain the limbs directly from a big int
    fn get_limbs_representations_from_big_integer(
        elem: &Uint<256, 4>,
        optimization_type: OptimizationType,
    ) -> SmallVec<[Fp; 10]> {
        let (num_limbs, bits_per_limb) = get_params(
            scalar::MODULUS_BITS as usize,
            fp::MODULUS_BITS as usize,
            optimization_type,
        );

        // Push the lower limbs first
        let mut limbs: SmallVec<[Fp; 10]> = SmallVec::new();
        let mut cur = *elem;
        let cmp = Uint::<256, 4>::from(1u64) << bits_per_limb;
        for _ in 0..num_limbs {
            let cur_mod_r = cur & cmp;
            limbs.push(Fp(Uint::<384, 6>::from(cur_mod_r)));
            cur = cur >> bits_per_limb;
        }

        // then we reserve, so that the limbs are ``big limb first''
        limbs.reverse();

        limbs
    }

    /// Push elements to sponge, treated in the non-native field representations.
    pub fn push_elements_to_sponge(
        &mut self,
        src: impl IntoIterator<Item = Scalar>,
        ty: OptimizationType,
    ) {
        let mut src_limbs = Vec::<(Fp, Fp)>::new();

        for elem in src {
            let limbs = Self::get_limbs_representations(&elem, ty);
            for limb in limbs.iter() {
                src_limbs.push((*limb, Fp::ONE));
                // specifically set to one, since most gadgets in the constraint world would not have zero noise (due to the relatively weak normal form testing in `alloc`)
            }
        }

        let dest_limbs = Self::compress_elements(&src_limbs, ty);
        self.absorb_native_field_elements(&dest_limbs);
    }

    /// obtain random bits from hashchain.
    /// not guaranteed to be uniformly distributed, should only be used in certain situations.
    pub fn get_bits(&mut self, num_bits: usize) -> Vec<bool> {
        let bits_per_element = (fp::MODULUS_BITS - 1) as usize;
        let num_elements = (num_bits + bits_per_element - 1) / bits_per_element;

        let src_elements = self.squeeze_native_field_elements(num_elements);
        let mut dest_bits = Vec::<bool>::with_capacity(num_elements * bits_per_element);

        let skip = (fp::REPR_SHAVE_BITS + 1) as usize;
        for elem in src_elements.iter() {
            // discard the highest bit
            let elem_bits = elem
                .0
                .as_limbs()
                .iter()
                .flat_map(|limb| limb.view_bits::<Lsb0>())
                .map(|b| *b.deref())
                .rev()
                .collect::<Vec<_>>();
            dest_bits.extend_from_slice(&elem_bits[skip..]);
        }
        dest_bits.truncate(num_bits);

        dest_bits
    }

    /// obtain random field elements from hashchain.
    /// not guaranteed to be uniformly distributed, should only be used in certain situations.
    pub fn get_fe(
        &mut self,
        num_elements: usize,
        outputs_short_elements: bool,
    ) -> SmallVec<[Scalar; 10]> {
        let num_bits_per_nonnative = if outputs_short_elements {
            168
        } else {
            (scalar::MODULUS_BITS - 1) as usize // also omit the highest bit
        };
        let bits = self.get_bits(num_bits_per_nonnative * num_elements);

        let mut lookup_table = Vec::<Scalar>::new();
        let mut cur = Scalar::ONE;
        for _ in 0..num_bits_per_nonnative {
            lookup_table.push(cur);
            cur.double_in_place();
        }

        let dest_elements = bits
            .chunks_exact(num_bits_per_nonnative)
            .map(|per_nonnative_bits| {
                // technically, this can be done via BigInterger::from_bits; here, we use this method for consistency with the gadget counterpart
                let mut res = Scalar::ZERO;

                for (i, bit) in per_nonnative_bits.iter().rev().enumerate() {
                    if *bit {
                        res += &lookup_table[i];
                    }
                }
                res
            })
            .collect::<SmallVec<_>>();
        debug_assert_eq!(dest_elements.len(), num_elements);

        dest_elements
    }
}
