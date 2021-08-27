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

private:
    // 静态成员函数主要为了调用方便，不需要生成对象就能调用。
    // 函数的行为对所有派生类都一致时，可以声明为静态。
    // 静态成员函数的作用基本上相当于：一个带有命名空间的全局函数。
    static double reflectance(double cosine, double ref_idx);
};

bool Medium::scatter(const Ray &ray_in, const HitRecord &record, Vec3d &attenuation, Ray &scattered) const {
    attenuation = Color({1.0, 1.0, 1.0});
    double refraction_ratio = record.front_face_ ? (1.0 / refractive_index_) : refractive_index_;

    Vec3d incident_direction = vecNormalized(ray_in.direction());

    double cos_theta_i = min<double>(dotProduct(-incident_direction, record.normal_), 1.0);
    double sin_theta_i = sqrt(1.0 - cos_theta_i * cos_theta_i);

    // 考虑能不能折射，也就是全内反射的情况
    // 值得注意的是，之所以 10.3 和 10.2 用 10.3 的世界没区别，是因为从外面进球里面的光肯定不会发生全内反射，光路可逆
    bool can_refract = refraction_ratio * sin_theta_i < 1.0;
    Vec3d out_direction;
    // 能折射的时候按 Schlick 近似反射率反射；不能折射的时候全反射。（这样逻辑会比教程清晰一些）
    if (can_refract) {
        if (reflectance(cos_theta_i, refraction_ratio) > randomDouble()) {
            out_direction = reflect(incident_direction, record.normal_);
        } else {
            out_direction = refract(incident_direction, record.normal_, refraction_ratio);
        }
    } else {
        out_direction = reflect(incident_direction, record.normal_);
    }
    scattered = Ray(record.p_, out_direction);

    return true;
}

// 真实的玻璃具有随角度变化的反射率—以陡峭的角度看窗户，它将变成一面镜子。Schlick 近似
double Medium::reflectance(double cosine, double ref_idx) {
    // Use Schlick's approximation for reflectance.
    double r0 = (1 - ref_idx) / (1 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1 - r0) * pow((1 - cosine), 5);
}


#endif //TOYRAYTRACER_MATERIAL_H
