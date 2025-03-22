use glam::DVec3;

use crate::color::Color;
use crate::dvec3::DVec3Ext;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color, // 反照率
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + DVec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz: fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = r_in.direction().reflect(rec.normal);
        reflected = reflected.normalize() + self.fuzz * DVec3::random_unit();
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
