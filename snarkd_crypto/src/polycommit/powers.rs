use anyhow::{anyhow, bail, ensure, Result};
use itertools::Itertools;
use std::{collections::BTreeMap, io::BufReader};

const DEGREE_15: usize = 1 << 15;
const DEGREE_16: usize = 1 << 16;
const DEGREE_17: usize = 1 << 17;
const DEGREE_18: usize = 1 << 18;
const DEGREE_19: usize = 1 << 19;
const DEGREE_20: usize = 1 << 20;
const DEGREE_21: usize = 1 << 21;
const DEGREE_22: usize = 1 << 22;
const DEGREE_23: usize = 1 << 23;
const DEGREE_24: usize = 1 << 24;
const DEGREE_25: usize = 1 << 25;
const DEGREE_26: usize = 1 << 26;
const DEGREE_27: usize = 1 << 27;
const DEGREE_28: usize = 1 << 28;

/// The maximum degree supported by the SRS.
const MAXIMUM_DEGREE: usize = DEGREE_28;

/// Number of powers contained in `UNIVERSAL_SRS_GAMMA`.
const NUM_UNIVERSAL_SRS_GAMMA: usize = 84;

/// A vector of powers of beta G.
#[derive(Debug)]
pub struct PowersOfG {
    /// A boolean indicator if the powers were from a setup.
    is_setup: bool,
    /// The number of group elements in `powers_of_beta_g`.
    current_degree: usize,
    /// Group elements of form `[G, \beta * G, \beta^2 * G, ..., \beta^{d} G]`.
    powers_of_beta_g: Vec<G1Affine>,
    /// Group elements of form `{ \beta^i \gamma G }`, where `i` is from 0 to `degree`, used for hiding.
    powers_of_beta_times_gamma_g: BTreeMap<usize, G1Affine>,
}

impl PowersOfG {
    /// Initializes a new instance of the powers.
    pub fn setup(
        powers_of_beta_g: Vec<G1Affine>,
        powers_of_beta_times_gamma_g: BTreeMap<usize, G1Affine>,
    ) -> Result<Self> {
        // Initialize the powers.
        let powers = Self {
            is_setup: true,
            current_degree: powers_of_beta_g.len(),
            powers_of_beta_g,
            powers_of_beta_times_gamma_g,
        };
        // Return the powers.
        Ok(powers)
    }

    /// Initializes an existing instance of the powers.
    pub fn load() -> Result<Self> {
        unimplemented!()
    }

    /// Returns the power of beta times G specified by `target_power`.
    pub fn power_of_beta_g(&mut self, target_power: usize) -> Result<G1Affine> {
        // Ensure the powers exist, and download the missing powers if necessary.
        if target_power >= self.current_degree {
            self.download_up_to(target_power)?;
        }
        // Return the power.
        self.powers_of_beta_g
            .get(target_power)
            .copied()
            .ok_or_else(|| anyhow!("Failed to get power of beta G"))
    }

    /// Slices the underlying file to return a vector of affine elements between `lower` and `upper`.
    pub fn powers_of_beta_g(&mut self, lower: usize, upper: usize) -> Result<Vec<G1Affine>> {
        // Ensure the lower power is less than the upper power.
        assert!(lower < upper, "Lower power must be less than upper power");
        // Ensure the powers exist, and download the missing powers if necessary.
        if upper >= self.current_degree {
            self.download_up_to(upper)?;
        }
        // Return the powers.
        Ok(self.powers_of_beta_g[lower..upper].to_vec())
    }

    /// Returns the powers of beta * gamma G.
    pub fn powers_times_gamma_g(&self) -> &BTreeMap<usize, G1Affine> {
        &self.powers_of_beta_times_gamma_g
    }

    /// Return the number of current powers of G.
    pub fn degree(&self) -> usize {
        self.current_degree
    }

    /// This method downloads the universal SRS powers up to the `next_power_of_two(target_degree)`,
    /// and updates `Self` in place with the new powers.
    pub fn download_up_to(&mut self, target_degree: usize) -> Result<()> {
        // TODO: we need to revamp parameters downloads
        unimplemented!()
    }
}

impl PowersOfG {
    fn regenerate_powers_of_beta_times_gamma_g(
        current_degree: usize,
    ) -> Result<BTreeMap<usize, G1Affine>> {
        unimplemented!()
    }
}
