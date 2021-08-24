//
// Created by luosq on 2021/8/21.
//

#include "toy_ray_tracer.h"
#include "color.h"
#include "hittable_list.h"
#include "sphere.h"


#include <iostream>
#include <fstream>
#include <format>

using namespace std;

// 这个函数的 world 是动态绑定，Hittable 类型的引用引用了 HittableList，调用了 HittableList 重写的hit函数
Color rayColor(const Ray &ray, const Hittable& world) {
    HitRecord record;
    if (world.hit(ray, 0, infinity, record)) {
        return 0.5 * (record.normal_ + Color({1,1,1}));
    } else {
        Vec3d unit_direction = vecNormalized(ray.direction());
        double t = 0.5 * (unit_direction.y() + 1.0);
        //线性插值
        return (1.0 - t) * Color({1.0, 1.0, 1.0}) + t * Color({0.5, 0.7, 1.0});
    }
}


int main() {
    ofstream output("image6.7.ppm");

    // Image
    const double aspect_ratio = 16.0 / 9.0;
    const int image_width = 400;
    const int image_height = static_cast<int>(image_width / aspect_ratio);

    // World
    HittableList world;
    world.add(make_shared<Sphere>(Point3d({0,0,-1}), 0.5));
    world.add(make_shared<Sphere>(Point3d({0,-100.5,-1}), 100));    // 下半部分的大球


    // Camera
    double viewport_height = 2.0;
    double viewport_width = aspect_ratio * viewport_height;
    double focal_length = 1.0;

    Point3d origin = Point3d({0, 0, 0});
    Vec3d horizontal = Vec3d({viewport_width, 0, 0});
    Vec3d vertical = Vec3d({0, viewport_height, 0});
    //视口左下角的坐标
    Point3d lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3d({0, 0, focal_length});

    //Render
    output << format("P3\n{} {}\n255\n", image_width, image_height);

    for (int j = image_height - 1; j >= 0; j--) {
        cerr << format("\rScanlines remaining: {} ", j) << flush;

        for (int i = 0; i < image_width; i++) {
            double u = static_cast<double>(i) / (image_width - 1);
            double v = static_cast<double>(j) / (image_height - 1);
            Ray r(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            Color pixel_color = rayColor(r, world);
            writeColor(output, pixel_color);
        }
        output << '\n';
    }

    cerr << "\nDone.\n";
    return 0;
}
