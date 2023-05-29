use crate::math::affine::transforms;
use glutin::dpi::PhysicalSize;
use nalgebra::{Matrix4, Point3, Vector3, Vector4};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub position: Point3<f32>,
    pub angle_y: f32,
    pub angle_x: f32,

    pub resolution: PhysicalSize<u32>,

    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3::origin(),
            angle_y: 0.0,
            angle_x: 0.0,

            resolution: PhysicalSize::new(0, 0),

            near_plane: 0.1,
            far_plane: 10000.0,
            fov: std::f32::consts::PI * 0.5,
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Point3<f32>) {
        self.position = position;
    }

    pub fn view_dir(&self) -> Vector3<f32> {
        Vector3::from_homogeneous(
            transforms::rotate_y(self.angle_y)
                * transforms::rotate_x(self.angle_x)
                * Vector4::new(0.0, 0.0, -1.0, 0.0),
        )
        .unwrap()
    }

    pub fn left_dir(&self) -> Vector3<f32> {
        Vector3::from_homogeneous(
            transforms::rotate_y(self.angle_y) * Vector4::new(-1.0, 0.0, 0.0, 0.0),
        )
        .unwrap()
    }

    pub fn up_dir(&self) -> Vector3<f32> {
        Vector3::from_homogeneous(
            transforms::rotate_x(self.angle_x) * Vector4::new(0.0, 1.0, 0.0, 0.0),
        )
        .unwrap()
    }

    pub fn view_transform(&self) -> Matrix4<f32> {
        transforms::rotate_x(-self.angle_x)
            * transforms::rotate_y(-self.angle_y)
            * transforms::translate(-self.position.coords)
    }

    pub fn inverse_view_transform(&self) -> Matrix4<f32> {
        transforms::translate(self.position.coords)
            * transforms::rotate_y(self.angle_y)
            * transforms::rotate_x(self.angle_x)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.resolution.width as f32 / self.resolution.height as f32
    }

    pub fn projection_transform(&self) -> Matrix4<f32> {
        transforms::projection(
            self.fov,
            self.aspect_ratio(),
            self.near_plane,
            self.far_plane,
        )
    }

    pub fn inverse_projection_transform(&self) -> Matrix4<f32> {
        transforms::inverse_projection(
            self.fov,
            self.aspect_ratio(),
            self.near_plane,
            self.far_plane,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
