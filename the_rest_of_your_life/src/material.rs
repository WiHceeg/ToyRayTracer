use std::sync::Arc;

use glam::DVec3;

use crate::color::Color;
use crate::dvec3::DVec3Ext;
use crate::hit_record::HitRecord;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Option<Color> {
        None
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_from_solid_color(albedo: Color) -> Lambertian {
        Lambertian {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Lambertian {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.unit_normal + DVec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.unit_normal;
        }
        let scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = r_in.direction().reflect(rec.unit_normal);
        reflected = reflected.normalize() + self.fuzz * DVec3::random_unit();
        let scattered = Ray::new_with_time(rec.p, reflected, r_in.time());
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.unit_normal) > 0. {
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
        Dielectric {
            refraction_index,
        }
    }

    // 反射比，Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ONE;
        let ri = if rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_r_in_direction = r_in.direction().normalize();

        let cos_theta = (-unit_r_in_direction).dot(rec.unit_normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;
        let r_out_direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand::random::<f64>() {
                unit_r_in_direction.reflect(rec.unit_normal)
            } else {
                unit_r_in_direction.refract(rec.unit_normal, ri)
            };

        let scattered = Ray::new_with_time(rec.p, r_out_direction, r_in.time());
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>
}


impl DiffuseLight {
    pub fn new_from_solid_color(emit: Color) -> DiffuseLight {
        DiffuseLight {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { tex }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Option<Color> {
        Some(self.tex.value(u, v, p))
    }
}

pub struct Isotropic{
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_from_solid_color(albedo: Color) -> Isotropic {
        Isotropic {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new_with_time(rec.p, DVec3::random_unit(), r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}