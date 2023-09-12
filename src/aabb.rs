pub mod interval;
pub use interval::*;

use crate::utils::*;

/// Axis-aligned bounding box
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from(a: &Vec3, b: &Vec3) -> Self {
        Self::new(
            Interval::new(a.x.min(b.x), a.x.max(b.x)),
            Interval::new(a.y.min(b.y), a.y.max(b.y)),
            Interval::new(a.z.min(b.z), a.z.max(b.z)),
        )
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let mut inv = 1.0 / ray.direction.x;
        ray_t.overlaps(&Interval::new_adjusted(
            (self.x.min - ray.origin.x) * inv,
            (self.x.max - ray.origin.x) * inv,
        ));
        if ray_t.empty() {
            return false;
        }

        inv = 1.0 / ray.direction.y;
        ray_t.overlaps(&Interval::new_adjusted(
            (self.y.min - ray.origin.y) * inv,
            (self.y.max - ray.origin.y) * inv,
        ));
        if ray_t.empty() {
            return false;
        }

        inv = 1.0 / ray.direction.z;
        ray_t.overlaps(&Interval::new_adjusted(
            (self.z.min - ray.origin.z) * inv,
            (self.z.max - ray.origin.z) * inv,
        ));
        !ray_t.empty()
    }
}
