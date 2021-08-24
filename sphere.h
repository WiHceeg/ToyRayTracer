//
// Created by luosq on 2021/8/23.
//

#ifndef TOYRAYTRACER_SPHERE_H
#define TOYRAYTRACER_SPHERE_H

#include "hittable.h"
#include "geometry.h"

class Sphere : public Hittable {
public:
    Sphere() {}

    Sphere(Point3d center, double radius) : center_(center), radius_(radius) {};

    virtual bool hit(const Ray &ray, double t_min, double t_max, HitRecord &record) const override;

public:
    Point3d center_;
    double radius_;
};

// record 是传出参数
bool Sphere::hit(const Ray &ray, double t_min, double t_max, HitRecord &record) const {
    Vec3d oc = ray.origin() - center_;
    double a = vecModulusSquare(ray.direction());
    double b = 2 * dotProduct(oc, ray.direction());
    double c = vecModulusSquare(oc) - radius_ * radius_;

    double discriminant = b * b - 4 * a * c;
    if (discriminant < 0) {
        return false;
    } else {
        double sqrt_discriminant = sqrt(discriminant);
        double t = (-b - sqrt_discriminant) / (2 * a);
        if (t < t_min || t > t_max) {
            t = (-b + sqrt_discriminant) / (2 * a);
            if (t < t_min || t > t_max) {
                return false;
            }
        }
        record.t_ = t;
        record.p_ = ray.at(record.t_);
        Vec3d outward_normal = (record.p_ - center_) / radius_;     // 交点指向球心
        record.setFaceNormal(ray, outward_normal);

        return true;
    }
}

#endif //TOYRAYTRACER_SPHERE_H
