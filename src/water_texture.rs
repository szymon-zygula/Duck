use crate::{render::gl_texture::GlTexture, water::Water};
use std::time::Duration;

pub struct WaterTexture<'gl> {
    water: Water,
    normal_texture: GlTexture<'gl>,
}

impl<'gl> WaterTexture<'gl> {
    pub fn new(gl: &'gl glow::Context, width: usize, wave_speed: f32) -> Self {
        let water = Water::new(width, wave_speed);
        Self {
            normal_texture: GlTexture::new(gl, &water.normal_texture()),
            water,
        }
    }

    pub fn wave_speed_mut(&mut self) -> &mut f32 {
        &mut self.water.wave_speed
    }

    pub fn update(&mut self, delta: Duration) {
        self.water.update(delta);
        let texture = self.water.normal_texture();
        self.normal_texture.load(&texture)
    }

    pub fn normal_texture(&self) -> &GlTexture {
        &self.normal_texture
    }
}
