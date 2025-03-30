mod aabb;
mod bvh;
mod camera;
mod color;
mod config;
mod config_bouncing_spheres;
mod config_checkered_spheres;
mod config_earth;
mod config_perlin_spheres;
mod config_quads;
mod constant;
mod dvec3;
mod enums;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod point3;
mod quad;
mod ray;
mod sphere;
mod texture;

use std::sync::Arc;

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use dvec3::DVec3Ext;
use glam::DVec3;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use point3::Point3;
use quad::Quad;
use rand::Rng;
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};

fn bouncing_spheres() -> anyhow::Result<()> {
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

fn checkered_spheres() -> anyhow::Result<()> {
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

fn earth() -> anyhow::Result<()> {
    let mut world = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg")?);
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Arc::new(Sphere::new_static(Point3::ZERO, 2.0, earth_surface));
    world.add(globe);

    let mut cam = Camera::default();
    cam.aspect_ratio = config_earth::ASPECT_RATIO;
    cam.image_width = config_earth::IMAGE_WIDTH;
    cam.samples_per_pixel = config_earth::SAMPLES_PER_PIXEL;
    cam.max_depth = config_earth::MAX_DEPTH;

    cam.vfov = config_earth::V_FOV;
    cam.lookfrom = config_earth::LOOKFROM;
    cam.lookat = config_earth::LOOKAT;
    cam.vup = config_earth::V_UP;

    cam.defocus_angle = config_earth::DEFOCUS_ANGLE;
    cam.focus_dist = config_earth::FOCUS_DIST;

    cam.render(&world)
}

fn perlin_spheres() -> anyhow::Result<()> {
    let mut world = HittableList::new();
    let texture = Arc::new(NoiseTexture::new(config_perlin_spheres::INPUT_POINT_SCALE));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(texture.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_from_texture(texture)),
    )));
    let mut cam = Camera::default();
    cam.aspect_ratio = config_perlin_spheres::ASPECT_RATIO;
    cam.image_width = config_perlin_spheres::IMAGE_WIDTH;
    cam.samples_per_pixel = config_perlin_spheres::SAMPLES_PER_PIXEL;
    cam.max_depth = config_perlin_spheres::MAX_DEPTH;

    cam.vfov = config_perlin_spheres::V_FOV;
    cam.lookfrom = config_perlin_spheres::LOOKFROM;
    cam.lookat = config_perlin_spheres::LOOKAT;
    cam.vup = config_perlin_spheres::V_UP;

    cam.defocus_angle = config_perlin_spheres::DEFOCUS_ANGLE;
    cam.focus_dist = config_perlin_spheres::FOCUS_DIST;

    cam.render(&world)
}

fn quads() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new_from_solid_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_from_solid_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), DVec3::new(0.0, 0.0, -4.0), DVec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), DVec3::new(4.0, 0.0, 0.0), DVec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), DVec3::new(0.0, 0.0, 4.0), DVec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), DVec3::new(4.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), DVec3::new(4.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -4.0), lower_teal)));


    let mut cam = Camera::default();
    cam.aspect_ratio = config_quads::ASPECT_RATIO;
    cam.image_width = config_quads::IMAGE_WIDTH;
    cam.samples_per_pixel = config_quads::SAMPLES_PER_PIXEL;
    cam.max_depth = config_quads::MAX_DEPTH;

    cam.vfov = config_quads::V_FOV;
    cam.lookfrom = config_quads::LOOKFROM;
    cam.lookat = config_quads::LOOKAT;
    cam.vup = config_quads::V_UP;

    cam.defocus_angle = config_quads::DEFOCUS_ANGLE;
    cam.focus_dist = config_quads::FOCUS_DIST;

    cam.render(&world)

}

fn main() {
    let res = match config::TARGET_SCENE {
        enums::Scene::BouncingSpheres => bouncing_spheres(),
        enums::Scene::CheckeredSpheres => checkered_spheres(),
        enums::Scene::Earth => earth(),
        enums::Scene::PerlinSpheres => perlin_spheres(),
        enums::Scene::Quads => quads(),
    };

    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
