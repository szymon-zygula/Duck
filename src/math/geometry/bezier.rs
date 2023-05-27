use crate::math::bspline::CubicBSpline;
use nalgebra::Point3;

#[derive(Clone, Debug)]
pub struct BezierBSpline {
    x_t: CubicBSpline,
    y_t: CubicBSpline,
    z_t: CubicBSpline,
}

impl BezierBSpline {
    pub fn through_points(points: Vec<Point3<f32>>) -> Self {
        assert!(points.len() >= 4);

        Self {
            x_t: CubicBSpline::with_coefficients(points.iter().map(|p| p.x).collect()),
            y_t: CubicBSpline::with_coefficients(points.iter().map(|p| p.y).collect()),
            z_t: CubicBSpline::with_coefficients(points.iter().map(|p| p.z).collect()),
        }
    }

    pub fn bernstein_points(&self) -> Vec<Point3<f32>> {
        let bernstein_x = self.x_t.bernstein_values();
        let bernstein_y = self.y_t.bernstein_values();
        let bernstein_z = self.z_t.bernstein_values();
        let mut bernstein = Vec::new();

        for i in 0..bernstein_x.len() {
            bernstein.push(Point3::new(bernstein_x[i], bernstein_y[i], bernstein_z[i]));
        }

        bernstein
    }

    pub fn deboor_points(&self) -> Vec<Point3<f32>> {
        let deboor_x = self.x_t.deboor_points();
        let deboor_y = self.y_t.deboor_points();
        let deboor_z = self.z_t.deboor_points();
        let mut deboor = Vec::new();

        for i in 0..deboor_x.len() {
            deboor.push(Point3::new(deboor_x[i], deboor_y[i], deboor_z[i]))
        }

        deboor
    }

    fn value(&self, t: f32) -> Point3<f32> {
        Point3::new(self.x_t.value(t), self.y_t.value(t), self.z_t.value(t))
    }
}
