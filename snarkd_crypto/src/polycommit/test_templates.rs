use super::sonic_pc::{
    BatchLCProof, BatchProof, Evaluations, LabeledCommitment, QuerySet, Randomness, SonicKZG10,
    VerifierKey,
};
use crate::{
    bls12_377::{Field, Scalar},
    fft::DensePolynomial,
    polycommit::{
        sonic_pc::{LabeledPolynomial, LabeledPolynomialWithBasis, LinearCombination},
        PCError,
    },
    utils::PoseidonSponge,
};
use itertools::Itertools;
use rand::{
    distributions::{self, Distribution},
    Rng,
};
use rayon::prelude::*;

#[derive(Default)]
struct TestInfo {
    num_iters: usize,
    max_degree: Option<usize>,
    supported_degree: Option<usize>,
    num_polynomials: usize,
    enforce_degree_bounds: bool,
    max_num_queries: usize,
    num_equations: Option<usize>,
}

pub struct TestComponents {
    pub verification_key: VerifierKey,
    pub commitments: Vec<LabeledCommitment>,
    pub query_set: QuerySet<'static>,
    pub evaluations: Evaluations<'static>,
    pub batch_lc_proof: Option<BatchLCProof>,
    pub batch_proof: Option<BatchProof>,
    pub randomness: Vec<Randomness>,
}

pub fn bad_degree_bound_test() -> Result<(), PCError> {
    let max_degree = 100;
    let pp = SonicKZG10::setup(max_degree)?;

    (0..10).into_par_iter().for_each(|_| {
        let rng = &mut rand::thread_rng();
        let supported_degree = distributions::Uniform::from(1..=max_degree).sample(rng);
        assert!(
            max_degree >= supported_degree,
            "max_degree < supported_degree"
        );

        let mut labels = Vec::new();
        let mut polynomials = Vec::new();
        let mut degree_bounds = Vec::new();

        for i in 0..10 {
            let label = format!("Test{}", i);
            labels.push(label.clone());
            let poly = DensePolynomial::rand(supported_degree);

            let degree_bound = 1usize;
            let hiding_bound = Some(1);
            degree_bounds.push(degree_bound);

            polynomials.push(LabeledPolynomial::new(
                label,
                poly,
                Some(degree_bound),
                hiding_bound,
            ))
        }

        println!("supported degree: {:?}", supported_degree);
        let (ck, vk) = SonicKZG10::trim(
            &pp,
            supported_degree,
            None,
            supported_degree,
            Some(degree_bounds.as_slice()),
        )
        .unwrap();
        println!("Trimmed");

        let (comms, rands) = SonicKZG10::commit(&ck, polynomials.iter().map(Into::into)).unwrap();

        let mut query_set = QuerySet::new();
        let mut values = Evaluations::new();
        let point = Scalar::rand();
        for (i, label) in labels.iter().enumerate() {
            query_set.insert((label.clone(), ("rand".into(), point)));
            let value = polynomials[i].evaluate(point);
            values.insert((label.clone(), point), value);
        }
        println!("Generated query set");

        let mut sponge_for_open = PoseidonSponge::default();
        let proof = SonicKZG10::batch_open(
            &ck,
            &polynomials,
            &comms,
            &query_set,
            &rands,
            &mut sponge_for_open,
        )
        .unwrap();
        let mut sponge_for_check = PoseidonSponge::default();
        let result = SonicKZG10::batch_check(
            &vk,
            &comms,
            &query_set,
            &values,
            &proof,
            &mut sponge_for_check,
        )
        .unwrap();
        assert!(result, "proof was incorrect, Query set: {:#?}", query_set);
    });
    Ok(())
}

