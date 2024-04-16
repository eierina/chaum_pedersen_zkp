use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::config::{GroupConfig, GroupParameters, ProtocolParams};

// Sigma Protocol -
// Equality of Discrete Logarithms

pub struct Prover {
    pub params: ProtocolParams,
    pub k: BigUint, // Nonce, randomly generated for each proof to ensure uniqueness
    pub a: BigUint, // commitment
    pub b: BigUint, // commitment
    pub y1: BigUint,
    pub y2: BigUint,
}

pub struct Proof {
    pub params: ProtocolParams,
    pub a: BigUint,  // commitment
    pub b: BigUint,  // commitment
    pub c: BigUint,  // Challenge from verifier, ensures proof integrity
    pub r: BigUint,  // Challenge response, combines nonce and secret exponent
}

pub struct Commitment {
    pub a1: BigUint,  // commitment
    pub a2: BigUint,  // commitment
}

pub struct PublicValues {
    pub y: BigUint,
    pub z: BigUint,
}

impl Prover {

    // Create a new Prover instance with cryptographic parameters initialized from a specified group configuration
    pub fn new(x: &BigUint, config: GroupConfig) -> Self {
        let mut rng = thread_rng();
        let params = config.protocol_params();
        let k = rng.sample(&Uniform::new(&2.to_biguint().unwrap(), &params.q));
        let a = params.g.modpow(&k, &params.p);
        let b = params.h.modpow(&k, &params.p);
        let y1 = params.g.modpow(x, &params.p);
        let y2 = params.h.modpow(x, &params.p);

        Self { params, k, a, b, y1, y2 }
    }

    // Create a commitment to a random value k only known to the prover
    pub fn create_commitment(&self) -> (BigUint, Commitment) {
        let mut rng = thread_rng();
        let k = rng.sample(&Uniform::new(&2.to_biguint().unwrap(), &self.params.q));
        let a1 = self.params.g.modpow(&k, &self.params.p);
        let a2 = self.params.h.modpow(&k, &self.params.p);
        (k, Commitment { a1, a2 })
    }

    pub fn create_public_values(&self, x: &BigUint) -> PublicValues {
        let y = self.params.g.modpow(&x, &self.params.p);
        let z = self.params.h.modpow(&x, &self.params.p);
        PublicValues { y, z }
    }

    pub fn create_response(&self, k: &BigUint, x: &BigUint, c: &BigUint) -> BigUint {
        assert!(c > &BigUint::one() && c < &self.params.q, "Challenge c must be in the range [2, q-1]");
        (k + (c * x)) % &self.params.q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // impl Prover {
    //     fn verify_zero_knowledge_proof(&self, p: &BigUint, g: &BigUint, proof: &Proof) -> bool {
    //         let validate1 = proof.params.g.modpow(&proof.r, &p) == ((&proof.a * &self.y1.modpow(&c, &p)) % &p);
    //         let validate2 = proof.params.h.modpow(&proof.r, &p) == ((&proof.b * &self.y2.modpow(&c, &p)) % &p);
    //         let success = validate1 && validate2;
    //     }
    // }

    #[test]
    fn proof() {
        let mut rng = thread_rng();
        let prover = Prover::new(&BigUint::from(2u32), GroupConfig::Group1024);

        let secret = rng.sample(&Uniform::new(&2.to_biguint().unwrap(), &prover.params.q));

        let public_values = prover.create_public_values(&secret);

        let (k, commitment) = prover.create_commitment();

        let challenge = rng.sample(&Uniform::new(&2.to_biguint().unwrap(), &prover.params.q));

        let response = prover.create_response(&k, &secret, &challenge);

        let validate1 = prover.params.g.modpow(&response, &prover.params.p) == ((&commitment.a1 * &public_values.y.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        let validate2 = prover.params.h.modpow(&response, &prover.params.p) == ((&commitment.a2 * &public_values.z.modpow(&challenge, &prover.params.p)) % &prover.params.p);

        let success = validate1 && validate2;
    }

    #[test]
    fn proof_verification_with_1024_bit_group() {
        // let mut rng = thread_rng();
        //
        // let q = BigUint::parse_bytes(b"F518AA8781A8DF278ABA4E7D64B7CB9D49462353", 16).unwrap();
        // let x = rng.sample(&Uniform::new(BigUint::one(), &q));
        // let c = rng.sample(&Uniform::new(BigUint::one(), &q));
        // let prover = Prover::new(&x, GroupConfig::Group1024);
        //
        // let proof = prover.create_zero_knowledge_proof(&x, &c);
        //
        // let validate1 = prover.g.modpow(&proof.r, &prover.p) == ((&proof.a * &prover.y1.modpow(&c, &prover.p)) % &prover.p);
        // let validate2 = prover.h.modpow(&proof.r, &prover.p) == ((&proof.b * &prover.y2.modpow(&c, &prover.p)) % &prover.p);
        // let success = validate1 && validate2;
        /*
        a = g^k
        r = k + cx mod q
        g^r == ay^c mod p ??
          => g^r = g^(k + cx) = g^k * g^cx = a * (g^x)^c = a * y^c

        b = h^k
        r = k + cx mod q
        h^r == by^c mod p ??
          => h^r = h^(k + cx) = h^k * h^cx = b * (h^x)^c = b * y^c
         */

        //assert!(prover.verify_zero_knowledge_proof(&prover.p, &prover.g, &proof), "Proof failed");
    }

    // #[test]
    // fn proof_verification_with_2048_224_bit_group() {
    //     let mut rng = thread_rng();
    //
    //     let q = BigUint::parse_bytes(b"801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB", 16).unwrap();
    //     let x = rng.sample(&Uniform::new(BigUint::one(), &q));
    //     let c = rng.sample(&Uniform::new(BigUint::one(), &q));
    //     let prover = Prover::new(&x, GroupConfig::Group2048_224);
    //
    //     let proof = prover.create_zero_knowledge_proof(&x, &c);
    //
    //     assert!(prover.verify_zero_knowledge_proof(&prover.p, &prover.g, &proof), "Proof failed");
    // }
    //
    // #[test]
    // fn proof_verification_with_2048_256_bit_group() {
    //     let mut rng = thread_rng();
    //
    //     let q = BigUint::parse_bytes(b"8CF83642A709A097B447997640129DA299B1A47D1EB3750BA308B0FE64F5FBD3", 16).unwrap();
    //     let x = rng.sample(&Uniform::new(BigUint::one(), &q));
    //     let c = rng.sample(&Uniform::new(BigUint::one(), &q));
    //     let prover = Prover::new(&x, GroupConfig::Group2048_256);
    //
    //     let proof = prover.create_zero_knowledge_proof(&x, &c);
    //
    //     assert!(prover.verify_zero_knowledge_proof(&prover.p, &prover.g, &proof), "Proof failed");
    // }
}
// 1. Init:
// G, p, q, g, y1
// 2. Setup:
// a
// 3. Challenge
// c
// 4. Response
// r
// 5. Verificatoin
// bool
