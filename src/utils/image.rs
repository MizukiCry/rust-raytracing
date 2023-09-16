use crate::*;

use ::image::{io::Reader, DynamicImage, GenericImageView};

pub struct Image {
    image: DynamicImage,
}

impl Image {
    pub fn from(file_name: &str) -> Self {
        Self {
            image: Reader::open(file_name).unwrap().decode().unwrap(),
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    // Indexes from 0
    pub fn pixel(&self, x: u32, y: u32) -> Vec3 {
        let pixel = self
            .image
            .get_pixel(x.clamp(0, self.width() - 1), y.clamp(0, self.height() - 1));
        Vec3::new(pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64)
    }
}
