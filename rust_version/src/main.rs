mod camera;
mod color;
mod config;
mod constant;
mod dvec3;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod point3;
mod ray;
mod sphere;

use std::fs;
use std::io;
use std::io::Write;
use std::sync::Arc;

use camera::Camera;
use color::Color;
use color::ColorExt;
use glam::DVec3;
use hittable_list::HittableList;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use point3::Point3;
use ray::Ray;
use sphere::Sphere;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1./1.33));
    let material_right = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2),1.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = config::ASPECT_RATIO;
    cam.image_width = config::IMAGE_WIDTH;
    cam.samples_per_pixel = config::SAMPLES_PER_PIXEL;
    cam.max_depth = config::MAX_DEPTH;
    cam.render(&world)
}