pub fn lagrange_test_template() -> Result<(), PCError> {
    let num_iters = 10usize;
    let max_degree = 256usize;
    let supported_degree = 127usize;
    let eval_size = 128usize;
    let num_polynomials = 1usize;
    let max_num_queries = 2usize;

    let pp = SonicKZG10::setup(max_degree)?;

    (0..num_iters).into_par_iter().for_each(|_| {
        let rng = &mut rand::thread_rng();
        assert!(
            max_degree >= supported_degree,
            "max_degree < supported_degree"
        );
        let mut polynomials = Vec::new();
        let mut lagrange_polynomials = Vec::new();
        let mut supported_lagrange_sizes = Vec::new();
        let degree_bounds = None;

        let mut labels = Vec::new();
        println!("Sampled supported degree");

        // Generate polynomials
        let num_points_in_query_set = distributions::Uniform::from(1..=max_num_queries).sample(rng);
        for i in 0..num_polynomials {
            let label = format!("Test{}", i);
            labels.push(label.clone());
            let eval_size: usize = distributions::Uniform::from(1..eval_size)
                .sample(rng)
                .next_power_of_two();
            let mut evals = vec![Scalar::ZERO; eval_size];
            for e in &mut evals {
                *e = Scalar::rand();
            }
            let domain = crate::fft::EvaluationDomain::new(evals.len()).unwrap();
            let evals = crate::fft::Evaluations::from_vec_and_domain(evals, domain);
            let poly = evals.interpolate_by_ref();
            supported_lagrange_sizes.push(domain.size());
            assert_eq!(poly.evaluate_over_domain_by_ref(domain), evals);

            let degree_bound = None;

            let hiding_bound = Some(1);
            polynomials.push(LabeledPolynomial::new(
                label.clone(),
                poly,
                degree_bound,
                hiding_bound,
            ));
            lagrange_polynomials.push(LabeledPolynomialWithBasis::new_lagrange_basis(
                label,
                evals,
                hiding_bound,
            ))
        }
        let supported_hiding_bound = polynomials
            .iter()
            .map(|p| p.hiding_bound().unwrap_or(0))
            .max()
            .unwrap_or(0);
        println!("supported degree: {:?}", supported_degree);
        println!("supported hiding bound: {:?}", supported_hiding_bound);
        println!("num_points_in_query_set: {:?}", num_points_in_query_set);
        let (ck, vk) = SonicKZG10::trim(
            &pp,
            supported_degree,
            supported_lagrange_sizes,
            supported_hiding_bound,
            degree_bounds,
        )
        .unwrap();
        println!("Trimmed");

        let (comms, rands) = SonicKZG10::commit(&ck, lagrange_polynomials).unwrap();

        // Construct query set
        let mut query_set = QuerySet::new();
        let mut values = Evaluations::new();
        // let mut point = E::Fr::one();
        for point_id in 0..num_points_in_query_set {
            let point = Scalar::rand();
            for (polynomial, label) in polynomials.iter().zip_eq(labels.iter()) {
                query_set.insert((label.clone(), (format!("rand_{}", point_id), point)));
                let value = polynomial.evaluate(point);
                values.insert((label.clone(), point), value);
            }
        }
        println!("Generated query set");

        let mut sponge_for_open = PoseidonSponge::default();
        let proof = SonicKZG10::batch_open(
            &ck,
            &polynomials,
            &comms,
            &query_set,
            &rands,
            &mut sponge_for_open,
        )
        .unwrap();
        let mut sponge_for_check = PoseidonSponge::default();
        let result = SonicKZG10::batch_check(
            &vk,
            &comms,
            &query_set,
            &values,
            &proof,
            &mut sponge_for_check,
        )
        .unwrap();
        if !result {
            println!(
                "Failed with {} polynomials, num_points_in_query_set: {:?}",
                num_polynomials, num_points_in_query_set
            );
            println!("Degree of polynomials:");
            for poly in polynomials {
                println!("Degree: {:?}", poly.degree());
            }
        }
        assert!(result, "proof was incorrect, Query set: {:#?}", query_set);
    });
    Ok(())
}

