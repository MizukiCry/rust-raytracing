use crate::*;

pub fn two_perlin_spheres() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.initialize();

    let mut world = HittableList::default();

    let material = Rc::new(Lambertian::new(Rc::new(NoiseTexture::new(4.0))));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&material) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::clone(&material) as Rc<dyn Material>,
    )));

    (camera, world)
}
