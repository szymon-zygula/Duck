use crate::primitives::color::ColorAlpha;

pub const WINDOW_TITLE: &str = "Ente";
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub const CLEAR_COLOR: ColorAlpha = ColorAlpha {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};

pub const DUCK_MODEL_PATH: &str = "models/duck.txt";
pub const DUCK_TEXTURE_PATH: &str = "textures/ducktex.jpg";
