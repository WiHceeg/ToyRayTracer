//
// Created by luosq on 2021/8/22.
//

#ifndef TOYRAYTRACER_RAY_H
#define TOYRAYTRACER_RAY_H

#include "geometry.h"

class Ray {
public:
    Ray() {}

    Ray(const Point3d &origin, const Vec3d &direction) : origin_(origin), direction_(direction) {}

    Point3d origin() const {
        return origin_;
    }

    Vec3d direction() const {
        return direction_;
    }

    Point3d at(double t) const {
        return origin_ + t * direction_;
    }

public:
    Point3d origin_;
    Vec3d direction_;
};


#endif //TOYRAYTRACER_RAY_H
