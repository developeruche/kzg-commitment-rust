use oblast::Fr;

#[derive(Debug)]
pub struct Polynomial {
    // NOTE: low-order coefficients are first in the vector
    pub coefficients: Vec<Fr>,
}

impl Polynomial {
    pub fn evaluate_at(self: &Self, point: Fr) -> Fr {
        let mut sum = self.coefficients[0].clone();
        let mut powers = point.clone();

        for coefficient in self.coefficients.iter().skip(1) {
            let term = *coefficient * powers;
            sum += term;
            powers *= point;
        }

        sum
    }
}

pub fn from_coefficients(coefficients: impl Iterator<Item = Fr>) -> Polynomial {
    Polynomial {
        coefficients: coefficients.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_eval_polynomial() {
        let coefficients = vec![42, 1, 1, 0, 1]
            .into_iter()
            .map(Fr::from_u64)
            .collect::<Vec<_>>();
        let polynomial = from_coefficients(coefficients.into_iter());
        let point = Fr::from_u64(2);
        let result_in_fr = polynomial.evaluate_at(point);
        let result = result_in_fr.as_u64();
        assert_eq!(result, 64);
    }
}
