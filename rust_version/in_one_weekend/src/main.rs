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

use std::io;
use std::sync::Arc;

use camera::Camera;
use color::Color;
use dvec3::DVec3Ext;
use hittable_list::HittableList;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use point3::Point3;
use rand::Rng;
use sphere::Sphere;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    // 地面：半径 1000，中心在 (0, -1000, 0)
    let ground_material = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::rng();
    // 生成 -11 到 10 的随机小球
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            // 排除靠近大球的区域
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // 漫反射材质
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(&albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // 金属材质
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // 介质（玻璃）
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // 三个大球
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = config::ASPECT_RATIO;
    cam.image_width = config::IMAGE_WIDTH;
    cam.samples_per_pixel = config::SAMPLES_PER_PIXEL;
    cam.max_depth = config::MAX_DEPTH;

    cam.vfov = config::V_FOV;
    cam.lookfrom = config::LOOKFROM;
    cam.lookat = config::LOOKAT;
    cam.vup = config::V_UP;

    cam.defocus_angle = config::DEFOCUS_ANGLE;
    cam.focus_dist = config::FOCUS_DIST;

    cam.render(&world)
}
