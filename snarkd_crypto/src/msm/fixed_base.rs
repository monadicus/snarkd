use crate::bls12_377::{scalar, Projective, Scalar};
use bitvec::prelude::*;
use core::ops::Deref;
use rayon::prelude::*;

pub struct FixedBase;

impl FixedBase {
    pub fn get_mul_window_size(num_scalars: usize) -> usize {
        match num_scalars < 32 {
            true => 3,
            false => super::ln_without_floats(num_scalars),
        }
    }

    pub fn get_window_table<P: Projective>(scalar_size: usize, window: usize, g: P) -> Vec<Vec<P>> {
        let in_window = 1 << window;
        let outerc = (scalar_size + window - 1) / window;
        let last_in_window = 1 << (scalar_size - (outerc - 1) * window);

        let mut multiples_of_g = vec![vec![P::ZERO; in_window]; outerc];

        let mut g_outer = g;
        let mut g_outers = Vec::with_capacity(outerc);
        for _ in 0..outerc {
            g_outers.push(g_outer);
            for _ in 0..window {
                g_outer.double_in_place();
            }
        }

        cfg_iter_mut!(multiples_of_g)
            .enumerate()
            .take(outerc)
            .zip(g_outers)
            .for_each(|((outer, multiples_of_g), g_outer)| {
                let cur_in_window = if outer == outerc - 1 {
                    last_in_window
                } else {
                    in_window
                };

                let mut g_inner = P::ZERO;
                for inner in multiples_of_g.iter_mut().take(cur_in_window) {
                    *inner = g_inner;
                    g_inner += &g_outer;
                }
            });
        multiples_of_g
    }

    pub fn windowed_mul<P: Projective>(
        outerc: usize,
        window: usize,
        multiples_of_g: &[Vec<P>],
        scalar: &Scalar,
    ) -> P {
        let scalar_val = scalar
            .0
            .as_limbs()
            .iter()
            .flat_map(|limb| limb.view_bits::<Lsb0>())
            .map(|bit| *bit.deref())
            .collect::<Vec<_>>();

        cfg_into_iter!(0..outerc)
            .map(|outer| {
                let mut inner = 0usize;
                for i in 0..window {
                    if outer * window + i < (scalar::MODULUS_BITS as usize)
                        && scalar_val[outer * window + i]
                    {
                        inner |= 1 << i;
                    }
                }
                multiples_of_g[outer][inner]
            })
            .sum::<P>()
            + multiples_of_g[0][0]
    }

    pub fn msm<P: Projective>(
        scalar_size: usize,
        window: usize,
        table: &[Vec<P>],
        v: &[Scalar],
    ) -> Vec<P> {
        let outerc = (scalar_size + window - 1) / window;
        assert!(outerc <= table.len());

        cfg_iter!(v)
            .map(|e| Self::windowed_mul::<P>(outerc, window, table, e))
            .collect::<Vec<_>>()
    }
}