fn test_template(info: TestInfo) -> Result<(), PCError>
where
{
    let TestInfo {
        num_iters,
        max_degree,
        supported_degree,
        num_polynomials,
        enforce_degree_bounds,
        max_num_queries,
        ..
    } = info;

    let rng = &mut rand::thread_rng();
    let max_degree = max_degree.unwrap_or_else(|| distributions::Uniform::from(8..=64).sample(rng));
    let pp = SonicKZG10::setup(max_degree)?;
    let supported_degree_bounds = pp.supported_degree_bounds();

    (0..num_iters).into_par_iter().for_each(|_| {
        let rng = &mut rand::thread_rng();
        let supported_degree = supported_degree
            .unwrap_or_else(|| distributions::Uniform::from(4..=max_degree).sample(rng));
        assert!(
            max_degree >= supported_degree,
            "max_degree < supported_degree"
        );
        let mut polynomials = Vec::new();
        let mut degree_bounds = if enforce_degree_bounds {
            Some(Vec::new())
        } else {
            None
        };

        let mut labels = Vec::new();
        println!("Sampled supported degree");

        // Generate polynomials
        let num_points_in_query_set = distributions::Uniform::from(1..=max_num_queries).sample(rng);
        for i in 0..num_polynomials {
            let label = format!("Test{}", i);
            labels.push(label.clone());
            let degree = distributions::Uniform::from(1..=supported_degree).sample(rng);
            let poly = DensePolynomial::rand(degree);

            let supported_degree_bounds_after_trimmed = supported_degree_bounds
                .iter()
                .copied()
                .filter(|x| *x >= degree && *x < supported_degree)
                .collect::<Vec<usize>>();

            let degree_bound = if let Some(degree_bounds) = &mut degree_bounds {
                if !supported_degree_bounds_after_trimmed.is_empty() && rng.gen() {
                    let range = distributions::Uniform::from(
                        0..supported_degree_bounds_after_trimmed.len(),
                    );
                    let idx = range.sample(rng);

                    let degree_bound = supported_degree_bounds_after_trimmed[idx];
                    degree_bounds.push(degree_bound);
                    Some(degree_bound)
                } else {
                    None
                }
            } else {
                None
            };

            let hiding_bound = if num_points_in_query_set >= degree {
                Some(degree)
            } else {
                Some(num_points_in_query_set)
            };
            println!("Hiding bound: {:?}", hiding_bound);

            polynomials.push(LabeledPolynomial::new(
                label,
                poly,
                degree_bound,
                hiding_bound,
            ))
        }
        let supported_hiding_bound = polynomials
            .iter()
            .map(|p| p.hiding_bound().unwrap_or(0))
            .max()
            .unwrap_or(0);
        println!("supported degree: {:?}", supported_degree);
        println!("supported hiding bound: {:?}", supported_hiding_bound);
        println!("num_points_in_query_set: {:?}", num_points_in_query_set);
        let (ck, vk) = SonicKZG10::trim(
            &pp,
            supported_degree,
            None,
            supported_hiding_bound,
            degree_bounds.as_deref(),
        )
        .unwrap();
        println!("Trimmed");

        let (comms, rands) = SonicKZG10::commit(&ck, polynomials.iter().map(Into::into)).unwrap();

        // Construct query set
        let mut query_set = QuerySet::new();
        let mut values = Evaluations::new();
        // let mut point = E::Fr::one();
        for point_id in 0..num_points_in_query_set {
            let point = Scalar::rand();
            for (polynomial, label) in polynomials.iter().zip_eq(labels.iter()) {
                query_set.insert((label.clone(), (format!("rand_{}", point_id), point)));
                let value = polynomial.evaluate(point);
                values.insert((label.clone(), point), value);
            }
        }
        println!("Generated query set");

        let mut sponge_for_open = PoseidonSponge::default();
        let proof = SonicKZG10::batch_open(
            &ck,
            &polynomials,
            &comms,
            &query_set,
            &rands,
            &mut sponge_for_open,
        )
        .unwrap();
        let mut sponge_for_check = PoseidonSponge::default();
        let result = SonicKZG10::batch_check(
            &vk,
            &comms,
            &query_set,
            &values,
            &proof,
            &mut sponge_for_check,
        )
        .unwrap();
        if !result {
            println!(
                "Failed with {} polynomials, num_points_in_query_set: {:?}",
                num_polynomials, num_points_in_query_set
            );
            println!("Degree of polynomials:");
            for poly in polynomials {
                println!("Degree: {:?}", poly.degree());
            }
        }
        assert!(result, "proof was incorrect, Query set: {:#?}", query_set);
    });
    Ok(())
}

