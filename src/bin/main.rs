// Windows Powershell:
// cargo run --release | out-file output.ppm -encoding ascii
use rust_raytracing::*;

fn main() {
    let (mut camera, world, lights) = cornell_box();
    camera.render(&world, &(Rc::new(lights) as Rc<dyn Hittable>));
}
