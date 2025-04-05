use glam::DVec3;
use rayon::prelude::*;

use std::fs;
use std::io;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time;

use crate::color::Color;
use crate::color::ColorExt;
use crate::config;
use crate::constant;
use crate::dvec3::DVec3Ext;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::random_number_generator::{random, random_range};

use crate::ray::Ray;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: usize,       // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene
    pub background: Color, // Scene background color
    pub enable_gradient_sky: bool,  // sky color

    pub vfov: f64, // Vertical view angle (field of view)
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: DVec3,

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: usize,      // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    sqrt_spp: usize,             // Square root of number of samples per pixel
    recip_sqrt_spp: f64,       // 1 / sqrt_spp
    
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: DVec3,     // Offset to pixel to the right
    pixel_delta_v: DVec3,     // Offset to pixel below

    defocus_disk_u: DVec3, // Defocus disk horizontal radius
    defocus_disk_v: DVec3, // Defocus disk vertical radius

    // Camera frame basis vectors
    u: DVec3,
    v: DVec3,
    w: DVec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) -> anyhow::Result<()> {
        let start_time = time::Instant::now();
        self.initialize();

        let file = fs::File::create("output.ppm")?;

        let mut writer = io::BufWriter::new(file);

        write!(
            writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        let counter = AtomicUsize::new(0);
        let pixels: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let mut row = Vec::with_capacity(self.image_width);
                for i in 0..self.image_width {
                    let mut pixel_color = Color::ZERO;

                    for s_j in 0.. self.sqrt_spp {
                        for s_i in 0.. self.sqrt_spp {
                            let r = self.get_ray(i, j, s_i, s_j);
                            pixel_color += self.ray_color(&r, self.max_depth, world);
                        }
                    }
                    row.push(self.pixel_samples_scale * pixel_color);
                }
                let finished_count = counter.fetch_add(1, Ordering::Relaxed) + 1;
                print!("\rScanlines remaining: {} ", self.image_height - finished_count);
                io::stdout().flush().expect("Failed to flush stdout");
                row
            })
            .collect();

        for row in pixels {
            for color in row {
                color.write_color(&mut writer)?;
            }
        }

        writer.flush()?;
        print!("\rDone.                 \n");
        let duration = start_time.elapsed();
        println!("Cost {:?}.", duration);
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as usize;
        self.pixel_samples_scale = 1. / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1. / (self.sqrt_spp as f64);

        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly
    // sampled point around the pixel location i, j for stratified sample square s_i, s_j.
    fn get_ray(&self, i: usize, j: usize, s_i: usize, s_j: usize) -> Ray {
        let (offset_x, offset_y) = self.sample_square_stratified(s_i, s_j);

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset_x) * self.pixel_delta_u)
            + ((j as f64 + offset_y) * self.pixel_delta_v);

        // 虚化原理是，从圆盘上无论哪个点往焦平面发射，一定能命中焦平面的目标点，因此焦平面最清晰
        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random();
        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: usize, s_j: usize) -> (f64, f64) {
        let px = (s_i as f64 + random()) / self.sqrt_spp as f64 - 0.5;
        let py = (s_j as f64 + random()) / self.sqrt_spp as f64 - 0.5;
        (px, py)
    }

    fn sample_square() -> (f64, f64) {
        (random_range(-0.5..0.5), random_range(-0.5..0.5))
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Point3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    pub fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::ZERO;
        }

        let Some(rec) = world.hit(r, Interval::new(constant::RAY_MIN_DISTANCE, f64::INFINITY)) else {
            return if self.enable_gradient_sky {
                // 没击中，背景色，这里可以理解成天空的颜色
                let unit_direction = r.direction().normalize();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * config::SKY_GRADIENT
            } else {
                self.background
            };
        };
        
        let color_from_emission_opt = rec.mat.emitted(rec.u, rec.v, rec.p);
        
        // 如果能散射，计算散射颜色
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);
            return color_from_emission_opt.unwrap_or(Color::ZERO) + color_from_scatter;
        }
        // 不能散射，只返回发射颜色（如果有）
        color_from_emission_opt.unwrap_or(Color::ZERO)
    }
}