fn equation_test_template(info: TestInfo) -> Result<(), PCError> {
    let TestInfo {
        num_iters,
        max_degree,
        supported_degree,
        num_polynomials,
        enforce_degree_bounds,
        max_num_queries,
        num_equations,
    } = info;

    let rng = &mut rand::thread_rng();
    let max_degree = max_degree.unwrap_or_else(|| distributions::Uniform::from(8..=64).sample(rng));
    let pp = SonicKZG10::setup(max_degree)?;
    let supported_degree_bounds = pp.supported_degree_bounds();

    (0..num_iters).into_par_iter().for_each(|_| {
        let rng = &mut rand::thread_rng();
        let supported_degree = supported_degree
            .unwrap_or_else(|| distributions::Uniform::from(4..=max_degree).sample(rng));
        assert!(
            max_degree >= supported_degree,
            "max_degree < supported_degree"
        );
        let mut polynomials = Vec::new();
        let mut degree_bounds = if enforce_degree_bounds {
            Some(Vec::new())
        } else {
            None
        };

        let mut labels = Vec::new();
        println!("Sampled supported degree");

        // Generate polynomials
        let num_points_in_query_set = distributions::Uniform::from(1..=max_num_queries).sample(rng);
        for i in 0..num_polynomials {
            let label = format!("Test{}", i);
            labels.push(label.clone());
            let degree = distributions::Uniform::from(1..=supported_degree).sample(rng);
            let poly = DensePolynomial::rand(degree);

            let supported_degree_bounds_after_trimmed = supported_degree_bounds
                .iter()
                .copied()
                .filter(|x| *x >= degree && *x < supported_degree)
                .collect::<Vec<usize>>();

            let degree_bound = if let Some(degree_bounds) = &mut degree_bounds {
                if !supported_degree_bounds_after_trimmed.is_empty() && rng.gen() {
                    let range = distributions::Uniform::from(
                        0..supported_degree_bounds_after_trimmed.len(),
                    );
                    let idx = range.sample(rng);

                    let degree_bound = supported_degree_bounds_after_trimmed[idx];
                    degree_bounds.push(degree_bound);
                    Some(degree_bound)
                } else {
                    None
                }
            } else {
                None
            };

            let hiding_bound = if num_points_in_query_set >= degree {
                Some(degree)
            } else {
                Some(num_points_in_query_set)
            };
            println!("Hiding bound: {:?}", hiding_bound);

            polynomials.push(LabeledPolynomial::new(
                label,
                poly,
                degree_bound,
                hiding_bound,
            ))
        }
        let supported_hiding_bound = polynomials
            .iter()
            .map(|p| p.hiding_bound().unwrap_or(0))
            .max()
            .unwrap_or(0);
        println!("supported degree: {:?}", supported_degree);
        println!("supported hiding bound: {:?}", supported_hiding_bound);
        println!("num_points_in_query_set: {:?}", num_points_in_query_set);
        println!("{:?}", degree_bounds);
        println!("{}", num_polynomials);
        println!("{}", enforce_degree_bounds);

        let (ck, vk) = SonicKZG10::trim(
            &pp,
            supported_degree,
            None,
            supported_hiding_bound,
            degree_bounds.as_deref(),
        )
        .unwrap();
        println!("Trimmed");

        let (comms, rands) = SonicKZG10::commit(&ck, polynomials.iter().map(Into::into)).unwrap();

        // Let's construct our equations
        let mut linear_combinations = Vec::new();
        let mut query_set = QuerySet::new();
        let mut values = Evaluations::new();
        for i in 0..num_points_in_query_set {
            let point = Scalar::rand();
            for j in 0..num_equations.unwrap() {
                let label = format!("query {} eqn {}", i, j);
                let mut lc = LinearCombination::empty(label.clone());

                let mut value = Scalar::ZERO;
                let should_have_degree_bounds: bool = rng.gen();
                for (k, label) in labels.iter().enumerate() {
                    if should_have_degree_bounds {
                        value += &polynomials[k].evaluate(point);
                        lc.add(Scalar::ONE, label.clone());
                        break;
                    } else {
                        let poly = &polynomials[k];
                        if poly.degree_bound().is_some() {
                            continue;
                        } else {
                            assert!(poly.degree_bound().is_none());
                            let coeff = Scalar::rand();
                            value += &(coeff * poly.evaluate(point));
                            lc.add(coeff, label.clone());
                        }
                    }
                }
                values.insert((label.clone(), point), value);
                if !lc.is_empty() {
                    linear_combinations.push(lc);
                    // Insert query
                    query_set.insert((label.clone(), (format!("rand_{}", i), point)));
                }
            }
        }
        if !linear_combinations.is_empty() {
            println!("Generated query set");
            println!("Linear combinations: {:?}", linear_combinations);

            let mut sponge_for_open = PoseidonSponge::default();
            let proof = SonicKZG10::open_combinations(
                &ck,
                &linear_combinations,
                &polynomials,
                &comms,
                &query_set,
                &rands,
                &mut sponge_for_open,
            )
            .unwrap();
            println!("Generated proof");
            let mut sponge_for_check = PoseidonSponge::default();
            let result = SonicKZG10::check_combinations(
                &vk,
                &linear_combinations,
                &comms,
                &query_set,
                &values,
                &proof,
                &mut sponge_for_check,
            )
            .unwrap();
            if !result {
                println!(
                    "Failed with {} polynomials, num_points_in_query_set: {:?}",
                    num_polynomials, num_points_in_query_set
                );
                println!("Degree of polynomials:");
                for poly in polynomials {
                    println!("Degree: {:?}", poly.degree());
                }
            }
            assert!(
                result,
                "proof was incorrect, equations: {:#?}",
                linear_combinations
            );
        }
    });
    Ok(())
}

