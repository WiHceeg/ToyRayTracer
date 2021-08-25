//
// Created by luosq on 2021/8/21.
//

#ifndef TOYRAYTRACER_GEOMETRY_H
#define TOYRAYTRACER_GEOMETRY_H

#include <iostream>
//#include <cassert>
#include <initializer_list>

#include <cmath>

using namespace std;

//向量类
template<int n, typename T>
class Vec {
public:
    T data_[n] = {0};

    Vec() = default;

    Vec(const Vec<n, T> &src) {
        for (int i = 0; i < n; i++) {
            data_[i] = src.data_[i];
        }
    }

    Vec(const std::initializer_list<T> &src) {
        auto it = src.begin();
        for (int i = 0; i < n; i++) {
            data_[i] = *it;
            it++;
        }
    }


    Vec(const T *ptr) {
        for (int i = 0; i < n; i++) {
            data_[i] = ptr[i];
        }
    }

    T &operator[](const int i) {
        //assert(0 <= i && i < n);
        return data_[i];
    }

    const T &operator[](const int i) const {
        //assert(0 <= i && i < n);
        return data_[i];
    }

    T &x() {
        //assert(n >= 1);
        return data_[0];
    }

    const T &x() const {
        //assert(n >= 1);
        return data_[0];
    }

    T &y() {
        //assert(n >= 2);
        return data_[1];
    }

    const T &y() const {
        //assert(n >= 2);
        return data_[1];
    }

    T &z() {
        //assert(n >= 3);
        return data_[2];
    }

    const T &z() const {
        //assert(n >= 3);
        return data_[2];
    }

    T &w() {
        //assert(n >= 4);
        return data_[3];
    }

    const T &w() const {
        //assert(n >= 4);
        return data_[3];
    }

};

//-------------------------------------------------------------------------------------------------
//重载向量输入输出

// 输入


// 输出
template<int n, typename T>
ostream &operator<<(ostream &os, const Vec<n, T> &a) {
    os << '[';
    for (int i = 0; i < n - 1; i++) {
        os << a[i] << ", ";
    }
    os << a[n - 1] << ']';
    return os;
}




//-------------------------------------------------------------------------------------------------
//重载向量普通运算

// +a
template<int n, typename T>
Vec<n, T> operator+(const Vec<n, T> &a) {
    return a;
}

// -a
template<int n, typename T>
Vec<n, T> operator-(const Vec<n, T> &a) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = -a[i];
    }
    return ret;
}

// a == b
template<int n, typename T>
bool operator==(const Vec<n, T> &a, const Vec<n, T> &b) {
    for (int i = 0; i < n; i++) {
        if (a[i] != b[i]) {
            return false;
        }
    }
    return true;
}

// a != b
template<int n, typename T>
bool operator!=(const Vec<n, T> &a, const Vec<n, T> &b) {
    return !(a == b);
}

// a + b
template<int n, typename T>
Vec<n, T> operator+(const Vec<n, T> &a, const Vec<n, T> &b) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] + b[i];
    }
    return ret;
}

// a - b
template<int n, typename T>
Vec<n, T> operator-(const Vec<n, T> &a, const Vec<n, T> &b) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] - b[i];
    }
    return ret;
}

// a * b，内积
template<int n, typename T>
T operator*(const Vec<n, T> &a, const Vec<n, T> &b) {
    return dotProduct(a, b);
}

// a / b，各个元素相除
template<int n, typename T>
Vec<n, T> operator/(const Vec<n, T> &a, const Vec<n, T> &b) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] / b[i];
    }
    return ret;
}


// x * a, 数乘向量
template<int n, typename T>
Vec<n, T> operator*(T x, const Vec<n, T> &a) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] * x;
    }
    return ret;
}

//  a * x，向量乘数
template<int n, typename T>
Vec<n, T> operator*(const Vec<n, T> &a, T x) {
    return x * a;
}


// a / x
template<int n, typename T>
Vec<n, T> operator/(const Vec<n, T> &a, T x) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] / x;
    }
    return ret;
}

// x / a
template<int n, typename T>
Vec<n, T> operator/(T x, const Vec<n, T> &a) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = x / a[i];
    }
    return ret;
}

// a += b
template<int n, typename T>
Vec<n, T> &operator+=(Vec<n, T> &a, const Vec<n, T> &b) {
    for (int i = 0; i < n; i++) {
        a[i] += b[i];
    }
    return a;
}

