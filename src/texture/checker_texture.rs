use crate::*;

pub struct CheckerTexture {
    inv_scale: f64,
    even_texture: Rc<dyn Texture>,
    odd_texture: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even_texture: Rc<dyn Texture>, odd_texture: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let x = (p.x * self.inv_scale) as i32;
        let y = (p.y * self.inv_scale) as i32;
        let z = (p.z * self.inv_scale) as i32;
        match (x + y + z) % 2 {
            0 => self.even_texture.color(u, v, p),
            _ => self.odd_texture.color(u, v, p),
        }
    }
}
