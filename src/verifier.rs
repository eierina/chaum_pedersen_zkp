use num_bigint::{BigUint, ToBigUint};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::config::{ ProtocolParams };
use crate::prover::{Commitment, PublicValues};

pub struct Verifier {
    params: ProtocolParams
}

impl Verifier {
    pub fn new(params: ProtocolParams) -> Self {
        Self { params }
    }

    pub fn create_challenge(&self) -> BigUint {
        let mut rng = thread_rng();
        rng.sample(&Uniform::new(&2.to_biguint().unwrap(), &self.params.q))
    }

    pub fn verify_challenge_response(&self, c: &BigUint, t: &BigUint, commitment: &Commitment, public_values: &PublicValues) -> bool {
        let validate1 = self.params.g.modpow(&t, &self.params.p) == ((&commitment.a1 * &public_values.y.modpow(&c, &self.params.p)) % &self.params.p);
        let validate2 = self.params.h.modpow(&t, &self.params.p) == ((&commitment.a2 * &public_values.z.modpow(&c, &self.params.p)) % &self.params.p);
        validate1 && validate2
    }
}

#[cfg(test)]
mod tests {
    use crate::config::GroupConfig;
    use crate::prover::Prover;
    use super::*;

    #[test]
    fn proof_verification_with_1024_bit_group() {
        let prover = Prover::new(GroupConfig::Group1024);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let verifier = Verifier::new(prover.params.clone());
        let challenge = verifier.create_challenge();
        let response = prover.create_response(&k, &secret, &challenge);


        let validate = verifier.verify_challenge_response(&challenge, &response, &commitment, &public_values);
        assert!(validate, "Proof verification failed");
    }

    #[test]
    fn proof_verification_with_2048_224_bit_group() {
        let prover = Prover::new(GroupConfig::Group2048_224);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let verifier = Verifier::new(prover.params.clone());
        let challenge = verifier.create_challenge();
        let response = prover.create_response(&k, &secret, &challenge);

        let verifier = Verifier::new(prover.params);
        let validate = verifier.verify_challenge_response(&challenge, &response, &commitment, &public_values);
        assert!(validate, "Proof verification failed");
    }

    #[test]
    fn proof_verification_with_2048_256_bit_group() {
        let prover = Prover::new(GroupConfig::Group2048_256);
        let secret = prover.generate_random_number();
        let public_values = prover.create_public_values(&secret);
        let (k, commitment) = prover.create_commitment();
        let verifier = Verifier::new(prover.params.clone());
        let challenge = verifier.create_challenge();
        let response = prover.create_response(&k, &secret, &challenge);

        let verifier = Verifier::new(prover.params);
        let validate = verifier.verify_challenge_response(&challenge, &response, &commitment, &public_values);
        assert!(validate, "Proof verification failed");
    }
}