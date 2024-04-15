use std::ops::{Mul, Sub};
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::config::{GroupConfig, GroupParameters};

/// Represents a Zero-Knowledge Proof (ZKP) using the Chaum-Pedersen protocol.
///
/// The Chaum-Pedersen protocol is used to demonstrate that the logarithm of `alpha` to the base `g`
/// is the same as the logarithm of `beta` to the base `h` without revealing the actual logarithm values.
pub struct Prover {
    x: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
    pub y1: BigUint,
    pub y2: BigUint,
    pub p: BigUint,
    pub g: BigUint,
    pub q: BigUint,
    rng: StdRng,
}

pub struct Proof {
    pub r1: BigUint,
    pub r2: BigUint,
    pub s: BigUint,
    pub c: BigUint
}

impl Prover {

    /// Creates a new instance of the Chaum-Pedersen ZKP protocol using the specified group configuration.
    pub fn new(x: BigUint, config: GroupConfig) -> Self {
        let mut rng = StdRng::from_entropy();
        let GroupParameters { p, g, q} = config.parameters();
        let alpha = g.modpow(&x, &p);
        let beta = g.modpow(&rng.gen_biguint_below(&q), &p);
        let y1 = alpha.modpow(&x, &p);
        let y2 = beta.modpow(&x, &p);

        Self {
            x, alpha, beta, y1, y2, p, g, q, rng
        }
    }

    pub fn generate_proof(&mut self, c: &BigUint) -> Proof {
        let k = self.rng.gen_biguint_below(&self.q);

        Proof {
            r1: self.alpha.modpow(&k, &self.p),
            r2: self.beta.modpow(&k, &self.p),
            s: k.sub(&c.mul(&self.x)).modpow(&1.to_biguint().unwrap(), &self.q),
            c: c.clone()
        }
    }
}
