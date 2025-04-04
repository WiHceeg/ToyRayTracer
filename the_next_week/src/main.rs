mod aabb;
mod bvh;
mod camera;
mod color;
mod config;
mod config_bouncing_spheres;
mod config_checkered_spheres;
mod config_cornell_box;
mod config_cornell_smoke;
mod config_earth;
mod config_final_scene;
mod config_perlin_spheres;
mod config_shapes;
mod config_simple_light;
mod constant;
mod constant_medium;
mod dvec3;
mod enums;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod point3;
mod shape;
mod transform;
mod ray;
mod sphere;
mod texture;

use std::sync::Arc;

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use constant_medium::ConstantMedium;
use dvec3::DVec3Ext;
use enums::Scene;
use glam::DVec3;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use point3::Point3;
use shape::{Annulus, Ellipse, Quad, Tri};
use rand::Rng;
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};
use transform::{RotateY, Translate};

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
                    let albedo = Color::random_range_with_rng(0.5, 1.0, &mut rng);
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
    cam.background = config_bouncing_spheres::BACKGROUND;
    cam.enable_gradient_sky = config_bouncing_spheres::ENABLE_GRADIENT_SKY;

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
    cam.background = config_checkered_spheres::BACKGROUND;
    cam.enable_gradient_sky = config_checkered_spheres::ENABLE_GRADIENT_SKY;

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
    cam.background = config_earth::BACKGROUND;
    cam.enable_gradient_sky = config_earth::ENABLE_GRADIENT_SKY;

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
    cam.background = config_perlin_spheres::BACKGROUND;
    cam.enable_gradient_sky = config_perlin_spheres::ENABLE_GRADIENT_SKY;

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
    cam.aspect_ratio = config_shapes::ASPECT_RATIO;
    cam.image_width = config_shapes::IMAGE_WIDTH;
    cam.samples_per_pixel = config_shapes::SAMPLES_PER_PIXEL;
    cam.max_depth = config_shapes::MAX_DEPTH;
    cam.background = config_shapes::BACKGROUND;
    cam.enable_gradient_sky = config_shapes::ENABLE_GRADIENT_SKY;

    cam.vfov = config_shapes::V_FOV;
    cam.lookfrom = config_shapes::LOOKFROM;
    cam.lookat = config_shapes::LOOKAT;
    cam.vup = config_shapes::V_UP;

    cam.defocus_angle = config_shapes::DEFOCUS_ANGLE;
    cam.focus_dist = config_shapes::FOCUS_DIST;

    cam.render(&world)

}


fn shapes() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new_from_solid_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_from_solid_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_from_solid_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Annulus::new(Point3::new(-3.0, 0.0, 2.5), DVec3::new(0.0, 0.0, -2.0), DVec3::new(0.0, 1.0, 0.0), left_red)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), DVec3::new(4.0, 0.0, 0.0), DVec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Arc::new(Tri::new(Point3::new(3.0, -2.0, 1.0), DVec3::new(0.0, 0.0, 4.0), DVec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Arc::new(Ellipse::new(Point3::new(0.0, 3.0, 2.5), DVec3::new(3.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 1.5), upper_orange)));
    world.add(Arc::new(Ellipse::new(Point3::new(0.0, -3.0, 2.5), DVec3::new(2.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -2.0), lower_teal)));


    let mut cam = Camera::default();
    cam.aspect_ratio = config_shapes::ASPECT_RATIO;
    cam.image_width = config_shapes::IMAGE_WIDTH;
    cam.samples_per_pixel = config_shapes::SAMPLES_PER_PIXEL;
    cam.max_depth = config_shapes::MAX_DEPTH;
    cam.background = config_shapes::BACKGROUND;
    cam.enable_gradient_sky = config_shapes::ENABLE_GRADIENT_SKY;

    cam.vfov = config_shapes::V_FOV;
    cam.lookfrom = config_shapes::LOOKFROM;
    cam.lookat = config_shapes::LOOKAT;
    cam.vup = config_shapes::V_UP;

    cam.defocus_angle = config_shapes::DEFOCUS_ANGLE;
    cam.focus_dist = config_shapes::FOCUS_DIST;

    cam.render(&world)

}

