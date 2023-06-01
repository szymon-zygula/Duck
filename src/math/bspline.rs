use super::bernstein_polynomial::BernsteinPolynomial;

#[derive(Clone, Debug)]
pub struct CubicBSpline {
    bernsteins: Vec<BernsteinPolynomial>,
    deboor_points: Vec<f32>,
}

impl CubicBSpline {
    pub fn with_coefficients(deboor_points: Vec<f32>) -> Self {
        Self {
            bernsteins: Self::as_cubic_c0(&deboor_points),
            deboor_points,
        }
    }

    fn as_cubic_c0(deboor_points: &[f32]) -> Vec<BernsteinPolynomial> {
        let mut bernsteins = Vec::new();

        for i in 0..deboor_points.len() - 1 {
            bernsteins.push(BernsteinPolynomial::with_coefficients(vec![
                0.0,
                (2.0 * deboor_points[i] + deboor_points[i + 1]) / 3.0,
                (deboor_points[i] + 2.0 * deboor_points[i + 1]) / 3.0,
                0.0,
            ]));
        }

        for i in 1..deboor_points.len() - 2 {
            bernsteins[i].coeffs[0] = (bernsteins[i - 1].coeffs[2] + bernsteins[i].coeffs[1]) * 0.5;
            bernsteins[i].coeffs[3] = (bernsteins[i].coeffs[2] + bernsteins[i + 1].coeffs[1]) * 0.5;
        }

        bernsteins[1..deboor_points.len() - 2].to_vec()
    }

    fn curve_idx(&self, t: f32) -> usize {
        if t == 1.0 {
            self.bernsteins.len() - 1
        } else {
            (t * self.bernsteins.len() as f32).floor() as usize
        }
    }

    fn curve_t(&self, t: f32, curve_idx: usize) -> f32 {
        self.bernsteins.len() as f32 * t - curve_idx as f32
    }

    pub fn value(&self, t: f32) -> f32 {
        let curve_idx = self.curve_idx(t);
        let curve_t = self.curve_t(t, curve_idx);
        self.bernsteins[curve_idx].value(curve_t)
    }

    pub fn derivative(&self, t: f32) -> f32 {
        let curve_idx = self.curve_idx(t);
        let curve_t = self.curve_t(t, curve_idx);
        self.bernsteins[curve_idx].derivative(curve_t)
    }

    pub fn bernstein_values(&self) -> Vec<f32> {
        let mut vals = Vec::new();
        for bernstein in &self.bernsteins {
            vals.push(bernstein.coeffs[0]);
            vals.push(bernstein.coeffs[1]);
            vals.push(bernstein.coeffs[2]);
        }

        vals.push(self.bernsteins.last().unwrap().coeffs[3]);
        vals
    }

    pub fn deboor_points(&self) -> Vec<f32> {
        self.deboor_points.clone()
    }
}
