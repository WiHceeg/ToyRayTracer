//
// Created by luosq on 2021/8/23.
//

#ifndef TOYRAYTRACER_HITTABLE_H
#define TOYRAYTRACER_HITTABLE_H

#include "ray.h"

// 光线与物体的碰撞记录。包括碰撞位置，法向量，时间，是否表面外
struct HitRecord {
public:
    Point3d p_;
    Vec3d normal_;
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
