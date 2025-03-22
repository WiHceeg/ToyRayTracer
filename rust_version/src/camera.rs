use glam::DVec3;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::time;

use crate::color::Color;
use crate::color::ColorExt;
use crate::config;
use crate::dvec3::DVec3Ext;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene

    image_height: usize,      // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: DVec3,     // Offset to pixel to the right
    pixel_delta_v: DVec3,     // Offset to pixel below
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        let start_time = time::Instant::now();
        self.initialize();

        let file = fs::File::create("output.ppm")?;

        let mut writer = io::BufWriter::new(file);

        write!(
            writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                // let pixel_center = self.pixel00_loc
                //     + i as f64 * self.pixel_delta_u
                //     + j as f64 * self.pixel_delta_v;
                // let ray_direction = pixel_center - self.center;
                // let r = Ray::new(self.center, pixel_center);
                // let pixel_color = Camera::ray_color(&r, world);
                // pixel_color.write_color(&mut writer)?;

                let mut pixel_color = Color::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, self.max_depth, world);
                }
                (self.pixel_samples_scale * pixel_color).write_color(&mut writer)?;
            }
        }

        writer.flush()?;
        print!("\rDone.                 \n");
        let duration = start_time.elapsed();
        println!("cost {:?}.", duration);
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        self.center = DVec3::ZERO;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = DVec3::new(viewport_width, 0., 0.);
        let viewport_v = DVec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - DVec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    // Construct a camera ray originating from the origin and directed at randomly sampled point around the pixel location i, j.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> DVec3 {
        let mut rng = rand::rng();
        DVec3::new(rng.random_range(-0.5..0.5), rng.random_range(-0.5..0.5), 0.)
    }

    pub fn ray_color(r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(r, Interval::new(config::RAY_MIN_DISTANCE, f64::INFINITY)) {
            let direction = rec.normal + DVec3::random_unit();
            // 0.5，漫反射击中点向外半球的随机向量
            return 0.5 * Camera::ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        // 没击中，背景色，这里可以理解成天空的颜色
        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * config::SKY_GRADIENT
    }
}
