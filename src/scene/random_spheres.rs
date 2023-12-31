use crate::*;

pub fn random_spheres() -> (Camera, BvhNode) {
    let mut camera = Camera::default();
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.defocus_angle = 0.6;
    camera.initialize();

    let mut world = HittableList::default();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(
        0.32,
        Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
    ))));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = match choose_material {
                    x if x < 0.8 => {
                        Rc::new(Lambertian::from_color(Vec3::random() * Vec3::random()))
                    }
                    x if x < 0.95 => Rc::new(Metal::new(
                        Vec3::random_range(0.5, 1.0),
                        random_range_f64(0.0, 0.5),
                    )),
                    _ => Rc::new(Dielectric::new(1.5)),
                };
                if choose_material < 0.8 {
                    world.add(Rc::new(Sphere::new_moving(
                        center,
                        center + Vec3::new(0.0, random_range_f64(0.0, 0.5), 0.0),
                        0.2,
                        sphere_material,
                    )));
                } else {
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
                // world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2: Rc<dyn Material> = Rc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let world = BvhNode::from(&mut world.objects);

    (camera, world)
}
