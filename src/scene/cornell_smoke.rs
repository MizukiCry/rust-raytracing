use crate::*;

pub fn cornell_smoke() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.background = Vec3::default();
    camera.vfov = 40.0;
    camera.camera_center = Vec3::new(278.0, 278.0, -800.0);
    camera.lookat = Vec3::new(278.0, 278.0, 0.0);
    camera.initialize();

    let mut world = HittableList::default();

    let red: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Vec3::new(7.0, 7.0, 7.0)));

    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&green),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&red),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        Rc::clone(&light),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&white),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&white),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Rc::clone(&white),
    )));

    world.add(Rc::new(ConstantMedium::from_color(
        Rc::new(Translate::new(
            Rc::new(RotateY::new(
                Rc::new(Quad::new_box(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 330.0, 165.0),
                    Rc::clone(&white),
                )),
                15.0,
            )),
            Vec3::new(265.0, 0.0, 295.0),
        )),
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));

    world.add(Rc::new(ConstantMedium::from_color(
        Rc::new(Translate::new(
            Rc::new(RotateY::new(
                Rc::new(Quad::new_box(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 165.0, 165.0),
                    Rc::clone(&white),
                )),
                -18.0,
            )),
            Vec3::new(130.0, 0.0, 65.0),
        )),
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    (camera, world)
}
