mod color;
mod config;
mod hit_record;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;

use std::fs;
use std::io;
use std::io::Write;
use std::sync::Arc;

use color::ColorExt;
use glam::DVec3;
use hittable_list::HittableList;
use point3::Point3;
use ray::Ray;
use ray::ray_color;
use sphere::Sphere;

fn main() -> io::Result<()> {
    let mut image_height = (config::IMAGE_WIDTH as f64 / config::ASPECT_RATIO) as usize;
    if image_height < 1 {
        image_height = 1;
    }

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (config::IMAGE_WIDTH as f64 / image_height as f64);
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = DVec3::new(viewport_width, 0., 0.);
    let viewport_v = DVec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / config::IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - DVec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = fs::File::create("output.ppm")?;

    let mut writer = io::BufWriter::new(file);

    write!(
        writer,
        "P3\n{} {}\n255\n",
        config::IMAGE_WIDTH,
        image_height
    )?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..config::IMAGE_WIDTH {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, pixel_center);
            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color(&mut writer)?;
        }
    }

    writer.flush()?;
    print!("\rDone.                 \n");

    Ok(())
}
