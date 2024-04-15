use std::ops::{Add, Mul};
use num_bigint::{BigUint};
use num_traits::{One, Zero};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::config::{GroupConfig, GroupParameters};

pub struct Prover2 {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub h: BigUint,
    pub k: BigUint,
    pub a: BigUint,
    pub b: BigUint,
}

pub struct Proof2 {
    pub r1: BigUint,
    pub r2: BigUint,
    pub s: BigUint,
    pub c: BigUint
}

impl Prover2 {

    pub fn new(config: GroupConfig) -> Self {
        let mut rng = thread_rng();
        let GroupParameters { p, g, q } = config.parameters();
        let z = rng.sample(&Uniform::new(BigUint::one(), &q));
        let h = g.modpow(&z, &p);
        let k = rng.sample(&Uniform::new(BigUint::one(), &q));
        let a = g.modpow(&k, &p);
        let b = h.modpow(&k, &p);

        Self {
            p, q, g, h, k, a, b
        }
    }

    pub fn generate_proof(&self, x: &BigUint, c: &BigUint) -> BigUint {
        assert!(c > &BigUint::zero() && c < &self.q, "Challenge c must be in the range [1, q-1]");
        (&self.k).add(c.mul(x)).modpow(&BigUint::one(), &self.q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        let q = BigUint::parse_bytes(b"F518AA8781A8DF278ABA4E7D64B7CB9D49462353", 16).unwrap();

        for i in 0..100
        {
            let x = rng.sample(&Uniform::new(BigUint::one(), &q));
            let c = rng.sample(&Uniform::new(BigUint::one(), &q));

            let prover = Prover2::new(GroupConfig::Group1024);
            let r = prover.generate_proof(&x, &c);
            let cx = &c.mul(&x);

            let validate =
                prover.g.modpow(&r, &prover.p) == (prover.a.mul(&prover.g.modpow(&cx, &prover.p)).modpow(&BigUint::one(), &prover.p)) &&
                    prover.h.modpow(&r, &prover.p) == (prover.b.mul(&prover.h.modpow(&cx, &prover.p)).modpow(&BigUint::one(), &prover.p));

            assert!(validate, "Proof failed for iteration {}", i);
        }
    }
}