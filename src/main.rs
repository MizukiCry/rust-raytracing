// cargo run --release | out-file output.ppm -encoding ascii
use rust_raytracing::utils::*;

fn main() {
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.vfov = 20.0;
    camera.samples_per_pixel = 20;
    camera.max_bounce = 5;
    camera.image_width = 1280;
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.focus_dist = 10.0;
    camera.defocus_angle = 0.6;
    camera.initialize();

    let mut world = HittableList::default();

    let ground_material: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<Box<dyn Material>> = match choose_material {
                    x if x < 0.8 => {
                        Rc::new(Box::new(Lambertian::new(Vec3::random() * Vec3::random())))
                    }
                    x if x < 0.95 => Rc::new(Box::new(Metal::new(
                        Vec3::random_range(0.5, 1.0),
                        random_range(0.0, 0.5),
                    ))),
                    _ => Rc::new(Box::new(Dielectric::new(1.5))),
                };
                world.add(Rc::new(Box::new(Sphere::new(center, 0.2, sphere_material))));
            }
        }
    }

    let material1: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));
    let material2: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));
    let material3: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    camera.render(&world);
}
