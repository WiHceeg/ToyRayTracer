//
// Created by luosq on 2021/8/26.
//

#ifndef TOYRAYTRACER_MATERIAL_H
#define TOYRAYTRACER_MATERIAL_H

#include "toy_ray_tracer.h"

struct HitRecord;

class Material {
public:
    virtual bool scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const = 0;
};

class Lambertian : public Material {
public:
    Lambertian(const Vec3d &albedo) : albedo_(albedo) {}

    virtual bool scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const override;

public:
    Vec3d albedo_;      // 反射率
};

bool Lambertian::scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const {
    Vec3d scatter_direction = record.normal_ + randomUnitVector();

    // Catch degenerate scatter direction
    if (vecNearZero(scatter_direction)) {
        scatter_direction = rec.normal;
    }

    scattered = Ray(record.p, scatter_direction);
    attenuation = albedo_;
    return true;
}

#endif //TOYRAYTRACER_MATERIAL_H
