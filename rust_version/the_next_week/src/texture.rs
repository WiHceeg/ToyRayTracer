use std::sync::Arc;

use image::RgbImage;

use crate::color::Color;
use crate::interval::Interval;
use crate::point3::Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> SolidColor {
        SolidColor { albedo: albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: crate::point3::Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_interger = (self.inv_scale * p.x).floor() as isize;
        let y_interger = (self.inv_scale * p.y).floor() as isize;
        let z_interger = (self.inv_scale * p.z).floor() as isize;
        if (x_interger + y_interger + z_interger) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    data: RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> anyhow::Result<ImageTexture> {
        let img  = image::open(filename)?.into_rgb8();
        Ok(ImageTexture { data: img})
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let itv = Interval::new(0.0, 1.0);
        let cord_u = itv.clamp(u);
        let cord_v = 1.0 - itv.clamp(v);
        
        let i = (cord_u * self.data.width() as f64) as u32;
        let j = (cord_v * self.data.height() as f64) as u32;
        let pixel = self.data.get_pixel(i, j);
        let color_scale = 1.0/255.0;
        Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
    }
}