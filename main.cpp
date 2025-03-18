#include <iostream>
#include <cmath>









class vec3 {
    public:
        double e[3];

        vec3(): e{0,0,0} {}
        vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}

        double x() const {return e[0];}
        double y() const {return e[1];}
        double z() const {return e[2];}


        vec3 operator-() const {return vec3(-e[0], -e[1], -e[2]);} 
        double operator[](int i) const {return e[i];}
        double& operator[](int i) {return e[i];}

        vec3& operator+=(const vec3& v) {
            e[0] += v.e[0];
            e[1] += v.e[1];
            e[2] += v.e[2];
            return *this;
        }

        vec3& operator*=(double t) {
            e[0] *= t;
            e[1] *= t;
            e[2] *= t;
            return *this;
        }

        vec3& operator/=(double t) {
            return *this *= 1/t;
        }

        double length() const {
            return std::sqrt(length_squared());
        }

        double length_squared() const {
            return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
        }

};


using point3 = vec3;

inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
    return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
}

inline vec3 operator+(const vec3& u, const vec3& v) {
    return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
}

inline vec3 operator-(const vec3& u, const vec3& v) {
    return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
}

inline vec3 operator*(const vec3& u, const vec3& v) {
    return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
}

inline vec3 operator*(double t, const vec3& v) {
    return vec3(t*v.e[0], t*v.e[1], t*v.e[2]);
}

inline vec3 operator*(const vec3& v, double t) {
    return t*v;
}

inline vec3 operator/(const vec3& v, double t) {
    return (1/t) * v;
}

inline double dot(const vec3& u, const vec3& v) {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]; 
}

inline vec3 cross(const vec3& u, const vec3& v) {
    return vec3(
        u.e[1]*v.e[2] - u.e[2] * v.e[1],
        u.e[2]*v.e[0] - u.e[0] * v.e[2],
        u.e[0]*v.e[1] - u.e[1] * v.e[0]
    );
}

inline vec3 unit_vector(const vec3& v) {
    return v/v.length();
}






using color = vec3;
void write_color(std::ostream& out, const color pixel_color) {
    auto r = pixel_color.x();
    auto g = pixel_color.y();
    auto b = pixel_color.z();

    int rbyte = int(255.999 * r);
    int gbyte = int(255.999 * g);
    int bbyte = int(255.999 * b);

    out << rbyte << ' ' << gbyte << ' ' << bbyte << '\n';
}







int main() {
    int image_width = 256;
    int image_height = 256;
    std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    for (int j = 0; j < image_height; j++) {
        std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
        for (int i = 0; i < image_width; i++) {

            auto pixel_color = color(double(i) / (image_width-1), double(j) / (image_height-1), 0);
            write_color(std::cout, pixel_color);

            // auto r = double(i) / (image_width-1);
            // auto g = double(j) / (image_height-1);
            // auto b = 0.0;
            // int ir = int(255.999 * r);
            // int ig = int(255.999 * g);
            // int ib = int(255.999 * b);
            // std::cout << ir << ' ' << ig << ' ' << ib << '\n';
        }
    }
    std::clog << "\rDone.    \n";
}
