use crate::*;

pub struct ImageTexture {
    pub image: Image,
}

impl ImageTexture {
    pub fn from(file_name: &str) -> Self {
        Self {
            image: Image::from(file_name),
        }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let x = u.clamp(0.0, 1.0) * self.image.width() as f64;
        let y = (1.0 - v.clamp(0.0, 1.0)) * self.image.height() as f64;
        self.image.pixel(x as u32, y as u32) / MAX_COLOR as f64
    }
}