// a -= b
template<int n, typename T>
Vec<n, T> &operator-=(Vec<n, T> &a, const Vec<n, T> &b) {
    for (int i = 0; i < n; i++) {
        a[i] -= b[i];
    }
    return a;
}


// a /= b
template<int n, typename T>
Vec<n, T> &operator/=(Vec<n, T> &a, const Vec<n, T> &b) {
    for (int i = 0; i < n; i++) {
        a[i] /= b[i];
    }
    return a;
}

// a *= x
template<int n, typename T>
Vec<n, T> &operator*=(Vec<n, T> &a, T x) {
    for (int i = 0; i < n; i++) {
        a[i] *= x;
    }
    return a;
}

// a /= x
template<int n, typename T>
Vec<n, T> &operator/=(Vec<n, T> &a, T x) {
    for (int i = 0; i < n; i++) {
        a[i] /= x;
    }
    return a;
}

//-------------------------------------------------------------------------------------------------
//向量几何运算


// 向量的元素乘法
template<int n, typename T>
Vec<n, T> componentWiseProduct(const Vec<n, T> &a, const Vec<n, T> &b) {
    Vec<n, T> ret;
    for (int i = 0; i < n; i++) {
        ret[i] = a[i] * b[i];
    }
    return ret;
}


// 向量的内积, 函数
template<int n, typename T>
T dotProduct(const Vec<n, T> &a, const Vec<n, T> &b) {
    T ret = 0;
    for (int i = 0; i < n; i++) {
        ret += a[i] * b[i];
    }
    return ret;
}


// 向量的叉积，函数，二三四维
// 二维向量叉乘，得到标量
template<typename T>
T crossProduct(const Vec<2, T> &a, const Vec<2, T> &b) {
    return a.x() * b.y() - a.y() * b.x();
}

// 三维向量叉乘，得到新向量
template<typename T>
Vec<3, T> crossProduct(const Vec<3, T> &a, const Vec<3, T> &b) {
    return Vec<3, T>{a.y() * b.z() - a.z() * b.y(),
                     a.z() * b.x() - a.x() * b.z(),
                     a.x() * b.y() - a.y() * b.x()};
}

// 四维向量叉乘：前三维叉乘，后一维保留
template<typename T>
Vec<4, T> crossProduct(const Vec<4, T> &a, const Vec<4, T> &b) {
    return Vec<4, T>{a.y() * b.z() - a.z() * b.y(),
                     a.z() * b.x() - a.x() * b.z(),
                     a.x() * b.y() - a.y() * b.x(),
                     a.w()};
}

// |a|^2, 向量的模长的平方
template<int n, typename T>
T vecModulusSquare(const Vec<n, T> &a) {
    return dotProduct(a, a);
}

// |a|, 向量的模长
template<int n, typename T>
T vecModulus(const Vec<n, T> &a) {
    return sqrt(dotProduct(a, a));
}

// |a| ,特化 float，用sqrtf
template<int n>
float vecModulus(const Vec<n, float>& a) {
    return sqrtf(dotProduct(a, a));
}

// 返回 a 单位化的结果
template<int n, typename T>
Vec<n, T> vecNormalized(const Vec<n, T> &a) {
    return a / vecModulus(a);
}

// 返回 a 齐次化的结果
template<int n, typename T>
Vec<n, T> vecHomogenized(const Vec<n, T> &a) {
    T last_d = a.data_[n - 1];
    return a / last_d;
}



// 返回 a 升维的结果
template<int n, typename T>
Vec<n + 1, T> vecDimensionIncreased(const Vec<n, T> &a) {
    Vec<n + 1, T> ret;
    for (int i = 0; i < n; i++) {
        ret.data_[i] = a.data_[i];
    }
    ret.data_[n] = 1;
    return ret;
}

// 返回 a 直接降维的结果，不用管最后一维
template<int n, typename T>
Vec<n - 1, T> vecDimensionReducedDir(const Vec<n, T> &a) {
    Vec<n - 1, T> ret;
    for (int i = 0; i < n - 1; i++) {
        ret.data_[i] = a.data_[i];
    }
    return ret;
}


// 返回 a 先变成齐次坐标（最后一维变成1）再降维的结果
template<int n, typename T>
Vec<n - 1, T> vecDimensionReducedHom(const Vec<n, T> &a) {
    Vec<n - 1, T> ret;
    T last_d = a.data_[n - 1];
    for (int i = 0; i < n - 1; i++) {
        ret.data_[i] = a.data_[i] / last_d;
    }
    return ret;
}

