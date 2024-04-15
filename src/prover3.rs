use num_bigint::{BigUint};
use num_traits::{One, Zero};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::config::{GroupConfig, GroupParameters};

pub struct Prover {
    pub p: BigUint, // Prime modulus, defines the finite field
    pub q: BigUint, // Order of the subgroup, must be a prime number
    pub g: BigUint, // Primary generator of the subgroup
    pub h: BigUint, // Secondary generator, used for additional proofs
    pub k: BigUint, // Nonce, randomly generated for each proof to ensure uniqueness
    pub a: BigUint, // Secret exponent used in generating the commitment A
    pub b: BigUint, // Secret exponent used in generating the commitment B
}

pub struct Proof {
    pub c: BigUint,  // Challenge from verifier, ensures proof integrity
    pub r: BigUint,  // Challenge response, combines nonce and secret exponent
    pub r1: BigUint, // Response component 1, calculated using g
    pub r2: BigUint, // Response component 2, calculated using h
}

impl Prover {

    // Create a new Prover instance with cryptographic parameters initialized from a specified group configuration
    pub fn new(config: GroupConfig) -> Self {
        let mut rng = thread_rng();
        let GroupParameters { p, g, q } = config.parameters();
        let q_sample_range = &Uniform::new(BigUint::one(), &q);
        let z = rng.sample(&q_sample_range);
        let k = rng.sample(&q_sample_range);
        let h = g.modpow(&z, &p);
        let a = g.modpow(&k, &p);
        let b = h.modpow(&k, &p);

        Self { p, q, g, h, k, a, b }
    }

    pub fn create_zero_knowledge_proof(&self, x: &BigUint, c: &BigUint) -> Proof {
        assert!(c > &BigUint::zero() && c < &self.q, "Challenge c must be in the range [1, q-1]");
        let r = (&self.k + (c * x)) % &self.q;
        Proof {
            r1: self.g.modpow(&r, &self.p),
            r2: self.h.modpow(&r, &self.p),
            r: r.clone(),
            c: c.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Prover {
        fn verify_zero_knowledge_proof(&self, x: &BigUint, c: &BigUint, proof: &Proof) -> bool {
            let cx = c * x;
            let validate1 = self.g.modpow(&proof.r, &self.p) == ((&self.a * &self.g.modpow(&cx, &self.p)) % &self.p);
            let validate2 = self.h.modpow(&proof.r, &self.p) == ((&self.b * &self.h.modpow(&cx, &self.p)) % &self.p);
            validate1 && validate2
        }
    }

    #[test]
    fn proof_verification_with_1024_bit_group() {
        let mut rng = thread_rng();
        let prover = Prover::new(GroupConfig::Group1024);

        let x = rng.sample(&Uniform::new(BigUint::one(), &prover.q));
        let c = rng.sample(&Uniform::new(BigUint::one(), &prover.q));

        let proof = prover.create_zero_knowledge_proof(&x, &c);

        assert!(prover.verify_zero_knowledge_proof(&x, &c, &proof), "Proof failed");
    }

    #[test]
    fn proof_verification_with_2048_224_bit_group() {
        let mut rng = thread_rng();
        let prover = Prover::new(GroupConfig::Group2048_224);

        let x = rng.sample(&Uniform::new(BigUint::one(), &prover.q));
        let c = rng.sample(&Uniform::new(BigUint::one(), &prover.q));

        let proof = prover.create_zero_knowledge_proof(&x, &c);

        assert!(prover.verify_zero_knowledge_proof(&x, &c, &proof), "Proof failed");
    }

    #[test]
    fn proof_verification_with_2048_256_bit_group() {
        let mut rng = thread_rng();
        let prover = Prover::new(GroupConfig::Group2048_256);

        let x = rng.sample(&Uniform::new(BigUint::one(), &prover.q));
        let c = rng.sample(&Uniform::new(BigUint::one(), &prover.q));

        let proof = prover.create_zero_knowledge_proof(&x, &c);

        assert!(prover.verify_zero_knowledge_proof(&x, &c, &proof), "Proof failed");
    }
}