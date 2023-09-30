mod playground;
use oblast::{curve_order, Scalar, P1, P2};
use num_bigint::BigUint;
use rand::prelude::*;





#[derive(Clone, Debug, PartialEq)]
struct PP {
    pub points_in_g1: Vec<P1>,
    pub point_in_g2: P2
}


#[derive(Clone, Debug, PartialEq)]
struct KZG {
    pub public_parameter: PP
}


#[derive(Debug)]
enum KZGErrors {
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
