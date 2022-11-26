use super::*;
use crate::keys::{Address, ComputeKey, PrivateKey};

use rand::{Rng, RngCore};

#[test]
fn test_coinbase_puzzle() {
    let mut rng = rand::thread_rng();

    let max_degree = 1 << 15;
    let max_config = PuzzleConfig { degree: max_degree };
    let srs = CoinbasePuzzle::setup(max_config).unwrap();

    for log_degree in 5..10 {
        let degree = (1 << log_degree) - 1;
        let config = PuzzleConfig { degree };
        let puzzle = CoinbasePuzzle::trim(&srs, config).unwrap();
        let epoch_challenge =
            EpochChallenge::new(rng.next_u32(), Default::default(), degree).unwrap();

        for batch_size in 1..10 {
            let solutions = (0..batch_size)
                .map(|_| {
                    let private_key = PrivateKey::rand();
                    let address: Address = ComputeKey::from(&private_key).to_address();
                    let nonce: u64 = rng.gen();
                    puzzle
                        .prove(&epoch_challenge, address, nonce, None)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let full_solution = puzzle
                .accumulate_unchecked(&epoch_challenge, &solutions)
                .unwrap();
            assert!(puzzle
                .verify(&full_solution, &epoch_challenge, 0u64, 0u64)
                .unwrap());

            let bad_epoch_challenge =
                EpochChallenge::new(rng.next_u32(), Default::default(), degree).unwrap();
            assert!(!puzzle
                .verify(&full_solution, &bad_epoch_challenge, 0u64, 0u64)
                .unwrap());
        }
    }
}

#[test]
fn test_prover_solution_minimum_target() {
    let mut rng = rand::thread_rng();

    let max_degree = 1 << 15;
    let max_config = PuzzleConfig { degree: max_degree };
    let srs = CoinbasePuzzle::setup(max_config).unwrap();

    for log_degree in 5..10 {
        let degree = (1 << log_degree) - 1;
        let config = PuzzleConfig { degree };
        let puzzle = CoinbasePuzzle::trim(&srs, config).unwrap();
        let epoch_challenge =
            EpochChallenge::new(rng.next_u32(), Default::default(), degree).unwrap();

        let private_key = PrivateKey::rand();
        let address: Address = ComputeKey::from(&private_key).to_address();
        let nonce: u64 = rng.gen();

        let solution = puzzle
            .prove(&epoch_challenge, address, nonce, None)
            .unwrap();
        let proof_target = solution.to_target().unwrap();

        // Assert that the operation will pass if the minimum target is low enough.
        puzzle
            .prove(
                &epoch_challenge,
                address,
                nonce,
                Some(proof_target.saturating_sub(1)),
            )
            .unwrap();

        // Assert that the operation will fail if the minimum target is too high.
        assert!(puzzle
            .prove(
                &epoch_challenge,
                address,
                nonce,
                Some(proof_target.saturating_add(1))
            )
            .is_err());
    }
}

#[test]
fn test_edge_case_for_degree() {
    let mut rng = rand::thread_rng();

    // Generate srs.
    let max_degree = 1 << 15;
    let max_config = PuzzleConfig { degree: max_degree };
    let srs = CoinbasePuzzle::setup(max_config).unwrap();

    // Generate PK and VK.
    let degree = (1 << 13) - 1; // IF YOU ADD `- 1` THIS WILL PASS
    let puzzle = CoinbasePuzzle::trim(&srs, PuzzleConfig { degree }).unwrap();

    // Generate proof inputs
    let private_key = PrivateKey::rand();
    let address: Address = ComputeKey::from(&private_key).to_address();
    let epoch_challenge = EpochChallenge::new(rng.gen(), Default::default(), degree).unwrap();

    // Generate a prover solution.
    let prover_solution = puzzle
        .prove(&epoch_challenge, address, rng.gen(), None)
        .unwrap();
    let coinbase_solution = puzzle
        .accumulate_unchecked(&epoch_challenge, &[prover_solution])
        .unwrap();
    assert!(puzzle
        .verify(&coinbase_solution, &epoch_challenge, 0u64, 0u64)
        .unwrap());
}
