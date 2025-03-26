mod aabb;
mod bvh;
mod camera;
mod color;
mod config;
mod config_bouncing_spheres;
mod config_checkered_spheres;
mod constant;
mod dvec3;
mod enums;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod point3;
mod ray;
mod sphere;
mod texture;

use std::io;
use std::sync::Arc;

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use dvec3::DVec3Ext;
use glam::DVec3;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use point3::Point3;
use rand::Rng;
use sphere::Sphere;
use texture::CheckerTexture;

fn bouncing_spheres() -> io::Result<()> {
    let mut world = HittableList::new();

    // 地面：半径 1000，中心在 (0, -1000, 0)
    /*
        let ground_material = Arc::new(Lambertian::new_from_solid_color(Color::new(0.5, 0.5, 0.5)));
        world.add(Arc::new(Sphere::new_static(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));
    */
    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(checker)),
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
                    let sphere_material = Arc::new(Lambertian::new_from_solid_color(albedo));
                    let end_center = center + DVec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        end_center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // 金属材质
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, sphere_material)));
                } else {
                    // 介质（玻璃）
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // 三个大球
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_from_solid_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    if config::ENABLE_BVH {
        let bvh_node = BvhNode::new(world);
        world = HittableList::new();
        world.add(Arc::new(bvh_node));
    }

    let mut cam = Camera::default();
    cam.aspect_ratio = config_bouncing_spheres::ASPECT_RATIO;
    cam.image_width = config_bouncing_spheres::IMAGE_WIDTH;
    cam.samples_per_pixel = config_bouncing_spheres::SAMPLES_PER_PIXEL;
    cam.max_depth = config_bouncing_spheres::MAX_DEPTH;

    cam.vfov = config_bouncing_spheres::V_FOV;
    cam.lookfrom = config_bouncing_spheres::LOOKFROM;
    cam.lookat = config_bouncing_spheres::LOOKAT;
    cam.vup = config_bouncing_spheres::V_UP;

    cam.defocus_angle = config_bouncing_spheres::DEFOCUS_ANGLE;
    cam.focus_dist = config_bouncing_spheres::FOCUS_DIST;

    cam.render(&world)
}

fn checkered_spheres() -> io::Result<()> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(checker)),
    )));

    if config::ENABLE_BVH {
        let bvh_node = BvhNode::new(world);
        world = HittableList::new();
        world.add(Arc::new(bvh_node));
    }

    let mut cam = Camera::default();
    cam.aspect_ratio = config_checkered_spheres::ASPECT_RATIO;
    cam.image_width = config_checkered_spheres::IMAGE_WIDTH;
    cam.samples_per_pixel = config_checkered_spheres::SAMPLES_PER_PIXEL;
    cam.max_depth = config_checkered_spheres::MAX_DEPTH;

    cam.vfov = config_checkered_spheres::V_FOV;
    cam.lookfrom = config_checkered_spheres::LOOKFROM;
    cam.lookat = config_checkered_spheres::LOOKAT;
    cam.vup = config_checkered_spheres::V_UP;

    cam.defocus_angle = config_checkered_spheres::DEFOCUS_ANGLE;
    cam.focus_dist = config_checkered_spheres::FOCUS_DIST;

    cam.render(&world)
}

fn main() -> io::Result<()> {
    match config::TARGET_SCENE {
        enums::Scene::BouncingSpheres => bouncing_spheres(),
        enums::Scene::CheckeredSpheres => checkered_spheres(),
    }
}
