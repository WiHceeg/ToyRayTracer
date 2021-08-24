//
// Created by luosq on 2021/8/24.
//

#ifndef TOYRAYTRACER_TOY_RAY_TRACER_H
#define TOYRAYTRACER_TOY_RAY_TRACER_H

#include <cmath>
#include <limits>
#include <memory>
#include <numbers>

using namespace std;
// Constants

constexpr double infinity = numeric_limits<double>::infinity();
constexpr double pi = numbers::pi;  // C++20 里才有

// Utility Functions
double degreesToRadians(double degrees) {
    return degrees * pi / 180.0;
}

// Common Headers

#include "Ray.h"
#include "geometry.h"


#endif //TOYRAYTRACER_TOY_RAY_TRACER_H
