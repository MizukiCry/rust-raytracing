use crate::utils::*;

const MAX_COLOR: i32 = 255;

pub struct Camera {
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_bounce: i32,
    pub image_width: i32,
    pub image_height: i32,
    camera_center: Vec3,
    pixel00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_bounce: i32,
        image_width: i32,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let focal_length = 1.0;
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            samples_per_pixel,
            max_bounce,
            image_width,
            image_height,
            camera_center,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
        }
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
                color = color / (self.samples_per_pixel as f64);
                Self::print_color(color);
            }
        }
        eprintln!("Done. [{} lines]", self.image_height);
    }

    fn get_random_ray(&mut self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00 + (j as f64) * self.pixel_delta_u + (i as f64) * self.pixel_delta_v;
        let pixel_sample = pixel_center
            + random_double(-0.5, 0.5) * self.pixel_delta_u
            + random_double(-0.5, 0.5) * self.pixel_delta_v;
        let ray_direction = pixel_sample - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    /// Initializes according to aspect_ratio and image_width.
    #[allow(dead_code)]
    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);
        let focal_length = 1.0;
        self.camera_center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00 = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(ray: &Ray, depth: i32, world: &HittableList) -> Vec3 {
        if depth == 0 {
            return Vec3::default();
        }
        let mut record = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            let direction = record.normal + Vec3::random_unit();
            return 0.5 * Self::ray_color(&Ray::new(record.p, direction), depth - 1, world);
        }
        let a = 0.5 * (ray.direction.unit().y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 5, 5, 100)
    }
}
