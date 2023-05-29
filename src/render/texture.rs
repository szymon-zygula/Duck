use image::{DynamicImage, RgbaImage};

pub struct Texture {
    pub image: DynamicImage,
}

impl Texture {
    pub fn from_file(path: &std::path::Path) -> Self {
        let image = image::io::Reader::open(path)
            .expect("Failed to load texture")
            .decode()
            .expect("Failed to decode texture");

        Self { image }
    }

    pub fn new_rgba(width: u32, height: u32) -> Self {
        let image_buffer = RgbaImage::new(width, height);
        let image = DynamicImage::ImageRgba8(image_buffer);

        Self { image }
    }
}
