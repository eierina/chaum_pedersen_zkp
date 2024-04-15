use crate::config::GroupConfig;
use crate::prover::Proof;

pub struct Verifier {
    config: GroupConfig,
}

impl Verifier {
    pub fn new(config: GroupConfig) -> Self {
        Self { config }
    }

    pub fn verify(&self, proof: &Proof) -> bool {
        todo!();
        //let validate1 = self.config.parameters().g.modpow(&proof.r, &prover.p) == (prover.a.mul(prover.g.modpow(&cx, &prover.p)).modpow(&BigUint::one(), &prover.p));
        //let validate2 = prover.h.modpow(&proof.r, &prover.p) == (prover.b.mul(prover.h.modpow(&cx, &prover.p)).modpow(&BigUint::one(), &prover.p));

        //self.verifyy(&proof.r1, &proof.r2, &self.config.y1, &self.config.y2, &self.config.alpha, &self.config.beta, &proof.c, &proof.s, &self.config.p);
    }

    // fn verifyy(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool {
    //     r1.eq(&alpha.modpow(s, p).mul(&y1.modpow(c, p)).modpow(&1.to_biguint().unwrap(), p)) &&
    //         r2.eq(&beta.modpow(s, p).mul(&y2.modpow(c, p)).modpow(&1.to_biguint().unwrap(), p) )
    // }
}

