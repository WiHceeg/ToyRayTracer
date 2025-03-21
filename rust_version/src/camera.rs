use glam::DVec3;
use std::fs;
use std::io;
use std::io::Write;

use crate::color::Color;
use crate::color::ColorExt;
use crate::config;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel

    image_height: usize,  // Rendered image height
    center: Point3,       // Camera center
    pixel00_loc: Point3,  // Location of pixel 0, 0
    pixel_delta_u: DVec3, // Offset to pixel to the right
    pixel_delta_v: DVec3, // Offset to pixel below
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        let file = fs::File::create("output.ppm")?;

        let mut writer = io::BufWriter::new(file);

        write!(
            writer,
            "P3\n{} {}\n255\n",
            config::IMAGE_WIDTH,
            self.image_height
        )?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..config::IMAGE_WIDTH {
                let pixel_center = self.pixel00_loc
                    + i as f64 * self.pixel_delta_u
                    + j as f64 * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, pixel_center);
                let pixel_color = Camera::ray_color(&r, world);
                pixel_color.write_color(&mut writer)?;
            }
        }

        writer.flush()?;
        print!("\rDone.                 \n");

        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (config::IMAGE_WIDTH as f64 / config::ASPECT_RATIO) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = DVec3::ZERO;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (config::IMAGE_WIDTH as f64 / self.image_height as f64);

        let viewport_u = DVec3::new(viewport_width, 0., 0.);
        let viewport_v = DVec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / config::IMAGE_WIDTH as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - DVec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {

        if let Some(rec) = world.hit(r, Interval::new(0., f64::INFINITY)) {
            return 0.5 * (rec.normal + DVec3::splat(1.0));
        }

        // 没击中，背景色，这里可以理解成天空的颜色
        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * config::SKY_GRADIENT
    }
}
