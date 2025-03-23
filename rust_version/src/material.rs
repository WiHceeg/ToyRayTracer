
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

pub struct Dielectric {
    refraction_index: f64, // 折射率或者折射率比例
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index: refraction_index }
    }

    // 反射比，Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0*r0;
        r0 + (1.-r0)*(1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ONE;
        let ri = if rec.front_face { 1. / self.refraction_index} else {self.refraction_index};

        let unit_r_in_direction = r_in.direction().normalize();

        let cos_theta = (-unit_r_in_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;
        let r_out_direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand::random::<f64>() {
            unit_r_in_direction.reflect(rec.normal)
        } else {
            unit_r_in_direction.refract(rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, r_out_direction);
        Some((attenuation, scattered))
    }
}