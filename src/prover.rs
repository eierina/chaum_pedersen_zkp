use num_bigint::{BigUint, ToBigUint};
use num_traits::{One};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::config::{GroupConfig, ProtocolParams};

#[derive(Debug)]
pub struct Prover {
    pub params: ProtocolParams,
}

pub struct Commitment {
    pub a1: BigUint,
    pub a2: BigUint,
}

pub struct PublicValues {
    pub y: BigUint,
    pub z: BigUint,
}

impl Prover {

    // Create a new Prover instance with cryptographic parameters initialized from a specified group configuration
    pub fn new(config: GroupConfig) -> Self {
        let params = config.protocol_params();
        Self { params }
    }

    // Create a commitment to a random value k only known to the prover
    pub fn create_commitment(&self) -> (BigUint, Commitment) {
        let k = self.generate_random_number();
        let a1 = self.modpow_g(&k);
        let a2 = self.modpow_h(&k);
        (k, Commitment { a1, a2 })
    }

    pub fn create_public_values(&self, x: &BigUint) -> PublicValues {
        let y = self.modpow_g(&x);
        let z = self.modpow_h(&x);
        PublicValues { y, z }
    }

    pub fn create_response(&self, k: &BigUint, x: &BigUint, c: &BigUint) -> BigUint {
        assert!(c > &BigUint::one() && c < &self.params.q, "Challenge c must be in the range [2, q-1]");
        (k + (c * x)) % &self.params.q
    }

    pub fn generate_random_number(&self) -> BigUint {
        let mut rng = thread_rng();
        rng.sample(&Uniform::new(2.to_biguint().unwrap(), &self.params.q))
    }

    // Modular exponentiation of a value with the base 'g' using the provided exponent and modulus 'p'
    fn modpow_g(&self, exponent: &BigUint) -> BigUint {
        self.params.g.modpow(exponent, &self.params.p)
    }

    // Modular exponentiation of a value with the base 'h' using the provided exponent and modulus 'p'
    fn modpow_h(&self, exponent: &BigUint) -> BigUint {
        self.params.h.modpow(exponent, &self.params.p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_verification_with_1024_bit_group() {
        let prover = Prover::new(GroupConfig::Group1024);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let challenge = prover.generate_random_number();
        let response = prover.create_response(&k, &secret, &challenge);

        let validate1 = prover.modpow_g(&response) == ((&commitment.a1 * &public_values.y.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        let validate2 = prover.modpow_h(&response) == ((&commitment.a2 * &public_values.z.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        assert!(validate1 && validate2);
    }

    #[test]
    fn proof_verification_with_2048_224_bit_group() {
        let prover = Prover::new(GroupConfig::Group2048_224);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let challenge = prover.generate_random_number();
        let response = prover.create_response(&k, &secret, &challenge);

        let validate1 = prover.modpow_g(&response) == ((&commitment.a1 * &public_values.y.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        let validate2 = prover.modpow_h(&response) == ((&commitment.a2 * &public_values.z.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        assert!(validate1 && validate2);
    }

    #[test]
    fn proof_verification_with_2048_256_bit_group() {
        let prover = Prover::new(GroupConfig::Group2048_256);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let challenge = prover.generate_random_number();
        let response = prover.create_response(&k, &secret, &challenge);

        let validate1 = prover.modpow_g(&response) == ((&commitment.a1 * &public_values.y.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        let validate2 = prover.modpow_h(&response) == ((&commitment.a2 * &public_values.z.modpow(&challenge, &prover.params.p)) % &prover.params.p);
        assert!(validate1 && validate2);
    }
}
