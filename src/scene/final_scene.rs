use crate::*;

pub fn final_scene() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 1.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 1000;
    camera.max_bounce = 10;
    camera.background = Vec3::default();
    camera.vfov = 40.0;
    camera.camera_center = Vec3::new(478.0, 278.0, -600.0);
    camera.lookat = Vec3::new(278.0, 278.0, 0.0);
    camera.initialize();

    let mut world = HittableList::default();

    let mut boxes1 = HittableList::default();
    let ground: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.48, 0.83, 0.53)));
    const BOXES_PER_SIDE: i32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = i as f64 * w - 1000.0;
            let y0 = 0.0;
            let z0 = j as f64 * w - 1000.0;
            let x1 = x0 + w;
            let y1 = random_range_f64(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Rc::new(Quad::new_box(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Rc::clone(&ground),
            )));
        }
    }
    world.add(Rc::new(BvhNode::from(&mut boxes1.objects)));

    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Rc::clone(&light),
    )));

    let sphere_material: Rc<dyn Material> =
        Rc::new(Lambertian::from_color(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new_moving(
        Vec3::new(400.0, 400.0, 200.0),
        Vec3::new(430.0, 400.0, 200.0),
        50.0,
        Rc::clone(&sphere_material),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary: Rc<dyn Hittable> = Rc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::clone(&boundary));
    world.add(Rc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    world.add(Rc::new(ConstantMedium::from_color(
        Rc::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            5000.0,
            Rc::new(Dielectric::new(1.5)),
        )),
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let earth_material = Rc::new(Lambertian::new(Rc::new(ImageTexture::from(
        "img/earthmap.jpg",
    ))));
    world.add(Rc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new(Rc::new(NoiseTexture::new(0.1)))),
    )));

    let mut boxes2 = HittableList::default();
    let white: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    for _ in 0..1000 {
        boxes2.add(Rc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            Rc::clone(&white),
        )));
    }
    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BvhNode::from(&mut boxes2.objects)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (camera, world)
}
