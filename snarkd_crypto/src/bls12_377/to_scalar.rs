use crate::bls12_377::{Field, Scalar};
use anyhow::{anyhow, Result};

pub trait ToScalar {
    fn to_scalar(&self) -> Result<Vec<Scalar>>;
}

impl ToScalar for bool {
    fn to_scalar(&self) -> Result<Vec<Scalar>> {
        if *self {
            Ok(vec![Scalar::ONE])
        } else {
            Ok(vec![Scalar::ZERO])
        }
    }
}

impl ToScalar for [Scalar] {
    fn to_scalar(&self) -> Result<Vec<Scalar>> {
        Ok(self.to_vec())
    }
}
