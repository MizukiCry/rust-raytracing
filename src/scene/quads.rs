use crate::*;

pub fn quads() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.vfov = 80.0;
    camera.camera_center = Vec3::new(0.0, 0.0, 9.0);
    camera.initialize();

    let mut world = HittableList::default();

    let left_red = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Vec3::new(
        1.0, 0.2, 0.2,
    )))));
    let back_green = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Vec3::new(
        0.2, 1.0, 0.2,
    )))));
    let right_blue = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Vec3::new(
        0.2, 0.2, 1.0,
    )))));
    let upper_orange = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Vec3::new(
        1.0, 0.5, 0.0,
    )))));
    let lower_teal = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Vec3::new(
        0.2, 0.8, 0.8,
    )))));

    world.add(Rc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    (camera, world)
}
