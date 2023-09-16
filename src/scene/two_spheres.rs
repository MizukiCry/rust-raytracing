use crate::*;

pub fn two_spheres() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.initialize();

    let mut world = HittableList::default();
    let texture = Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(
        0.8,
        Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
    ))));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::clone(&texture) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::clone(&texture) as Rc<dyn Material>,
    )));

    (camera, world)
}