pub fn single_poly_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 1,
        enforce_degree_bounds: false,
        max_num_queries: 1,
        ..Default::default()
    };
    test_template(info)
}

pub fn linear_poly_degree_bound_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: Some(2),
        supported_degree: Some(1),
        num_polynomials: 1,
        enforce_degree_bounds: true,
        max_num_queries: 1,
        ..Default::default()
    };
    test_template(info)
}

pub fn single_poly_degree_bound_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 1,
        enforce_degree_bounds: true,
        max_num_queries: 1,
        ..Default::default()
    };
    test_template(info)
}

pub fn quadratic_poly_degree_bound_multiple_queries_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: Some(3),
        supported_degree: Some(2),
        num_polynomials: 1,
        enforce_degree_bounds: true,
        max_num_queries: 2,
        ..Default::default()
    };
    test_template(info)
}

pub fn single_poly_degree_bound_multiple_queries_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 1,
        enforce_degree_bounds: true,
        max_num_queries: 2,
        ..Default::default()
    };
    test_template(info)
}

pub fn two_polys_degree_bound_single_query_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 2,
        enforce_degree_bounds: true,
        max_num_queries: 1,
        ..Default::default()
    };
    test_template(info)
}

pub fn full_end_to_end_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 10,
        enforce_degree_bounds: true,
        max_num_queries: 5,
        ..Default::default()
    };
    test_template(info)
}

pub fn full_end_to_end_equation_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 10,
        enforce_degree_bounds: true,
        max_num_queries: 5,
        num_equations: Some(10),
    };
    equation_test_template(info)
}

pub fn single_equation_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 1,
        enforce_degree_bounds: false,
        max_num_queries: 1,
        num_equations: Some(1),
    };
    equation_test_template(info)
}

pub fn two_equation_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 2,
        enforce_degree_bounds: false,
        max_num_queries: 1,
        num_equations: Some(2),
    };
    equation_test_template(info)
}

pub fn two_equation_degree_bound_test() -> Result<(), PCError> {
    let info = TestInfo {
        num_iters: 10,
        max_degree: None,
        supported_degree: None,
        num_polynomials: 2,
        enforce_degree_bounds: true,
        max_num_queries: 1,
        num_equations: Some(2),
    };
    equation_test_template(info)
}
