mkdir ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs neg 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs add 100 three-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sub 100 two-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs mul 100 three-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs inversion 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs double 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs square 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs expansion 100 four-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sqrt 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs pow 100 one-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sum_of_products 100 two-fp-lists -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs math_properties 100 two-fp -o ../../tests/crypto/bls12_377/field/fp/generated

mkdir ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns neg 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns add 100 three-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sub 100 two-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns mul 100 three-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns inversion 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns double 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns square 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns expansion 100 four-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns frobenius 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sqrt 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns pow 100 one-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sum_of_products 100 two-fp2-lists -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns math_properties 100 two-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated

mkdir ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns neg 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns add 100 three-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns sub 100 two-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul 100 three-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns inversion 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns double 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns square 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns expansion 100 four-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns frobenius 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns pow 100 one-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns sum_of_products 100 two-fp6-lists -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns math_properties 100 two-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul_by_1 100 fp2-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul_by_01 100 fp2-fp2-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated

mkdir ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns neg 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns add 100 three-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns sub 100 two-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul 100 three-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns inversion 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns double 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns square 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns expansion 100 four-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns frobenius 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns pow 100 one-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns sum_of_products 100 two-fp12-lists -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns math_properties 100 two-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul_by_014 100 fp2-fp2-fp2-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul_by_034 100 fp2-fp2-fp2-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated