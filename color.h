//
// Created by luosq on 2021/8/21.
//

#ifndef TOYRAYTRACER_COLOR_H
#define TOYRAYTRACER_COLOR_H

#include "geometry.h"
#include <iostream>
#include <format>

using namespace std;

void writeColor(ostream &out, Color pixel_color) {
    //写下每个颜色分量转换后的值[0,255]
    out << format("{} {} {}\n", static_cast<int>(255.999 * pixel_color.x()), static_cast<int>(255.999 * pixel_color.y()), static_cast<int>(255.999 * pixel_color.z()));
}

#endif //TOYRAYTRACER_COLOR_H
