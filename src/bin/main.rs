// Windows Powershell:
// cargo run --release | out-file output.ppm -encoding ascii
use rust_raytracing::*;

fn main() {
    let (mut camera, world) = final_scene();
    camera.render(&world);
}
