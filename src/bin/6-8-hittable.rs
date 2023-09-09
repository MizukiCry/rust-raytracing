// // cargo run --bin 6-8-hittable | out-file output.ppm -encoding ascii
// use std::rc::Rc;

// use rust_raytracing::utils::*;

// fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
//     let mut record = HitRecord::default();
//     if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
//         return 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
//     }
//     let a = 0.5 * (ray.direction.unit().y + 1.0);
//     (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
// }

// fn main() {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const MAX_COLOR: i32 = 255;
//     let image_width = 500;
//     let image_height = ((image_width as f64 / ASPECT_RATIO) as i32).max(1);

//     let mut world = HittableList::default();
//     world.add(Rc::new(Box::new(Sphere::new(
//         Vec3::new(0.0, 0.0, -1.0),
//         0.5,
//     ))));
//     world.add(Rc::new(Box::new(Sphere::new(
//         Vec3::new(0.0, -100.5, -1.0),
//         100.0,
//     ))));

//     let viewport_height = 2.0;
//     let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
//     let focal_length = 1.0;
//     let camera_center = Vec3::new(0.0, 0.0, 0.0);

//     let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
//     let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
//     let pixel_delta_u = viewport_u / (image_width as f64);
//     let pixel_delta_v = viewport_v / (image_height as f64);

//     let viewport_upper_left =
//         camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
//     let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

//     println!("P3\n{image_width} {image_height}\n{MAX_COLOR}");

//     for i in 0..image_height {
//         eprintln!("Progressing... [{} / {image_height}]", i + 1);
//         for j in 0..image_width {
//             let pixel_center = pixel00 + (j as f64) * pixel_delta_u + (i as f64) * pixel_delta_v;
//             let ray_direction = pixel_center - camera_center;
//             let ray = Ray::new(camera_center, ray_direction);
//             let color = ray_color(&ray, &world);
//             print_color(color);
//         }
//     }
//     eprintln!("Done. [{image_height} lines]");
// }
fn main() {}