//-------------------------------------------------------------------------------------------------
// 类型转换
template<int n, typename T>
Vec<n, float> vecToFloat(const Vec<n, T> &a) {
    Vec<n, float> ret;
    for (int i = 0; i < n; i++) {
        ret.data_[i] = static_cast<float>(a.data_[i]);
    }
    return ret;
}

template<int n, typename T>
Vec<n, double> vecToDouble(const Vec<n, T> &a) {
    Vec<n, double> ret;
    for (int i = 0; i < n; i++) {
        ret.data_[i] = static_cast<double>(a.data_[i]);
    }
    return ret;
}

template<int n, typename T>
Vec<n, int> vecToInt(const Vec<n, T> &a) {
    Vec<n, int> ret;
    for (int i = 0; i < n; i++) {
        ret.data_[i] = lround(a.data_[i]);
    }
    return ret;
}


//-------------------------------------------------------------------------------------------------
//矩阵类
template<int m, int n, typename T>
class Mat {
        public:
            T data_[m][n] = {0};

            Mat() = default;

            Mat(const Mat<m, n, T> &src) {
                for (int i = 0; i < m; i++) {
                    for (int j = 0; j < n; j++) {
                        data_[i][j] = src.data_[i][j];
                    }
                }
            }

            Mat(const initializer_list<Vec<n, T>> &src) {
                auto it = src.begin();
                for (int i = 0; i < m; i++) {
                    setRow(i, *it);
                    it++;
                }
            }

            T *operator[](const int i) {
                //assert(0 <= i && i < m);
                return data_[i];
            }

            const T *operator[](const int i) const {
                //assert(0 <= i && i < m);
                return data_[i];
            }


            // 取一行
            Vec<n, T> getRow(int row) const {
                //assert(row < m);
                Vec<n, T> ret;
                for (int j = 0; j < n; j++) {
                    ret[j] = data_[row][j];
                }
                return ret;
            }

            // 取一列
            Vec<m, T> getCol(int col) const {
                //assert(col < n);
                Vec<m, T> ret;
                for (int i = 0; i < m; i++) {
                    ret[i] = data_[i][col];
                }
                return ret;
            }

            // 设置一行
            void setRow(int row, const Vec<n, T> &a) {
                //assert(row < m);
                for (int i = 0; i < n; i++) {
                    data_[row][i] = a[i];
                }
            }

            // 设置一列
            void setCol(int col, const Vec<m, T> &a) {
                //assert(col < n);
                for (int i = 0; i < m; i++) {
                    data_[i][col] = a[i];
                }
            }

        };


//-------------------------------------------------------------------------------------------------
//重载矩阵输入输出

// 输出
template<int m, int n, typename T>
ostream &operator<<(ostream &os, const Mat<m, n, T> &a) {
    for (int i = 0; i < m; i++) {
        if (i == 0) {
            os << '[';
        } else {
            os << ' ';
        }

        Vec<n, T> ai = a[i];

        os << ai;

        if (i == m - 1) {
            os << ']';
        } else {
            os << endl;
        }
    }
    return os;
}


//-------------------------------------------------------------------------------------------------
// 矩阵普通运算

// a == b
template<int m, int n, typename T>
bool operator==(const Mat<m, n, T> &a, const Mat<m, n, T> &b) {
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            if (a.m[i][j] != b.m[i][j]) {
                return false;
            }
        }
    }
    return true;
}

// a != b
template<int m, int n, typename T>
bool operator!=(const Mat<m, n, T> &a, const Mat<m, n, T> &b) {
    return !(a == b);
}

// +a
template<int m, int n, typename T>
Mat<m, n, T> operator+(const Mat<m, n, T> &src) {
    return src;
}

// -a
template<int m, int n, typename T>
Mat<m, n, T> operator-(const Mat<m, n, T> &src) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = -src.data_[i][j];
        }
    }
    return ret;
}

// a + b
template<int m, int n, typename T>
Mat<m, n, T> operator+(const Mat<m, n, T> &a, const Mat<m, n, T> &b) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = a.data_[i][j] + b.data_[i][j];
        }
    }
    return ret;
}

