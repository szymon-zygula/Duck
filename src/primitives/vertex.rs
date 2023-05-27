use nalgebra::{Point3, Vector2, Vector3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DuckVertex {
    position: Point3<f32>,
    normal: Vector3<f32>,
    tex: Vector2<f32>,
}

impl DuckVertex {
    pub fn new(position: Point3<f32>, normal: Vector3<f32>, tex: Vector2<f32>) -> Self {
        Self {
            position,
            normal,
            tex,
        }
    }
}
