//
// Created by luosq on 2021/8/25.
//

#ifndef TOYRAYTRACER_CAMERA_H
#define TOYRAYTRACER_CAMERA_H

#include "toy_ray_tracer.h"

class Camera {
public:
    /**
     *
     * @param look_from 世界坐标下，相机的位置
     * @param look_at 相机看向的点
     * @param view_up 相机的上方，按理来说应该与在 与看向方向垂直的那个平面 上，但这里的 viewup 没有要求那么高，只要能投影到那个平面上就行
     * @param vertical_fov vertical field-of-view in degrees
     * @param aspect_ratio 宽高比
     * @param aperture 光圈直径
     * @param focus_dist    lens 到 focus plane 的距离
     */
    Camera(Point3d look_from,
           Point3d look_at,
           Vec3d view_up,
           double vertical_fov,
           double aspect_ratio,
           double aperture,
           double focus_dist) {

        double theta = degreesToRadians(vertical_fov);
        double h = focus_dist * tan(theta / 2);      // 我是在这里乘focus_dist，原文是在下面的 horizontal_ 那行乘的，虽然不影响结果，但感觉是原文有问题
        double viewport_height = 2.0 * h;   // 世界坐标下的 height 长度
        double viewport_width = aspect_ratio * viewport_height;     // 世界坐标下的 width 长度

        w_ = vecNormalized(look_from - look_at);     // w_ 与看向的方向反向
        u_ = vecNormalized(crossProduct(view_up, w_));
        v_ = crossProduct(w_, u_);

        origin_ = look_from;
        horizontal_ = viewport_width * u_;   // viewport 水平方向的完整向量
        vertical_ = viewport_height * v_;    // viewport 垂直方向的完整向量
        // 视口左下角的坐标
        lower_left_corner_ = origin_ - horizontal_ / 2.0 - vertical_ / 2.0 - focus_dist * w_;

        lens_radius_ = aperture / 2;
    }

    // 这里传入的 s, t 都是 0-1 之间的小数
    Ray getRay(double s, double t) const {
        Vec2d rd2d = lens_radius_ * randomInUnitCircle();
        Vec3d offset = rd2d.x() * u_ + rd2d.x() * v_;

        return Ray(origin_ + offset, lower_left_corner_ + s * horizontal_ + t * vertical_ - (origin_ + offset));
    }

private:
    Point3d origin_;
    Point3d lower_left_corner_;
    Vec3d horizontal_;
    Vec3d vertical_;
    Vec3d u_, v_, w_;      // u_, v_, w_ 是相机的正交基
    double lens_radius_;
};


#endif //TOYRAYTRACER_CAMERA_H
