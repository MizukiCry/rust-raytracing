// cargo run | out-file output.ppm -encoding ascii
use rust_raytracing::utils::*;

fn main() {
    let mut camera = Camera::new(16.0 / 9.0, 50, 100, 500);
    let mut world = HittableList::default();

    let material_ground: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
    let material_center: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))));
    let material_left: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    let material_right: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)));

    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_center),
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&material_left),
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    ))));

    camera.render(&world);
}
