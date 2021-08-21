//
// Created by luosq on 2021/8/21.
//

#include <iostream>
#include <fstream>
#include <format>

#include "geometry.h"
#include "color.h"
#include "ray.h"

using namespace std;


int main() {
    ofstream output("image.ppm");
    const int image_width = 256;
    const int image_height = 256;
//    output << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    output << format("P3\n{} {}\n255\n", image_width, image_height);

    for (int j = image_height - 1; j >= 0; --j) {
        for (int i = 0; i < image_width; ++i) {
            double r = static_cast<double>(i) / (image_width - 1);
            double g = static_cast<double>(j) / (image_height - 1);
            double b = 0.25;
            int ir = static_cast<int>(255.999 * r);
            int ig = static_cast<int>(255.999 * g);
            int ib = static_cast<int>(255.999 * b);
//            output << ir << ' ' << ig << ' ' << ib << '\n';
//            output << format("{} {} {}\n", ir, ig, ib);
            Color pixel_color({r, g, b});
            writeColor(output, pixel_color);

        }
    }

    return 0;
}
