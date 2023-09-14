// Windows Powershell:
// cargo run --release | out-file output.ppm -encoding ascii
use rust_raytracing::*;

fn main() {
    let (mut camera, world) = earth();
    camera.render(&world);
}

#[allow(dead_code)]
fn random_spheres() -> (Camera, BvhNode) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.vfov = 20.0;
    camera.samples_per_pixel = 10;
    camera.max_bounce = 5;
    camera.image_width = 480;
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.focus_dist = 10.0;
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
                    x if x < 0.8 => Rc::new(Lambertian::new(Rc::new(SolidColor::new(
                        Vec3::random() * Vec3::random(),
                    )))),
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
    let material2: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(SolidColor::new(
        Vec3::new(0.4, 0.2, 0.1),
    ))));
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

#[allow(dead_code)]
fn two_spheres() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.vfov = 20.0;
    camera.samples_per_pixel = 100;
    camera.max_bounce = 50;
    camera.image_width = 480;
    camera.camera_center = Vec3::new(13.0, 2.0, 3.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.focus_dist = 10.0;
    camera.defocus_angle = 0.0;
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

#[allow(dead_code)]
fn earth() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.vfov = 20.0;
    camera.samples_per_pixel = 100;
    camera.max_bounce = 10;
    camera.image_width = 480;
    camera.camera_center = Vec3::new(0.0, 0.0, 12.0);
    camera.lookat = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.focus_dist = 10.0;
    camera.defocus_angle = 0.0;
    camera.initialize();

    let mut world = HittableList::default();

    let erath_texture = Rc::new(ImageTexture::from("img/earthmap.jpg"));
    let earth_material = Rc::new(Lambertian::new(Rc::clone(&erath_texture) as Rc<dyn Texture>));
    world.add(Rc::new(Sphere::new(
        Vec3::default(),
        2.0,
        Rc::clone(&earth_material) as Rc<dyn Material>,
    )));

    (camera, world)
}
