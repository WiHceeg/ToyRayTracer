//
// Created by luosq on 2021/8/23.
//

#ifndef TOYRAYTRACER_HITTABLE_H
#define TOYRAYTRACER_HITTABLE_H

#include "toy_ray_tracer.h"

class Material;     // 类互相引用，用前置声明


// 光线与物体的碰撞记录。包括碰撞位置，法向量，时间，是否表面外
struct HitRecord {
public:
    Point3d p_;
    Vec3d normal_;                          // Sphere hit 时，会计算 normal_，用 (record.p_ - center_) / radius_ 算的，normal_被单位化了
    shared_ptr<Material> material_ptr;      // 当光线撞击表面（例如特定的球体）时，HitRecord中的材质指针将设置为在main()中设置该球体时所给定的材质指针
    double t_;
    bool front_face_;

    void setFaceNormal(const Ray &ray, const Vec3d &outward_normal) {
        // ray 是 cam 指向物体，outword_normal 是物体内朝外，
        front_face_ = dotProduct(ray.direction(), outward_normal) < 0;
        // 目前的做法是让法线朝外
        normal_ = front_face_ ? outward_normal : -outward_normal;
    }

};

class Hittable {
public:
    // 限定 t 的范围，当且仅当 t_min < t < t_max 时才认为命中(有交点)
    // = 0 代表是纯虚函数
    virtual bool hit(const Ray &ray, double t_min, double t_max, HitRecord &record) const = 0;
};


#endif //TOYRAYTRACER_HITTABLE_H
