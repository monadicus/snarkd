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

lazy_static::lazy_static! {
    static ref UNIVERSAL_SRS_15: Vec<u8> = Degree15::load_bytes().expect("Failed to load universal SRS of degree 15");
    static ref UNIVERSAL_SRS_GAMMA: Vec<u8> = Gamma::load_bytes().expect("Failed to load universal SRS gamma powers");
}

/// A vector of powers of beta G.
#[derive(Debug)]
pub struct PowersOfG<E: PairingEngine> {
    /// A boolean indicator if the powers were from a setup.
    is_setup: bool,
    /// The number of group elements in `powers_of_beta_g`.
    current_degree: usize,
    /// Group elements of form `[G, \beta * G, \beta^2 * G, ..., \beta^{d} G]`.
    powers_of_beta_g: Vec<G1Affine>,
    /// Group elements of form `{ \beta^i \gamma G }`, where `i` is from 0 to `degree`, used for hiding.
    powers_of_beta_times_gamma_g: BTreeMap<usize, G1Affine>,
}

impl<E: PairingEngine> PowersOfG<E> {
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
        // Initialize a `BufReader`.
        let mut reader = BufReader::new(&UNIVERSAL_SRS_15[..]);
        // Deserialize the group elements.
        let powers_of_beta_g = (0..DEGREE_15)
            .map(|_| G1Affine::read_le(&mut reader))
            // .map(|_| G1Affine::deserialize_with_mode(&mut reader, Compress::No, Validate::No))
            .collect::<Result<Vec<_>, _>>()?;
        // Ensure the number of elements is correct.
        assert!(
            powers_of_beta_g.len() == DEGREE_15,
            "Incorrect number of powers in the recovered SRS"
        );

        // Initialize the powers.
        let powers = Self {
            is_setup: false,
            current_degree: DEGREE_15,
            powers_of_beta_g,
            powers_of_beta_times_gamma_g: Self::regenerate_powers_of_beta_times_gamma_g(DEGREE_15)?,
        };
        // Return the powers.
        Ok(powers)
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

impl<E: PairingEngine> PowersOfG<E> {
    fn regenerate_powers_of_beta_times_gamma_g(
        current_degree: usize,
    ) -> Result<BTreeMap<usize, G1Affine>> {
        let mut alpha_powers_g1 = vec![];
        let mut reader = BufReader::new(UNIVERSAL_SRS_GAMMA.as_slice());
        for _ in 0..NUM_UNIVERSAL_SRS_GAMMA {
            alpha_powers_g1.push(G1Affine::read_le(&mut reader)?);
        }

        let mut alpha_tau_powers_g1 = BTreeMap::new();
        for (i, power) in alpha_powers_g1.iter().enumerate().take(3) {
            alpha_tau_powers_g1.insert(i, *power);
        }
        alpha_powers_g1[3..]
            .iter()
            .chunks(3)
            .into_iter()
            .enumerate()
            .for_each(|(i, powers)| {
                // Avoid underflows and just stop populating the map if we're going to.
                if current_degree - 1 > (1 << i) {
                    let powers = powers.into_iter().collect::<Vec<_>>();
                    alpha_tau_powers_g1.insert(current_degree - 1 - (1 << i) + 2, *powers[0]);
                    alpha_tau_powers_g1.insert(current_degree - 1 - (1 << i) + 3, *powers[1]);
                    alpha_tau_powers_g1.insert(current_degree - 1 - (1 << i) + 4, *powers[2]);
                }
            });
        Ok(alpha_tau_powers_g1)
    }
}
