use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct BernsteinPolynomial {
    pub coeffs: Vec<f32>,
}

impl BernsteinPolynomial {
    pub fn with_coefficients(coeffs: Vec<f32>) -> Self {
        Self { coeffs }
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    pub fn value(&self, t: f32) -> f32 {
        let t1 = 1.0 - t;

        let mut values = self.coeffs.clone();
        let mut values_swap = vec![0.0; values.len()];

        // De Casteljau algorithm
        for i in (1..=self.degree()).rev() {
            for j in 0..i {
                values_swap[j] = t1 * values[j] + t * values[j + 1];
            }

            std::mem::swap(&mut values, &mut values_swap);
        }

        values[0]
    }

    pub fn derivative(&self, t: f32) -> f32 {
        assert!(self.coeffs.len() >= 1);
        let degree = self.coeffs.len() as f32;

        // This is inefficient to do on every call to `derivative`
        let derivative_coeffs: Vec<_> = self
            .coeffs
            .iter()
            .tuple_windows()
            .map(|(a0, a1)| degree * (-a0 + a1))
            .collect();

        let derivative = BernsteinPolynomial::with_coefficients(derivative_coeffs);
        derivative.value(t)
    }
}