// a - b
template<int m, int n, typename T>
Mat<m, n, T> operator-(const Mat<m, n, T> &a, const Mat<m, n, T> &b) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = a.data_[i][j] - b.data_[i][j];
        }
    }
    return ret;
}


// x * a，数与矩阵相乘
template<int m, int n, typename T>
Mat<m, n, T> operator*(T x, const Mat<m, n, T> &a) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = x * a.data_[i][j];
        }
    }
    return ret;
}

// a * x
template<int m, int n, typename T>
Mat<m, n, T> operator*(const Mat<m, n, T> &a, T x) {
    return x * a;
}

// a / x
template<int m, int n, typename T>
Mat<m, n, T> operator/(const Mat<m, n, T> &a, T x) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = a.data_[i][j] / x;
        }
    }
    return ret;
}


// x / a，这个一般用不上
template<int m, int n, typename T>
Mat<m, n, T> operator/(T x, const Mat<m, n, T> &a) {
    Mat<m, n, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[i][j] = x / a.data_[i][j];
        }
    }
    return ret;
}


//-------------------------------------------------------------------------------------------------
// 矩阵与向量运算


//行向量与矩阵相乘(其实不区分行向量和列向量)，a(1*m) * A(m*n) = ret(1*n)
template<int m, int n, typename T>
Vec<n, T> operator*(const Vec<m, T> &a, const Mat<m, n, T> &A) {
    Vec<n, T> ret;
    for (int j = 0; j < n; j++)
        ret[j] = dotProduct(a, A.getCol(j));
    return ret;
}

//矩阵与列向量相乘，A(m*n) * a(n*1) = ret(m*1)
template<int m, int n, typename T>
Vec<m, T> operator*(const Mat<m, n, T> &A, const Vec<n, T> &a) {
    Vec<m, T> ret;
    for (int i = 0; i < m; i++) {
        ret[i] = dotProduct(A.getRow(i), a);
    }
    return ret;
}

//-------------------------------------------------------------------------------------------------
// 矩阵与矩阵运算


// a(m*n) * b(n*p) = ret(m*p)，这里的矩阵乘法缓存命中率比普通写法高
template<int m, int n, int p, typename T>
Mat<m, p, T> operator*(const Mat<m, n, T> &a, const Mat<n, p, T> &b) {
    Mat<m, p, T> ret;
    for (int k = 0; k < n; k++) {
        for (int i = 0; i < m; i++) {
            T r = a.data_[i][k];
            for (int j = 0; j < p; j++) {
                ret.data_[i][j] += r * b.data_[k][j];
            }
        }
    }
    /*普通写法
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < p; j++) {
                T sum = 0;
                for (int k = 0; k < n; k++) {
                    sum += a.data_[i][k] * b.data_[k][j];
                }
                ret.data_[i][j] = sum;
            }
        }
    */
    return ret;
}

//-------------------------------------------------------------------------------------------------
// 生成矩阵

// 单位阵
template<int n, typename T>
Mat<n, n, T> identityMat() {
    Mat<n, n, T> ret;
    for (int i = 0; i < n; i++) {
        ret.data_[i][i] = 1;
    }
    return ret;
}

// 转置矩阵
template<int m, int n, typename T>
Mat<n, m, T> matTransposed(const Mat<m, n, T> &a) {
    Mat<n, m, T> ret;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            ret.data_[j][i] = a.data_[i][j];
        }
    }
    return ret;
}

// 逆矩阵
//template<int n, typename T>
//Mat<



//-------------------------------------------------------------------------------------------------
// 类型别名

using Vec2i = Vec<2, int>;
using Vec2f = Vec<2, float>;
using Vec2d = Vec<2, double>;

using Vec3i = Vec<3, int>;
using Vec3f = Vec<3, float>;
using Vec3d = Vec<3, double>;

using Vec4i = Vec<4, int>;
using Vec4f = Vec<4, float>;
using Vec4d = Vec<4, double>;

using Mat33f = Mat<3, 3, float>;
using Mat44f = Mat<4, 4, float>;
using Mat34f = Mat<3, 4, float>;
using Mat43f = Mat<4, 3, float>;

using Mat33d = Mat<3, 3, double>;
using Mat44d = Mat<4, 4, double>;
using Mat34d = Mat<3, 4, double>;
using Mat43d = Mat<4, 3, double>;

using Point3d = Vec3d;	//3D point
using Color = Vec3d;		//RGB color

#endif //TOYRAYTRACER_GEOMETRY_H
