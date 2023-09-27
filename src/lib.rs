mod playground;

use ark_bls12_381::Bls12_381;
use ark_ff::{BigInt, BigInteger, Field, One, PrimeField, Zero};
use ark_poly::univariate::DensePolynomial;
use ark_ec::pairing::Pairing;
use ark_poly::DenseUVPolynomial;
use ark_std::test_rng;





type UniPoly_381 = DensePolynomial<<Bls12_381 as Pairing>::ScalarField>;


#[derive(Default, Debug, Clone)]
pub struct KZG {
    pub polynomial: UniPoly_381,
    // pub curve:
    pub polynomial_degree: usize, // this would be operated on using BigInteger trait
    pub tau: usize,
    pub a: usize,
    pub commitment: BigInt<{ usize }>,
    pub quotient_polynomial: UniPoly_381,
    pub proof_pie: BigInt<{ usize }>
}



impl KZG {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_rand_poly() -> Self {
        let rng = &mut test_rng();

        Self {
            polynomial: UniPoly_381::rand(10, rng),
            ..Default::default()
        }
    }


    pub fn trusted_set_up(&mut self) {

    }

    pub fn commit(&mut self) {

    }

    pub fn evaluate_n_proof(&mut self) {

    }

    pub fn verify_an_evaluation(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    pub fn test_kzg() {
        let kzg = KZG::new_with_rand_poly();
        let pt = kzg.clone().polynomial;

        println!("I am printing now! pt _> {:?}", pt[0].to_string());
    }
}