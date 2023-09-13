use std::cmp::Ordering;

use crate::*;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    fn new(left: Rc<dyn Hittable>, right: Rc<dyn Hittable>) -> Self {
        let bounding_box = left.bounding_box().union(right.bounding_box());
        Self {
            left,
            right,
            bounding_box,
        }
    }

    fn x_axis_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        a.bounding_box().x.min.total_cmp(&b.bounding_box().x.min)
    }

    fn y_axis_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        a.bounding_box().y.min.total_cmp(&b.bounding_box().y.min)
    }

    fn z_axis_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        a.bounding_box().z.min.total_cmp(&b.bounding_box().z.min)
    }

    pub fn from(objects: &mut [Rc<dyn Hittable>]) -> Self {
        objects.sort_by(match random_range_i32(0, 2) {
            0 => Self::x_axis_compare,
            1 => Self::y_axis_compare,
            _ => Self::z_axis_compare,
        });
        match objects.len() {
            1 => Self::new(Rc::clone(&objects[0]), Rc::clone(&objects[0])),
            2 => Self::new(Rc::clone(&objects[0]), Rc::clone(&objects[1])),
            len => Self::new(
                Rc::new(Self::from(&mut objects[..len / 2])),
                Rc::new(Self::from(&mut objects[len / 2..])),
            ),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, mut ray_t: Interval, record: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, record);
        if hit_left {
            ray_t.max = record.t;
        }
        let hit_right = self.right.hit(ray, ray_t, record);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
