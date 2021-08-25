//
// Created by luosq on 2021/8/21.
//

#ifndef TOYRAYTRACER_COLOR_H
#define TOYRAYTRACER_COLOR_H

#include "geometry.h"
#include <iostream>
#include <format>
#include <algorithm>

using namespace std;

void writeColor(ostream &out, Color pixel_color, int samples_per_pixel) {

    double r = pixel_color.x();
    double g = pixel_color.y();
    double b = pixel_color.z();
    //根据样本数对颜色取平均值
    double scale = 1.0 / samples_per_pixel;
    r = sqrt(r * scale);
    g = sqrt(g * scale);
    b = sqrt(b * scale);

    //写下每个颜色分量转换后的值[0,255]
    out << format("{} {} {} ",
                  static_cast<int>(256 * clamp(r, 0.0, 0.999)),
                  static_cast<int>(256 * clamp(g, 0.0, 0.999)),
                  static_cast<int>(256 * clamp(b, 0.0, 0.999)));
}

#endif //TOYRAYTRACER_COLOR_H
