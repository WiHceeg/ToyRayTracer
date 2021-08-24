//
// Created by luosq on 2021/8/24.
//

#ifndef TOYRAYTRACER_HITTABLE_LIST_H
#define TOYRAYTRACER_HITTABLE_LIST_H

#include "hittable.h"

#include <memory>
#include <vector>

using namespace std;

class HittableList : public Hittable {
public:
    HittableList() {}

    HittableList(shared_ptr<Hittable> object) {
        add(object);
    }

    void clear() {
        objects_.clear();
    }

    void add(shared_ptr<Hittable> object) {
        objects_.push_back(object);
    }

    virtual bool hit(const Ray &ray, double t_min, double t_max, HitRecord &record) const override;

public:
    vector<shared_ptr<Hittable>> objects_;
};

bool HittableList::hit(const Ray &ray, double t_min, double t_max, HitRecord &record) const {
    HitRecord temp_rec;
    bool hit_anything = false;
    double closest_so_far = t_max;  // 找到 objects 里最早碰撞的 object

    for (const auto &object : objects_) {
        if (object->hit(ray, t_min, closest_so_far, temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t_;
            record = temp_rec;  //循环后， 这里的 record 是与最早碰撞到的 object
        }
    }

    return hit_anything;
}


#endif //TOYRAYTRACER_HITTABLE_LIST_H
