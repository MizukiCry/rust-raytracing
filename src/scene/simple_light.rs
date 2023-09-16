use crate::*;

pub fn simple_light() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.camera_center = Vec3::new(26.0, 3.0, 6.0);
    camera.lookat = Vec3::new(0.0, 2.0, 0.0);
    camera.background = Vec3::default();
    camera.initialize();

    let mut world = HittableList::default();

    let perlin_material = Rc::new(Lambertian::new(Rc::new(NoiseTexture::new(4.0))));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&perlin_material) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::clone(&perlin_material) as Rc<dyn Material>,
    )));

    let diffuse_light = Rc::new(DiffuseLight::from_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Rc::clone(&diffuse_light) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Rc::clone(&diffuse_light) as Rc<dyn Material>,
    )));

    (camera, world)
}
