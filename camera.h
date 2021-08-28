//
// Created by luosq on 2021/8/25.
//

#ifndef TOYRAYTRACER_CAMERA_H
#define TOYRAYTRACER_CAMERA_H

#include "toy_ray_tracer.h"

class Camera {
public:
    Camera(double vfov,     // vertical field-of-view in degrees
           double aspect_ratio) {
        double theta = degreesToRadians(vfov);
        double h = tan(theta / 2);
        double viewport_height = 2.0 * h;
        double viewport_width = aspect_ratio * viewport_height;

        double focal_length = 1.0;

        origin_ = Point3d({0, 0, 0});
        horizontal_ = Vec3d({viewport_width, 0.0, 0.0});
        vertical_ = Vec3d({0.0, viewport_height, 0.0});
        // 视口左下角的坐标
        lower_left_corner_ = origin_ - horizontal_ / 2.0 - vertical_ / 2.0 - Vec3d({0, 0, focal_length});
    }

    Ray getRay(double u, double v) const {
        return Ray(origin_, lower_left_corner_ + u * horizontal_ + v * vertical_ - origin_);
    }

private:
    Point3d origin_;
    Point3d lower_left_corner_;
    Vec3d horizontal_;
    Vec3d vertical_;
};


#endif //TOYRAYTRACER_CAMERA_H
