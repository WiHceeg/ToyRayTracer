mod color;
mod config;
mod constant;
mod hit_record;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;
mod interval;
mod camera;
mod dvec3;

use std::fs;
use std::io;
use std::io::Write;
use std::sync::Arc;

use camera::Camera;
use color::ColorExt;
use glam::DVec3;
use hittable_list::HittableList;
use point3::Point3;
use ray::Ray;
use sphere::Sphere;

fn main() -> io::Result<()> {

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut cam = Camera::default();
    cam.aspect_ratio = config::ASPECT_RATIO;
    cam.image_width = config::IMAGE_WIDTH;
    cam.render(&world)



}
