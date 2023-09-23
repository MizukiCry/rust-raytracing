use crate::*;

pub fn cornell_box() -> (Camera, HittableList, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 1000;
    camera.background = Vec3::default();
    camera.vfov = 40.0;
    camera.camera_center = Vec3::new(278.0, 278.0, -800.0);
    camera.lookat = Vec3::new(278.0, 278.0, 0.0);
    // camera.stratified = false;
    camera.initialize();

    let mut world = HittableList::default();

    let red: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Vec3::new(15.0, 15.0, 15.0)));

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
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Rc::clone(&light),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Rc::clone(&white),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Rc::clone(&white),
    )));
    world.add(Rc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Rc::clone(&white),
    )));

    // world.add(Rc::new(Translate::new(
    //     Rc::new(RotateY::new(
    //         Rc::new(Quad::new_box(
    //             Vec3::new(0.0, 0.0, 0.0),
    //             Vec3::new(165.0, 330.0, 165.0),
    //             Rc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0)),
    //         )),
    //         15.0,
    //     )),
    //     Vec3::new(265.0, 0.0, 295.0),
    // )));
    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(Quad::new_box(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                Rc::clone(&white),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    )));

    // world.add(Rc::new(Translate::new(
    //     Rc::new(RotateY::new(
    //         Rc::new(Quad::new_box(
    //             Vec3::new(0.0, 0.0, 0.0),
    //             Vec3::new(165.0, 165.0, 165.0),
    //             Rc::clone(&white),
    //         )),
    //         -18.0,
    //     )),
    //     Vec3::new(130.0, 0.0, 65.0),
    // )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    let mut lights = HittableList::default();
    lights.add(Rc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Rc::new(DefaultMaterial::default()),
    )));
    lights.add(Rc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Rc::new(DefaultMaterial::default()),
    )));

    (camera, world, lights)
}
