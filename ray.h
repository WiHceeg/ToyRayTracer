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

// incident_direction，normal 都是单位向量
Vec3d reflect(const Vec3d &incident_direction, const Vec3d &normal) {
    return incident_direction + 2 * dotProduct(-incident_direction, normal) * normal;       // incident_direction 和 n 成钝角
}


// incident_direction，normal 都是单位向量。etai_over_etat 是 η_incidence / η_transmission，也就是折射率的比值
// 折射的 Snell's Law，η_i * sin(θ_i) = η_t * sin(θ_t)，也就是说折射率的比值和 sinθ 的比值互为倒数
// 下面公式的推导可以参考 https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
Vec3d refract(const Vec3d &incident_direction, const Vec3d &normal, double etai_over_etat) {
    double cos_theta_i = min<double>(dotProduct(-incident_direction, normal), 1.0);

    // (incident_direction + cos_theta_i * normal) 是 incident_direction 与 normal 的垂直分量，显然它的模长为 sin_theta
    Vec3d r_out_perpendicular = etai_over_etat * (incident_direction + cos_theta_i * normal);
    Vec3d r_out_parallel = -sqrt(fabs(1.0 - vecModulusSquare(r_out_perpendicular))) * normal;
    return r_out_perpendicular + r_out_parallel;
}


#endif //TOYRAYTRACER_RAY_H
