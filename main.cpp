//
// Created by luosq on 2021/8/21.
//

#include "toy_ray_tracer.h"
#include "camera.h"
#include "color.h"
#include "hittable_list.h"
#include "sphere.h"
#include "material.h"

#include <thread>
#include <iostream>
#include <fstream>
#include <sstream>
#include <format>
#include <chrono>

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
//        Point3d target = record.p_ + record.normal_ + randomUnitVector();   // 8.5 真正的朗伯反射。1.更改后阴影不那么明显；2.更改后两个球体都更加明亮了。这两个变化都是由于光线的散射更加均匀，朝法线散射的光线更少。
//        Point3d target = record.p_ + randomInHemisphere(record.normal_);    // 8.6 在法向量的半球均匀反射。许多第一批射线追踪论文都使用这种扩散方法（在采用朗伯散射之前）

        Ray scattered;
        Vec3d attenuation;  // 衰减
        if (record.material_ptr->scatter(ray, record, attenuation, scattered)) {
            // 此处递归。之前有个疑问，attenuation 为什么不是 0.5，后来想明白了，因为每次反射的颜色都不一样，所以把这些颜色都乘起来才对
            return componentWiseProduct(attenuation, rayColor(scattered, world, depth - 1));
        } else {
            // 这里特指 Metal 的 scatter 方向往球里面去了
            return Color({0, 0, 0});
        }

    } else {
        // 没击中，背景色，这里可以理解成天空的颜色
        Vec3d unit_direction = vecNormalized(ray.direction());
        double t = 0.5 * (unit_direction.y() + 1.0);
        //线性插值
        return (1.0 - t) * Color({1.0, 1.0, 1.0}) + t * Color({0.5, 0.7, 1.0});
    }
}

HittableList randomScene() {
    HittableList world;

    // 地面
    shared_ptr<Lambertian> ground_material = make_shared<Lambertian>(Color({0.5, 0.5, 0.5}));
    world.add(make_shared<Sphere>(Point3d({0, -1000, 0}), 1000, ground_material));

    // 随机小球
    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            double choose_mat = randomDouble();
            Point3d center({a + 0.9 * randomDouble(), 0.2, b + 0.9 * randomDouble()});

            if (vecModulus(center - Point3d({4, 0.2, 0})) > 0.9) {
                shared_ptr<Material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    Color albedo = componentWiseProduct(randomVec3d(), randomVec3d());    // 反射率
                    sphere_material = make_shared<Lambertian>(albedo);
                    world.add(make_shared<Sphere>(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    Color albedo = randomVec3d(0.5, 1);
                    double fuzz = randomDouble(0, 0.5);
                    sphere_material = make_shared<Metal>(albedo, fuzz);
                    world.add(make_shared<Sphere>(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = make_shared<Medium>(1.5);
                    world.add(make_shared<Sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    // 三个大球
    auto material1 = make_shared<Medium>(1.5);
    world.add(make_shared<Sphere>(Point3d({0, 1, 0}), 1.0, material1));

    auto material2 = make_shared<Lambertian>(Color({0.4, 0.2, 0.1}));
    world.add(make_shared<Sphere>(Point3d({-4, 1, 0}), 1.0, material2));

    auto material3 = make_shared<Metal>(Color({0.7, 0.6, 0.5}), 0.0);
    world.add(make_shared<Sphere>(Point3d({4, 1, 0}), 1.0, material3));

    return world;
}

int main() {
    auto start_time = chrono::steady_clock::now();

    // Image
    constexpr double aspect_ratio = 3.0 / 2.0;
    constexpr int image_width = 1200;
    constexpr int image_height = static_cast<int>(image_width / aspect_ratio);
    constexpr int samples_per_pixel = 500;
    constexpr int max_depth = 50;

    // World
    HittableList world = randomScene();


    // Camera
    Point3d lookfrom({13, 2, 2});
    Point3d lookat({0, 0, 0});
    Vec3d viewup({0, 1, 0});
    double dist_to_focus = 10.0;
    double aperture = 0.1;

    Camera cam(lookfrom, lookat, viewup, 20.0, aspect_ratio, aperture, dist_to_focus);

    // 多线程 Render
    int concurrent_count = static_cast<int>(thread::hardware_concurrency());
    clog << format("{} CPU logical cores, {} threads are supported.\n\n", concurrent_count, concurrent_count);

    ofstream ofs_ppm("image13_MultiThread.ppm");
    vector<ostringstream> oss_str(concurrent_count);

    ofs_ppm << format("P3\n{} {}\n255\n", image_width, image_height);

    /*  定义子线程要执行的函数
        t 这是第 t 个线程
        j_hi 此线程渲染的最高 height
        j_lo 此线程渲染的最低 height
    */
    auto thread_task = [&](int t, int j_hi, int j_lo){
        for (int j = j_hi; j >= j_lo; j--) {
            clog << format("Thread {} scans lines [{}, {}], now {}\n", t, j_hi, j_lo, j);
            for (int i = 0; i < image_width; i++) {
                Color pixel_color({0.0, 0.0, 0.0});
                // 引入一点随机性，每个像素求 100 次
                for (int s = 0; s < samples_per_pixel; s++) {
                    double u = (i + randomDouble()) / (image_width - 1);
                    double v = (j + randomDouble()) / (image_height - 1);
                    Ray r = cam.getRay(u, v);
                    pixel_color += rayColor(r, world, max_depth);
                }
                writeColor(oss_str[t], pixel_color, samples_per_pixel);
            }
            oss_str[t] << '\n';
        }
        clog << format("\nThread {} scans lines [{}, {}], finished.\n\n", t, j_hi, j_lo);
    };

    vector<thread> threads;

    // 任务均分
    double hi_d, lo_d = static_cast<double>(image_height);
    double interval = static_cast<double>(image_height) / static_cast<double>(concurrent_count);
    int hi, lo;
    for (int t = 0; t < concurrent_count; t++) {
        hi_d = lo_d - 1.0;
        lo_d = hi_d - interval + 1.0;
        hi = static_cast<int>(round(hi_d));
        lo = static_cast<int>(round(lo_d));
        threads.push_back(thread(thread_task, t, hi, lo));
    }

    // 线程 join
    for (thread& t : threads) {
        t.join();
    }

    // 将每个线程的 ostringstream 输出到文件里
    for (int t = 0; t < concurrent_count; t++) {
        ofs_ppm << oss_str[t].str();
    }

    ofs_ppm.close();

    auto end_time = chrono::steady_clock::now();
    auto ms = chrono::duration_cast<chrono::milliseconds>(end_time - start_time).count();
    clog << format("\nConcurrent task finish, cost {} s\n", ms / 1000);

    return 0;
}
