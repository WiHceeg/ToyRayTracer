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
    Color albedo_;      // 反射率，其实就是颜色
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
    Metal(const Color &albedo, double fuzz) : albedo_(albedo), fuzz_(fuzz < 1 ? fuzz : 1.0) {}

    virtual bool scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const override;

public:
    Color albedo_;      // 反射率，其实就是颜色
    double fuzz_;       // 模糊度
};

bool Metal::scatter(const Ray &ray_in, const HitRecord &record, Vec3d &attenuation, Ray &scattered) const {
    Vec3d reflected = reflect(vecNormalized(ray_in.direction()), record.normal_);
    scattered = Ray(record.p_, reflected + fuzz_ * randomInUnitSphere());
    attenuation = albedo_;
    return (dotProduct(scattered.direction(), record.normal_) > 0);
}

// 原文用的 dielectric，是电介质的意思。medium 才是介质的意思
class Medium : public Material {
public:
    Medium(double refractive_index) : refractive_index_(refractive_index) {}

    virtual bool scatter(const Ray &ray_in,
                         const HitRecord &record,
                         Vec3d &attenuation,     // 衰减
                         Ray &scattered) const override;

public:
    double refractive_index_;   // 折射率
};

bool Medium::scatter(const Ray &ray_in, const HitRecord &record, Vec3d &attenuation, Ray &scattered) const {
    attenuation = Color({1.0, 1.0, 1.0});
    double refraction_ratio = record.front_face_ ? (1.0 / refractive_index_) : refractive_index_;

//    Vec3d incident_direction = vecNormalized(ray_in.direction());
//
//    double cos_theta_i = min<double>(dotProduct(-incident_direction, record.normal_), 1.0);
//    double sin_theta_i = sqrt(1.0 - cos_theta_i * cos_theta_i);
//
//    bool can_refract = refraction_ratio * sin_theta_i < 1.0;
//    Vec3d out_direction;
//    if (can_refract) {
//        out_direction = refract(incident_direction, record.normal_, refraction_ratio);
//    } else {
//        out_direction = reflect(incident_direction, record.normal_);
//    }
//    scattered = Ray(record.p_, out_direction);

    Vec3d unit_direction = vecNormalized(ray_in.direction());
    Vec3d refracted = refract(unit_direction, record.normal_, refraction_ratio);

    scattered = Ray(record.p_, refracted);

    return true;
}

#endif //TOYRAYTRACER_MATERIAL_H
