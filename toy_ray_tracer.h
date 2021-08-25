//
// Created by luosq on 2021/8/24.
//

#ifndef TOYRAYTRACER_TOY_RAY_TRACER_H
#define TOYRAYTRACER_TOY_RAY_TRACER_H

// Common Headers
#include "ray.h"
#include "geometry.h"


#include <cmath>
#include <limits>
#include <memory>
#include <numbers>
#include <random>
#include <algorithm>

using namespace std;
// Constants

constexpr double infinity = numeric_limits<double>::infinity();
constexpr double pi = numbers::pi;  // C++20 里才有

// Utility Functions
double degreesToRadians(double degrees) {
    return degrees * pi / 180.0;
}

double randomDouble() {
    static uniform_real_distribution<double> distribution(0.0, 1.0);
    static mt19937 generator;
    return distribution(generator);
}

double randomDouble(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max - min) * randomDouble();
}

Vec3d randomVec3d() {
    return Vec3d({randomDouble(), randomDouble(), randomDouble()});
}

Vec3d randomVec3d(double min, double max) {
    return Vec3d({randomDouble(min, max), randomDouble(min, max), randomDouble(min, max)});
}

// 单位球里的坐标
Vec3d randomInUnitSphere() {
    while (true) {
        Vec3d p = randomVec3d(-1.0, 1.0);
        if (vecModulusSquare(p) < 1) {
            return p;
        }
    }
}


#endif //TOYRAYTRACER_TOY_RAY_TRACER_H
