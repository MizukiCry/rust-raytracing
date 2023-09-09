// // cargo run --bin 6-2-normal | out-file output.ppm -encoding ascii
// use rust_raytracing::ray::Ray;
// use rust_raytracing::vec3::{print_color, Vec3};

// fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
//     let oc = ray.origin - center;
//     let a = ray.direction.length_squared();
//     let h = ray.direction.dot(oc);
//     let c = oc.dot(oc) - radius * radius;
//     let discriminant = h * h - a * c;
//     if discriminant.is_sign_negative() {
//         -1.0
//     } else {
//         (-h - discriminant.sqrt()) / a
//     }
// }

// fn ray_color(ray: Ray) -> Vec3 {
//     let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
//     if t.is_sign_positive() {
//         let normal = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
//         return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
//     }
//     let a = 0.5 * (ray.direction.unit().y + 1.0);
//     (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
// }

// fn main() {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const MAX_COLOR: i32 = 255;

//     let image_width = 500;
//     let image_height = ((image_width as f64 / ASPECT_RATIO) as i32).max(1);

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
//             let color = ray_color(ray);
//             print_color(color);
//         }
//     }
//     eprintln!("Done. [{image_height} lines]");
// }
fn main() {}
