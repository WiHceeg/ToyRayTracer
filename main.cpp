//
// Created by luosq on 2021/8/21.
//

#include "toy_ray_tracer.h"
#include "camera.h"
#include "color.h"
#include "hittable_list.h"
#include "sphere.h"


#include <iostream>
#include <fstream>
#include <format>

using namespace std;

// 这个函数的 world 是动态绑定，Hittable 类型的引用引用了 HittableList，调用了 HittableList 重写的hit函数
Color rayColor(const Ray &ray, const Hittable &world, int depth) {
    HitRecord record;

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if (depth <= 0)
        return Color({0, 0, 0});

    /*
     * 8.4 Fixing Shadow Acne，t_min 改为 0.001 的原因，参考 https://www.reddit.com/r/GraphicsProgramming/comments/m9rwx7/shadow_acne_in_ray_tracing_in_one_weekend/
     * 官方教程没讲清楚，其实是浮点数精度的问题，t_min 如果是 0，由于浮点精度的限制，算出一个很小很小的 double，它 > 0，于是继续反射衰减了。
     * 但事实上这个解应该是 0，这个解应该舍弃才对，所以设置 t_min 为 0.001，强迫光线走一段路
     */
    if (world.hit(ray, 0.001, infinity, record)) {
//        return 0.5 * (record.normal_ + Color({1,1,1}));   // 之前直接根据撞击位置的法向量生成颜色
//        Point3d target = record.p_ + record.normal_ + randomInUnitSphere();   // 8.2 引入漫反射，随机点
        Point3d target = record.p_ + record.normal_ + randomUnitVector();   // 8.5 真正的朗伯反射。1.更改后阴影不那么明显；2.更改后两个球体都更加明亮了。这两个变化都是由于光线的散射更加均匀，朝法线散射的光线更少。
//        Point3d target = record.p_ + randomInHemisphere(record.normal_);    // 8.6 在法向量的半球均匀反射。许多第一批射线追踪论文都使用这种扩散方法（在采用朗伯散射之前）

        return 0.5 * rayColor(Ray(record.p_, target - record.p_), world, depth - 1);

    } else {
        // 没击中，背景色
        Vec3d unit_direction = vecNormalized(ray.direction());
        double t = 0.5 * (unit_direction.y() + 1.0);
        //线性插值
        return (1.0 - t) * Color({1.0, 1.0, 1.0}) + t * Color({0.5, 0.7, 1.0});
    }
}


int main() {
    ofstream output("image8.6.ppm");

    // Image
    constexpr double aspect_ratio = 16.0 / 9.0;
    constexpr int image_width = 400;
    constexpr int image_height = static_cast<int>(image_width / aspect_ratio);
    constexpr int samples_per_pixel = 100;
    constexpr int max_depth = 50;

    // World
    HittableList world;
    world.add(make_shared<Sphere>(Point3d({0, 0, -1}), 0.5));
    world.add(make_shared<Sphere>(Point3d({0, -100.5, -1}), 100));    // 下半部分的大球


    // Camera
    Camera cam;

    //Render
    output << format("P3\n{} {}\n255\n", image_width, image_height);

    for (int j = image_height - 1; j >= 0; j--) {
        cerr << format("\rScanlines remaining: {} ", j) << flush;
        for (int i = 0; i < image_width; i++) {
            Color pixel_color({0.0, 0.0, 0.0});
            // 引入一点随机性，每个像素求 100 次
            for (int s = 0; s < samples_per_pixel; s++) {
                double u = (i + randomDouble()) / (image_width - 1);
                double v = (j + randomDouble()) / (image_height - 1);
                Ray r = cam.getRay(u, v);
                pixel_color += rayColor(r, world, max_depth);
            }
            writeColor(output, pixel_color, samples_per_pixel);
        }
        output << '\n';
    }

    cerr << "\nDone.\n";
    return 0;
}
