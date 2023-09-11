use crate::utils::*;

const MAX_COLOR: i32 = 255;

/// Depends on following arguments:
/// `aspect_ratio: f64`, `vfov: f64`, `samples_per_pixel: i32`, `max_bounce: i32`,
/// `image_width: i32`, `camera_center: Vec3`, `lookat: Vec3`, `vup: Vec3`,
/// `focus_dist: f64`, `defocus_angle: f64`
#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub samples_per_pixel: i32,
    pub max_bounce: i32,
    pub image_width: i32,
    pub camera_center: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub focus_dist: f64,
    pub defocus_angle: f64,
    image_height: i32,
    pixel00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);
        let viewport_height = 2.0 * f64::tan(degree_to_radian(self.vfov) / 2.0) * self.focus_dist;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        let w = (self.camera_center - self.lookat).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left =
            self.camera_center - self.focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00 = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * f64::tan(degree_to_radian(self.defocus_angle / 2.0));
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn print_color(mut color: Vec3) {
        // Convert linear space to gamma space.
        color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
        color *= MAX_COLOR as f64 + 1.0;
        println!(
            "{} {} {}",
            (color.x as i32).clamp(0, MAX_COLOR),
            (color.y as i32).clamp(0, MAX_COLOR),
            (color.z as i32).clamp(0, MAX_COLOR)
        );
    }

    pub fn render(&mut self, world: &HittableList) {
        println!(
            "P3\n{} {}\n{}",
            self.image_width, self.image_height, MAX_COLOR
        );

        for i in 0..self.image_height {
            eprintln!("Progressing... [{} / {}]", i + 1, self.image_height);
            for j in 0..self.image_width {
                let mut color = Vec3::default();
                for _ in 0..self.samples_per_pixel {
                    color += Self::ray_color(&self.get_random_ray(i, j), self.max_bounce, world);
                }
                color /= self.samples_per_pixel as f64;
                Self::print_color(color);
            }
        }
        eprintln!("Done. [{} lines]", self.image_height);
    }

    fn get_random_ray(&mut self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00 + (j as f64) * self.pixel_delta_u + (i as f64) * self.pixel_delta_v;
        let pixel_sample = pixel_center
            + random_range(-0.5, 0.5) * self.pixel_delta_u
            + random_range(-0.5, 0.5) * self.pixel_delta_v;
        let ray_origin = if !self.defocus_angle.is_sign_positive() {
            self.camera_center
        } else {
            self.camera_center
                + Vec3::random_in_unit_disk() * (self.defocus_disk_u + self.defocus_disk_v)
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Vec3 {
        if depth == 0 {
            return Vec3::default();
        }
        let mut record = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if record
                .material
                .scatter(ray, &record, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Vec3::default();
        }
        let a = 0.5 * (ray.direction.unit().y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 0.0,
            vfov: 0.0,
            samples_per_pixel: 0,
            max_bounce: 0,
            image_width: 0,
            camera_center: Vec3::default(),
            lookat: Vec3::default(),
            vup: Vec3::default(),
            focus_dist: 3.0,
            defocus_angle: 0.0,
            image_height: 0,
            pixel00: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
