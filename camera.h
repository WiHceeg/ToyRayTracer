//
// Created by luosq on 2021/8/25.
//

#ifndef TOYRAYTRACER_CAMERA_H
#define TOYRAYTRACER_CAMERA_H

#include "toy_ray_tracer.h"

class Camera {
public:
    // 这几个参数是世界坐标下的
    Camera(Point3d lookfrom,    // 相机的位置
           Point3d lookat,      // 相机看向的点
           Vec3d viewup,        // 相机的上方，按理来说应该与在 与看向方向垂直的那个平面 上，但这里的 viewup 没有要求那么高，只要能投影到那个平面上就行
           double vfov,         // vertical field-of-view in degrees
           double aspect_ratio) {
        double theta = degreesToRadians(vfov);
        double h = tan(theta / 2);      // 这里应该有个隐含条件是 focal_length == 1.0，h = focal_length * tan(theta / 2)
        double viewport_height = 2.0 * h;   // 世界坐标下的 height 长度
        double viewport_width = aspect_ratio * viewport_height;     // 世界坐标下的 width 长度

        // w, u, v 是相机的正交基
        Vec3d w = vecNormalized(lookfrom - lookat);     // w 与看向的方向反向
        Vec3d u = vecNormalized(crossProduct(viewup, w));
        Vec3d v = crossProduct(w, u);

        origin_ = lookfrom;
        horizontal_ = viewport_width * u;   // viewport 水平方向的单位向量
        vertical_ = viewport_height * v;    // viewport 垂直方向的单位向量
        // 视口左下角的坐标
        lower_left_corner_ = origin_ - horizontal_ / 2.0 - vertical_ / 2.0 - w;     // 因为 focal_length == 1.0，所以 w 没有修饰
    }

    // 这里传入的 u, v 都是 0-1 之间的小数
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
