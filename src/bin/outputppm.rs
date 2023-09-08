// cargo run --bin outputppm | out-file output.ppm -encoding ascii
fn main() {
    const HEIGHT: i32 = 720;
    const WIDTH: i32 = 1280;
    const MAX_COLOR: i32 = 255;

    println!("P3\n{WIDTH} {HEIGHT}\n{MAX_COLOR}");

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let red = ((i as f64) / (HEIGHT as f64) * (MAX_COLOR as f64)) as i32;
            let green = ((j as f64) / (WIDTH as f64) * (MAX_COLOR as f64)) as i32;
            let blue = 0;
            println!("{red} {green} {blue}");
        }
    }
}
