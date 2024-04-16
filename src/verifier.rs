use num_bigint::BigUint;
use crate::config::{GroupConfig, GroupParameters};
use crate::prover3::Proof;

pub struct Verifier {
    params: GroupParameters,
}

impl Verifier {
    pub fn new(config: GroupConfig) -> Self {
        Self { params: config.group_params() }
    }
    // fn verify_zero_knowledge_proof(&self, proof: &Proof) -> bool {
    //     let GroupParameters { p, g, q } = &self.params;
    //     // let cx = &proof.c * x;
    //     // let validate1 = g.modpow(&proof.r, &p) == ((&proof.a * &g.modpow(&cx, &p)) % p);
    //     // let validate2 = proof.h.modpow(&proof.r, &p) == ((&proof.b * proof.h.modpow(&cx, &p)) % p);
    //     // validate1 && validate2
    //
    //     // let validate1 = proof.g.modpow(&proof.r, &proof.p) == ((&proof.a * &proof.y1.modpow(&proof.c, &proof.p)) % &proof.p);
    //     // let validate2 = proof.h.modpow(&proof.r, &proof.p) == ((&proof.b * &proof.y2.modpow(&proof.c, &proof.p)) % &proof.p);
    //     // let success = validate1 && validate2;
    // }
}