fn simple_light() -> anyhow::Result<()> {
    let mut world = HittableList::new();
    
    let perlin_texture = Arc::new(NoiseTexture::new(config_simple_light::INPUT_POINT_SCALE));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(perlin_texture.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_from_texture(perlin_texture)),
    )));

    let difflight = Arc::new(DiffuseLight::new_from_solid_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    world.add(Arc::new(Quad::new(Point3::new(3.0, 1.0, -2.0), DVec3::new(2.0, 0.0, 0.0), DVec3::new(0.0, 2.0, 0.0), difflight)));
    let mut cam = Camera::default();
    cam.aspect_ratio = config_simple_light::ASPECT_RATIO;
    cam.image_width = config_simple_light::IMAGE_WIDTH;
    cam.samples_per_pixel = config_simple_light::SAMPLES_PER_PIXEL;
    cam.max_depth = config_simple_light::MAX_DEPTH;
    cam.background = config_simple_light::BACKGROUND;
    cam.enable_gradient_sky = config_simple_light::ENABLE_GRADIENT_SKY;

    cam.vfov = config_simple_light::V_FOV;
    cam.lookfrom = config_simple_light::LOOKFROM;
    cam.lookat = config_simple_light::LOOKAT;
    cam.vup = config_simple_light::V_UP;

    cam.defocus_angle = config_simple_light::DEFOCUS_ANGLE;
    cam.focus_dist = config_simple_light::FOCUS_DIST;

    cam.render(&world)    
}

fn cornell_box() -> anyhow::Result<()> {
    let mut world = HittableList::new();
    
    let red = Arc::new(Lambertian::new_from_solid_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_solid_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_solid_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_solid_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), DVec3::new(0.0, 0.0, 555.0), green)));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), DVec3::new(0.0, 0.0, 555.0), red)));
    world.add(Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), DVec3::new(-130.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -105.0), light)));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), DVec3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), DVec3::new(-555.0, 0.0, 0.0), DVec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), DVec3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), white.clone())));
    
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::cuboid(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, DVec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::cuboid(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, DVec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::default();
    cam.aspect_ratio = config_cornell_box::ASPECT_RATIO;
    cam.image_width = config_cornell_box::IMAGE_WIDTH;
    cam.samples_per_pixel = config_cornell_box::SAMPLES_PER_PIXEL;
    cam.max_depth = config_cornell_box::MAX_DEPTH;
    cam.background = config_cornell_box::BACKGROUND;
    cam.enable_gradient_sky = config_cornell_box::ENABLE_GRADIENT_SKY;

    cam.vfov = config_cornell_box::V_FOV;
    cam.lookfrom = config_cornell_box::LOOKFROM;
    cam.lookat = config_cornell_box::LOOKAT;
    cam.vup = config_cornell_box::V_UP;

    cam.defocus_angle = config_cornell_box::DEFOCUS_ANGLE;
    cam.focus_dist = config_cornell_box::FOCUS_DIST;

    cam.render(&world)    
}


fn cornell_smoke() -> anyhow::Result<()> {
    let mut world = HittableList::new();
    
    let red = Arc::new(Lambertian::new_from_solid_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_solid_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_solid_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_solid_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), DVec3::new(0.0, 0.0, 555.0), green)));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), DVec3::new(0.0, 0.0, 555.0), red)));
    world.add(Arc::new(Quad::new(Point3::new(113.0, 554.0, 127.0), DVec3::new(330.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 305.0), light)));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 555.0, 0.0), DVec3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), DVec3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), DVec3::new(555.0, 0.0, 0.0), DVec3::new(0.0, 555.0, 0.0), white.clone())));
    
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::cuboid(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, DVec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::cuboid(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, DVec3::new(130.0, 0.0, 65.0)));
    
    world.add(Arc::new(ConstantMedium::new_from_solid_color(box1, 0.01, Color::ZERO)));
    world.add(Arc::new(ConstantMedium::new_from_solid_color(box2, 0.01, Color::ONE)));

    let mut cam = Camera::default();
    cam.aspect_ratio = config_cornell_smoke::ASPECT_RATIO;
    cam.image_width = config_cornell_smoke::IMAGE_WIDTH;
    cam.samples_per_pixel = config_cornell_smoke::SAMPLES_PER_PIXEL;
    cam.max_depth = config_cornell_smoke::MAX_DEPTH;
    cam.background = config_cornell_smoke::BACKGROUND;
    cam.enable_gradient_sky = config_cornell_smoke::ENABLE_GRADIENT_SKY;

    cam.vfov = config_cornell_smoke::V_FOV;
    cam.lookfrom = config_cornell_smoke::LOOKFROM;
    cam.lookat = config_cornell_smoke::LOOKAT;
    cam.vup = config_cornell_smoke::V_UP;

    cam.defocus_angle = config_cornell_smoke::DEFOCUS_ANGLE;
    cam.focus_dist = config_cornell_smoke::FOCUS_DIST;

    cam.render(&world)    
}

fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) -> anyhow::Result<()> {

    // 地面绿色盒子阵列，高度随机
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_from_solid_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side  = 20;
    let mut rng = rand::rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 1.0;
            let x1 = x0 + w;
            let y1 = rng.random_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Quad::cuboid(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground.clone())));
        }
    }
    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::new(boxes1)));

    // 光源
    let light = Arc::new(DiffuseLight::new_from_solid_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(Point3::new(123.0,554.0,147.0), DVec3::new(300.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 256.0), light)));

    // 运动模糊的橙黄色球
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + DVec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new_from_solid_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(center1, center2, 50.0, sphere_material)));

    // 玻璃球
    world.add(Arc::new(Sphere::new_static(Point3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
    
    // 金属球
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)))));

    // 蓝色玻璃浓雾球
    let mut boundary = Arc::new(Sphere::new_static(Point3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_from_solid_color(boundary, 0.2, Color::new(0.2, 0.4, 0.9))));
    
    // 全局白色薄雾
    boundary = Arc::new(Sphere::new_static(Point3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    world.add(Arc::new(ConstantMedium::new_from_solid_color(boundary, 0.0001, Color::new(1.0, 1.0, 1.0))));

    // 地球
    let emat = Arc::new(Lambertian::new_from_texture(Arc::new(ImageTexture::new("earthmap.jpg")?)));
    world.add(Arc::new(Sphere::new_static(Point3::new(400.0, 200.0, 400.0), 100.0, emat)));
    
    // 噪声纹理球
    let perlin_texture = Arc::new(NoiseTexture::new(config_final_scene::INPUT_POINT_SCALE));
    world.add(Arc::new(Sphere::new_static(Point3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new_from_texture(perlin_texture)))));

    // 随机小球群组成的立方体
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new_from_solid_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new_static(Point3::random_range_with_rng(0.0, 165.0, &mut rng), 10.0, white.clone())));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(boxes2)), 15.0)),
        DVec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = config_final_scene::ASPECT_RATIO;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = config_final_scene::BACKGROUND;
    cam.enable_gradient_sky = config_final_scene::ENABLE_GRADIENT_SKY;

    cam.vfov = config_final_scene::V_FOV;
    cam.lookfrom = config_final_scene::LOOKFROM;
    cam.lookat = config_final_scene::LOOKAT;
    cam.vup = config_final_scene::V_UP;

    cam.defocus_angle = config_final_scene::DEFOCUS_ANGLE;
    cam.focus_dist = config_final_scene::FOCUS_DIST;

    cam.render(&world)    
}

fn main() {
    let res = match config::TARGET_SCENE {
        Scene::BouncingSpheres => bouncing_spheres(),
        Scene::CheckeredSpheres => checkered_spheres(),
        Scene::Earth => earth(),
        Scene::PerlinSpheres => perlin_spheres(),
        Scene::Quads => quads(),
        Scene::Shapes => shapes(),
        Scene::SimpleLight => simple_light(),
        Scene::CornellBox => cornell_box(),
        Scene::CornellSmoke => cornell_smoke(),
        Scene::FinalSceneLD => final_scene(config_final_scene::IMAGE_WIDTH_LD, config_final_scene::SAMPLES_PER_PIXEL_LD, config_final_scene::MAX_DEPTH_LD),
        Scene::FinalSceneHD => final_scene(config_final_scene::IMAGE_WIDTH_HD, config_final_scene::SAMPLES_PER_PIXEL_HD, config_final_scene::MAX_DEPTH_HD),
    };

    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
