use crate::{render::texture::Texture, utils};
use image::{GenericImage, Rgba};
use nalgebra::Vector3;

pub struct Water {
    width: usize,

    pub wave_speed: f32,

    heights: Vec<f32>,
    heights_swap: Vec<f32>,
}

impl Water {
    const PATCH_SIDE_LEN: f32 = 2.0;

    pub fn new(width: usize, wave_speed: f32) -> Self {
        let heights = vec![0.0; width * width];
        Self {
            width,

            wave_speed,

            heights_swap: heights.clone(),
            heights,
        }
    }

    fn height_index(&self, x: isize, y: isize) -> usize {
        let x = x.clamp(0, self.width as isize - 1) as usize;
        let y = y.clamp(0, self.width as isize - 1) as usize;
        self.width * y + x
    }

    pub fn height(&self, x: isize, y: isize) -> f32 {
        let idx = self.height_index(x, y);
        self.heights[idx]
    }

    fn height_mut(&mut self, x: isize, y: isize) -> &mut f32 {
        let idx = self.height_index(x, y);
        &mut self.heights[idx]
    }

    pub fn height_swap(&self, x: isize, y: isize) -> f32 {
        let idx = self.height_index(x, y);
        self.heights_swap[idx]
    }

    fn height_swap_mut(&mut self, x: isize, y: isize) -> &mut f32 {
        let idx = self.height_index(x, y);
        &mut self.heights_swap[idx]
    }

    pub fn normal(&self, x: isize, y: isize) -> Vector3<f32> {
        let x_tangent = Vector3::new(
            Self::PATCH_SIDE_LEN / self.width as f32,
            0.5 * (self.height(x - 1, y) - self.height(x + 1, y)),
            0.0,
        );

        let z_tangent = Vector3::new(
            0.0,
            0.5 * (self.height(x, y - 1) - self.height(x, y + 1)),
            Self::PATCH_SIDE_LEN / self.width as f32,
        );

        Vector3::cross(&z_tangent, &x_tangent).normalize()
    }

    fn normal_rgba(&self, x: isize, y: isize) -> Rgba<u8> {
        let n = self.normal(x, y);
        Rgba([
            utils::normal_f32_to_u8(n.x),
            utils::normal_f32_to_u8(n.y),
            utils::normal_f32_to_u8(n.z),
            utils::normal_f32_to_u8(0.0),
        ])
    }

    pub fn disturb(&mut self, x: isize, y: isize, height: f32) {
        *self.height_mut(x, y) = height;
    }

    fn derivative_step(&self) -> f32 {
        2.0 / (self.width as f32 - 1.0)
    }

    fn damping_coeff(&self, x: isize, y: isize) -> f32 {
        let border_distance = f32::max(
            (x as f32 - Self::PATCH_SIDE_LEN).abs(),
            (y as f32 - Self::PATCH_SIDE_LEN).abs(),
        );

        0.95 * f32::min(1.0, 5.0 * border_distance)
    }

    fn delta(&self) -> f32 {
        1.0 / self.width as f32
    }

    fn a_coeff(&self) -> f32 {
        let derivative_step = self.derivative_step();
        let delta = self.delta();
        self.wave_speed * self.wave_speed * delta * delta
            / (derivative_step * derivative_step)
    }

    fn b_coeff(a_coeff: f32) -> f32 {
        2.0 - 4.0 * a_coeff
    }

    fn neighbor_sum(&self, x: isize, y: isize) -> f32 {
        self.height(x, y - 1)
            + self.height(x, y + 1)
            + self.height(x - 1, y)
            + self.height(x + 1, y)
    }

    pub fn update(&mut self) {
        // In theory, this should be constant
        let a = self.a_coeff();
        let b = Self::b_coeff(a);

        for x in 0..(self.width as isize) {
            for y in 0..(self.width as isize) {
                let d = self.damping_coeff(x, y);
                let n = self.neighbor_sum(x, y);

                *self.height_swap_mut(x, y) =
                    d * (a * n + b * self.height(x, y) - self.height_swap(x, y));
            }
        }

        std::mem::swap(&mut self.heights, &mut self.heights_swap);
    }

    pub fn normal_texture(&self) -> Texture {
        let mut texture = Texture::new_rgba(self.width as u32, self.width as u32);

        for x in 0..(self.width as isize) {
            for y in 0..(self.width as isize) {
                texture
                    .image
                    .put_pixel(x as u32, y as u32, self.normal_rgba(x, y));
            }
        }

        texture
    }
}
