mod playground;
mod poly;

use oblast::{curve_order, verify_pairings, Scalar, P1, P2, Fr};
use num_bigint::BigUint;
use rand::prelude::*;




// ====================================
// DATA-STRUCTURE                     =
// ====================================
#[derive(Clone, Debug, PartialEq)]
pub struct PP {
    pub points_in_g1: Vec<P1>,
    pub point_in_g2: P2
}


#[derive(Clone, Debug, PartialEq)]
pub  struct KZG {
    pub public_parameter: PP
}

#[derive(Debug)]
pub struct Commitment<'a> {
    pub element: P1,
    pub polynomial: &'a poly::Polynomial,
    pub public_parameter: &'a PP,
}

#[derive(Debug)]
pub struct Opening {
    pub value: Fr,
    pub proof: P1,
}



// =========== CUSTOM DEFINED ERROR;
#[derive(Debug)]
pub enum KZGErrors {
    SecretMustBeLessThanTheOrderOfTheGroup
}


impl KZG {
    fn new(tau: &[u8; 32], degree: usize) -> Result<KZG, KZGErrors> {
        KZG::setup_internal(tau, degree)
    }

    fn new_rand(degree: usize) -> Result<KZG, KZGErrors> {
        let mut rng = thread_rng();

        let mut secret = [0u8; 32];
        rng.fill_bytes(&mut secret);

        let mut s = BigUint::from_bytes_be(&secret);

        let modulus = curve_order();
        while s >= modulus {
            rng.fill_bytes(&mut secret);
            s = BigUint::from_bytes_be(&secret);
        }


        KZG::setup_internal(&secret, degree)
    }


    fn setup_internal(tau: &[u8; 32], degree: usize) -> Result<KZG, KZGErrors> {
        let modulus = curve_order();
        let bytes_tau = BigUint::from_bytes_be(tau);


        if bytes_tau > modulus {
            return Err(KZGErrors::SecretMustBeLessThanTheOrderOfTheGroup);
        }

        let mut points_in_g1 = vec![];

        // obtaining the generator in the first group (this is the cyclic group)
        let g1 = P1::generator();

        // obtaining the "power of tau" (a part of the public parameter)
        for i in 0..=degree {
            let i_as_bigint = BigUint::from_slice(&[i as u32]);
            let s_i_as_bigint = bytes_tau.modpow(&i_as_bigint, &modulus);

            let mut s_i_bytes = vec![0u8; 32];
            let raw_bytes = s_i_as_bigint.to_bytes_be();
            s_i_bytes[32 - raw_bytes.len()..].copy_from_slice(&raw_bytes);
            let s_i_scalar = Scalar::from_fr_bytes(&s_i_bytes);

            let result = s_i_scalar * g1;
            points_in_g1.push(result);
        }


        let scalar = Scalar::from_fr_bytes(tau);
        let result_in_g2 = scalar * P2::generator();

        let public_parameter = PP {
            points_in_g1,
            point_in_g2: result_in_g2,
        };

        Ok(
            KZG {
                public_parameter
            }
        )
    }


    pub fn commit<'a>(
        public_parameter: &'a PP,
        polynomial: &'a poly::Polynomial,
    ) -> Result<Commitment<'a>, KZGErrors> {
        let basis = &public_parameter.points_in_g1;
        let coefficients = &polynomial.coefficients;

        let mut result = P1::default();
        for (coefficient, element) in coefficients.iter().zip(basis.iter()) {
            let term = *coefficient * *element;
            result = result + term;
        }

        Ok(Commitment {
            element: result,
            polynomial,
            public_parameter: &public_parameter,
        })
    }
}


impl<'a> Commitment<'a> {
    pub fn open_at(self: &Self, point: Fr) -> Result<Opening, KZGErrors> {
        let result = self.polynomial.evaluate_at(point);

        // divisor `s - x` for `f(x) = y`
        let divisor_coefficients = vec![-point, Fr::from_u64(1)];
        let divisor = poly::from_coefficients(divisor_coefficients.into_iter());
        let quotient_polynomial = compute_quotient(self.polynomial, &divisor);

        let commitment = KZG::commit(self.public_parameter, &quotient_polynomial)?;

        Ok(Opening {
            value: result,
            proof: commitment.element,
        })
    }
}


// ===================================
// FREE FUNCTIONS
// ===================================
fn compute_quotient( // this is a simple function for dividing a polynomial and returning the q
    dividend: &poly::Polynomial,
    divisor: &poly::Polynomial,
) -> poly::Polynomial {
    let mut dividend = dividend.coefficients.clone();
    let mut coefficients = vec![];

    let mut dividend_pos = dividend.len() - 1;
    let divisor_pos = divisor.coefficients.len() - 1;
    let mut difference = dividend_pos as isize - divisor_pos as isize;

    while difference >= 0 {
        let term_quotient = dividend[dividend_pos] / divisor.coefficients[divisor_pos];
        coefficients.push(term_quotient);

        for i in (0..=divisor_pos).rev() {
            let difference = difference as usize;
            let x = divisor.coefficients[i];
            let y = x * term_quotient;
            let z = dividend[difference + i];
            dividend[difference + i] = z - y;
        }

        dividend_pos -= 1;
        difference -= 1;
    }

    coefficients.reverse();
    poly::Polynomial { coefficients }
}

impl Opening {
    pub fn verify(&self, input: &Fr, commitment: &Commitment) -> bool {
        // Compute [f(s) - y]_1 for LHS
        let y_p1 = self.value * P1::generator();
        let commitment_minus_y = commitment.element + -y_p1;

        // Compute [s - z]_2 for RHS
        let z_p2 = *input * P2::generator();
        let s_minus_z = commitment.public_parameter.point_in_g2 + -z_p2;

        verify_pairings(commitment_minus_y, P2::generator(), self.proof, s_minus_z)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        let tau = [34u8; 32];
        let degree = 29;


        let kzg = KZG::new(&tau, degree).unwrap();
        println!("This is KZG -> {:?}", kzg);
        assert_eq!(kzg.public_parameter.points_in_g1.len(), degree + 1);
    }
}
