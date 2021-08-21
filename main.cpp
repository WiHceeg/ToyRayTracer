//
// Created by luosq on 2021/8/21.
//

#include <iostream>
#include <fstream>
#include <format>

#include "geometry.h"
#include "color.h"
#include "ray.h"

using namespace std;

// 实现渐变色
Color rayColor(const Ray& r) {
    Vec3d unit_direction = vecNormalized(r.direction());
    double t = 0.5 * (unit_direction.y() + 1.0);
    //线性插值
    return (1.0 - t) * Color({1.0, 1.0, 1.0}) + t * Color({0.5, 0.7, 1.0});
}

int main() {
    ofstream output("image2.2.ppm");
    const int image_width = 256;
    const int image_height = 256;
//    output << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    output << format("P3\n{} {}\n255\n", image_width, image_height);

    for (int j = image_height - 1; j >= 0; --j) {
        cerr << "\rScanlines remaining: " << j << ' ' << flush;
        for (int i = 0; i < image_width; ++i) {
            double r = static_cast<double>(i) / (image_width - 1);
            double g = static_cast<double>(j) / (image_height - 1);
            double b = 0.25;
            int ir = static_cast<int>(255.999 * r);
            int ig = static_cast<int>(255.999 * g);
            int ib = static_cast<int>(255.999 * b);
//            output << ir << ' ' << ig << ' ' << ib << '\n';
//            output << format("{} {} {}\n", ir, ig, ib);
            Color pixel_color({r, g, b});
            writeColor(output, pixel_color);
        }
        output << '\n';
    }
//    const double aspect_ratio = 16.0 / 9.0;
//    const int image_width = 400;
//    const int image_height = static_cast<int>(image_width / aspect_ratio);
//    //Camera
//    double viewport_height = 2.0;
//    double viewport_width = aspect_ratio * viewport_height;
//    double focal_length = 1.0;
//
//    Point3d origin = Point3d({0, 0, 0});
//    Vec3d horizontal = Vec3d({viewport_width, 0, 0});
//    Vec3d vertical = Vec3d({0, viewport_height, 0});
//    //视口左下角的坐标
//    Point3d lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3d({0, 0, focal_length});
//
//    //Render
//    output << format("P3\n{} {}\n255\n", image_width, image_height);
//
//    for (int j = image_height - 1; j >= 0; j--) {
//        for (int i = 0; i < image_width; i++) {
//            double u = static_cast<double>(i) / (image_width - 1);
//            double v = static_cast<double>(j) / (image_height - 1);
//            Ray r(origin, lower_left_corner + u * horizontal + v * vertical - origin);
//            Color pixel_color = rayColor(r);
//            writeColor(output, pixel_color);
//        }
//        output << '\n';
//    }

    return 0;
}
