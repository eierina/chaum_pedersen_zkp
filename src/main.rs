mod verifier;
mod config;
mod prover3;

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::{thread_rng, Rng};

fn main() {
}

// /// Represents a Zero-Knowledge Proof (ZKP) using the Chaum-Pedersen protocol.
// ///
// /// The Chaum-Pedersen protocol is used to demonstrate that the logarithm of `alpha` to the base `g`
// /// is the same as the logarithm of `beta` to the base `h` without revealing the actual logarithm values.
// pub struct ZKP {
//     x: BigUint,
//
//     /// The `alpha` value in the ZKP protocol, which is one of the public keys
//     /// used to demonstrate knowledge of the logarithm without disclosing it.
//     pub alpha: BigUint,
//
//     /// The `beta` value in the ZKP protocol, typically another public key
//     /// used alongside `alpha` to substantiate the hidden logarithm's equality across different bases.
//     pub beta: BigUint,
//
//     pub y1: BigUint,
//
//     pub y2: BigUint,
//
//     /// A large prime `p`, used as the modulus for calculations in the proof to ensure that
//     /// arithmetic operations are performed within a finite field.
//     pub p: BigUint,
//
//     /// The base `g` used in calculations, serving as one of the generators of the cyclic group.
//     /// This is used to compute powers for public keys and to demonstrate knowledge of logarithms securely.
//     pub g: BigUint,
//
//     /// A prime number `q`, often used as an order of the subgroup in cryptographic applications,
//     /// ensuring the cyclic group generated has prime order which is critical for security reasons.
//     pub q: BigUint,
//
//     rng: StdRng,
// }
//
// pub struct ZKPResponse {
//     pub r1: BigUint,
//     pub r2: BigUint,
//     pub s: BigUint,
//     pub c: BigUint
// }
//
// impl ZKP {
//
//     /// Creates a new instance of the Chaum-Pedersen ZKP protocol using the specified group configuration.
//     pub fn new(x: BigUint, parameters: GroupParameters) -> Self {
//         let mut rng = StdRng::from_entropy();
//         let (p_str, g_str, q_str) = parameters.parameters();
//
//         let p = BigUint::parse_bytes(p_str.as_bytes(), 16).unwrap();
//         let g = BigUint::parse_bytes(g_str.as_bytes(), 16).unwrap();
//         let q = BigUint::parse_bytes(q_str.as_bytes(), 16).unwrap();
//
//         let alpha = g.modpow(&x, &p);
//         let beta = g.modpow(&rng.gen_biguint_below(&q), &p);  // Example: Using g for beta as well for simplicity
//
//         let y1 = alpha.modpow(&x, &p);
//         let y2 = beta.modpow(&x, &p);  // Using beta and alpha for y1 and y2 for demonstration
//
//         Self {
//             x, alpha, beta, y1, y2, p, g, q, rng
//         }
//     }
//
//     pub fn response(&mut self, c: &BigUint) -> ZKPResponse {
//         let k = self.rng.gen_biguint_below(&self.q);
//
//         ZKPResponse {
//             r1: self.alpha.modpow(&k, &self.p),
//             r2: self.beta.modpow(&k, &self.p),
//             s: k.sub(&c.mul(&self.x)).modpow(&1.to_biguint().unwrap(), &self.q),
//             c: c.clone()
//         }
//     }
// }

// fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool {
//     r1.eq(&alpha.modpow(s, p).mul(&y1.modpow(c, p)).modpow(&1.to_biguint().unwrap(), p)) &&
//         r2.eq(&beta.modpow(s, p).mul(&y2.modpow(c, p)).modpow(&1.to_biguint().unwrap(), p) )
// }


/*
chose random a, b - need not to be (?) a priem number
compute <g, A, B, C> - known to prover and verifier
A = g^a mod q
B = g^b mod q
C = g^ab mod q
*/