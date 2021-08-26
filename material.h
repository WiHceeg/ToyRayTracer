//
// Created by luosq on 2021/8/26.
//

#ifndef TOYRAYTRACER_MATERIAL_H
#define TOYRAYTRACER_MATERIAL_H

#include "toy_ray_tracer.h"

struct HitRecord;   // 类互相引用，前置声明

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
    Vec3d scatter_direction = record.normal_ + randomUnitVector();      // 8.5的朗伯反射

    // Catch degenerate scatter direction
    if (vecNearZero(scatter_direction)) {
        scatter_direction = record.normal_;
    }

    scattered = Ray(record.p_, scatter_direction);
    attenuation = albedo_;
    return true;
}

class Metal : public Material {
public:
    Metal(const Vec3d &albedo) : albedo_(albedo) {}

    virtual bool scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const override;

public:
    Color albedo_;
};

bool Metal::scatter(const Ray &ray_in, const HitRecord &record, Vec3d &attenuation, Ray &scattered) const {
    Vec3d reflected = reflect(vecNormalized(ray_in.direction()), record.normal_);
    scattered = Ray(record.p_, reflected);
    attenuation = albedo_;
    return (dotProduct(scattered.direction(), record.normal_) > 0);
}

#endif //TOYRAYTRACER_MATERIAL_H
