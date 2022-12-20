use anyhow::Result;
use rand::{RngCore, Rng};
use snarkd_common::Digest;
use snarkd_crypto::{coinbase_puzzle::{PuzzleConfig, CoinbasePuzzle, EpochChallenge}, PrivateKey, ComputeKey, Address, kzg10::UniversalParams};

fn generate_private_key() -> PrivateKey {
    PrivateKey::rand()
}

fn make_compute_key(key: &PrivateKey) -> ComputeKey {
    ComputeKey::from(key)
}

fn get_address(key: &ComputeKey) -> Address {
    key.to_address()
}

lazy_static::lazy_static! {
    static ref SRS: UniversalParams = {

        let max_degree = 1 << 15;
        let max_config = PuzzleConfig { degree: max_degree };
        CoinbasePuzzle::setup(max_config).unwrap()
    };
    static ref PUZZLE: CoinbasePuzzle = {
        let config = PuzzleConfig { degree: DEGREE };
        CoinbasePuzzle::trim(&*SRS, config).unwrap()
    };
}

const DEGREE: u32 = (1 << 15) - 1;

fn mine_block(address: Address, previous_block_hash: Digest, minimum_target: u64) -> Result<()> {
    let mut rng = rand::thread_rng();

    let epoch_challenge = EpochChallenge::new(rng.next_u32(), previous_block_hash.try_into()?, DEGREE)?;

    let nonce: u64 = rng.gen();

    let solution = PUZZLE
        .prove(&epoch_challenge, address, nonce, Some(minimum_target))
        .unwrap();
    let proof_target = solution.to_target().unwrap();

    PUZZLE.verify(coinbase_solution, epoch_challenge, coinbase_target, proof_target)

}