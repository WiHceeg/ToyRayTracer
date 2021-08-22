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

double hitSphere(const Point3d &center, double radius, const Ray &r) {
    Vec3d oc = r.origin() - center;
    double a = vecModulusSquare(r.direction());
    double half_b = dotProduct(oc, r.direction());
    double c = vecModulusSquare(oc) - radius * radius;
    double discriminant = half_b * half_b - a * c;    // 判别式
    if (discriminant < 0) {
        return -1;
    } else {
        double t1 = (-half_b - sqrt(discriminant)) / a;
        if (t1 > 0) {
            return t1;
        }
        double t2 = (-half_b + sqrt(discriminant)) / a;
        if (t2 > 0) {
            return t2;
        }
        return -1.0;
    }
}

Color rayColor(const Ray &r) {
    double t = hitSphere(Point3d({0, 0, -1}), 0.5, r);
    if (t > 0) {
        Vec3d N = vecNormalized(r.at(t) - Point3d({0, 0, -1}));
        return 0.5 * Color({N.x() + 1, N.y() + 1, N.z() + 1});  // 这里加 1 是为了防止负值
    }
    Vec3d unit_direction = vecNormalized(r.direction());
    t = 0.5 * (unit_direction.y() + 1.0);
    //线性插值
    return (1.0 - t) * Color({1.0, 1.0, 1.0}) + t * Color({0.5, 0.7, 1.0});
}


int main() {
    ofstream output("image6.2.ppm");
    const double aspect_ratio = 16.0 / 9.0;
    const int image_width = 400;
    const int image_height = static_cast<int>(image_width / aspect_ratio);
    //Camera
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
            Color pixel_color = rayColor(r);
            writeColor(output, pixel_color);
        }
        output << '\n';
    }

    cerr << "\nDone.\n";
    return 0;
}
