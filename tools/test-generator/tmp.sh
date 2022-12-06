set -e

rm -rf ../../tests/crypto/bls12_377/field/fp/generated
mkdir ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs neg 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs add 100 fp fp fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sub 100 fp fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs mul 100 fp fp fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs inversion 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs double 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs square 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs expansion 100 fp fp fp fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sqrt 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs pow 100 fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs sum_of_products 100 vec-fp vec-fp -o ../../tests/crypto/bls12_377/field/fp/generated
cargo run --release -- FpNs math_properties 100 fp fp -o ../../tests/crypto/bls12_377/field/fp/generated

rm -rf ../../tests/crypto/bls12_377/field/fp2/generated
mkdir ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns neg 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns add 100 fp2 fp2 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sub 100 fp2 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns mul 100 fp2 fp2 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns inversion 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns double 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns square 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns expansion 100 fp2 fp2 fp2 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns frobenius 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sqrt 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns pow 100 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns sum_of_products 100 vec-fp2 vec-fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated
cargo run --release -- Fp2Ns math_properties 100 fp2 fp2 -o ../../tests/crypto/bls12_377/field/fp2/generated

rm -rf ../../tests/crypto/bls12_377/field/fp6/generated
mkdir ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns neg 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns add 100 fp6 fp6 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns sub 100 fp6 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul 100 fp6 fp6 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns inversion 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns double 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns square 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns expansion 100 fp6 fp6 fp6 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns frobenius 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns pow 100 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns sum_of_products 100 vec-fp6 vec-fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns math_properties 100 fp6 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul_by_1 100 fp2 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated
cargo run --release -- Fp6Ns mul_by_01 100 fp2 fp2 fp6 -o ../../tests/crypto/bls12_377/field/fp6/generated

rm -rf ../../tests/crypto/bls12_377/field/fp12/generated
mkdir ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns neg 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns add 100 fp12 fp12 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns sub 100 fp12 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul 100 fp12 fp12 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns inversion 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns double 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns square 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns expansion 100 fp12 fp12 fp12 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns frobenius 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns pow 100 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns sum_of_products 100 vec-fp12 vec-fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns math_properties 100 fp12 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul_by_014 100 fp2 fp2 fp2 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated
cargo run --release -- Fp12Ns mul_by_034 100 fp2 fp2 fp2 fp12 -o ../../tests/crypto/bls12_377/field/fp12/generated

rm -rf ../../tests/crypto/bls12_377/field/scalar/generated
mkdir ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs neg 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs add 100 scalar scalar scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs sub 100 scalar scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs mul 100 scalar scalar scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs inversion 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs double 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs square 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs expansion 100 scalar scalar scalar scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs sqrt 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs pow 100 scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs sum_of_products 100 vec-scalar vec-scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
cargo run --release -- ScalarNs math_properties 100 scalar scalar -o ../../tests/crypto/bls12_377/field/scalar/generated
