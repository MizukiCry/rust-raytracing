// // cargo run --bin 2-outputppm | out-file output.ppm -encoding ascii
// use rust_raytracing::vec3::{Vec3, print_color};

// fn main() {
//     const HEIGHT: i32 = 720;
//     const WIDTH: i32 = 1280;
//     const MAX_COLOR: i32 = 255;

//     println!("P3\n{WIDTH} {HEIGHT}\n{MAX_COLOR}");

//     for i in 0..HEIGHT {
//         eprintln!("Progressing... [{} / {HEIGHT}]", i + 1);
//         for j in 0..WIDTH {
//             let red = ((i as f64) / (HEIGHT as f64) * (MAX_COLOR as f64)) as i32;
//             let green = ((j as f64) / (WIDTH as f64) * (MAX_COLOR as f64)) as i32;
//             let blue = 0;
//             let color = Vec3::new(red as f64, green as f64, blue as f64);
//             print_color(color);
//         }
//     }
//     eprintln!("Done. [{HEIGHT} lines]");
// }
fn main() {}
