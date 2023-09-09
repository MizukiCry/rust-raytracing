use crate::utils::*;

const MAX_COLOR: i32 = 255;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    camera_center: Vec3,
    pixel00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
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
            image_width,
            image_height,
            camera_center,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!(
            "P3\n{} {}\n{}",
            self.image_width, self.image_height, MAX_COLOR
        );

        for i in 0..self.image_height {
            eprintln!("Progressing... [{} / {}]", i + 1, self.image_height);
            for j in 0..self.image_width {
                let pixel_center = self.pixel00
                    + (j as f64) * self.pixel_delta_u
                    + (i as f64) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);
                let color = Self::ray_color(&ray, world);
                print_color(color);
            }
        }
        eprintln!("Done. [{} lines]", self.image_height);
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

    fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
        let mut record = HitRecord::default();
        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
            return 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        let a = 0.5 * (ray.direction.unit().y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 5)
    }
}
