use crate::{math::affine::transforms, mouse::MouseState, window::Window};
use glutin::dpi::PhysicalSize;
use nalgebra::{Matrix4, Point3, Point4};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub view_basis: Matrix4<f32>,
    pub resolution: PhysicalSize<u32>,
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
}

impl Camera {
    const ROTATION_SPEED: f32 = 0.05;
    const MOVEMENT_SPEED: f32 = 0.01;
    const SCROLL_SPEED: f32 = 0.2;

    pub fn new() -> Camera {
        Camera {
            view_basis: Matrix4::identity(),
            resolution: PhysicalSize::new(0, 0),
            near_plane: 0.1,
            far_plane: 10000.0,
            fov: std::f32::consts::PI * 0.5,
        }
    }

    pub fn update_from_mouse(&mut self, mouse: &mut MouseState, window: &Window) -> bool {
        let mouse_delta = mouse.position_delta();
        let scroll_delta = mouse.scroll_delta();

        (mouse_delta.x != 0.0 || mouse_delta.y != 0.0 || scroll_delta != 0.0)
            && !window.imgui_using_mouse()
    }

    pub fn position(&self) -> Point3<f32> {
        let homogeneous_position = self.inverse_view_transform() * Point4::new(0.0, 0.0, 0.0, 1.0);
        Point3::from_homogeneous(homogeneous_position.coords).unwrap()
    }

    pub fn view_transform(&self) -> Matrix4<f32> {
        self.view_basis.transpose()
    }

    pub fn inverse_view_transform(&self) -> Matrix4<f32> {
        self.view_basis
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
