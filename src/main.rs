// cargo run | out-file output.ppm -encoding ascii
use rust_raytracing::utils::*;
use std::rc::Rc;

fn main() {
    let mut camera = Camera::new(16.0 / 9.0, 20, 50, 500);

    let mut world = HittableList::default();
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    ))));

    camera.render(&world);
}
