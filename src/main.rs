use std::io::{self, stderr, Stderr, Write as IoWrite};
use std::path::{Path, PathBuf};
use std::fmt::Write as FmtWrite;
use std::thread::sleep;



struct Image<'a> {
    width: i32,
    height: i32,
    data: String,
    out_dest: &'a Path
}


impl<'a> Image<'a> {
    
    fn new(w: i32, h: i32, path: &'a str) -> Self {
        Self {
            width: w,
            height: h,
            data: String::new(),
            out_dest: Path::new(path)
        }
    }
    
    fn prepare_metadata(&mut self) {
        self.data.push_str("P3\n");
        write!(self.data, "{} {}\n", self.width, self.height).unwrap();
        self.data.push_str("255\n");
    }

    fn construct_append_main_data(&mut self) {
        todo!();
    }

    fn make_image_one(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        for i in 0..self.height {
            eprintln!("Remaining lines: {}", self.height - i);
            std::io::stderr().flush().unwrap();
            for k in 0..self.width {
                let color = Color {
                    rgb: Vec3 {
                        x: k as f64 / (self.width-1) as f64,
                        y: i as f64 / (self.height-1) as f64,
                        z: 0.0f64
                    }
                };
                color.draw();
            }
        }
        eprint!("done");
    }

    fn clear_data(&mut self) {
        self.data.clear();
    }

}




struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        // self.direction.new_mul_scal_vec(t).new_sum_vec(self.origin);
    }
}


struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}



struct Color {
    rgb: Vec3
}

impl Color {
    fn draw(&self) {
        let r = self.rgb.x;
        let g = self.rgb.y;
        let b = self.rgb.z;

        let rbyte = (r * 255.999) as u8;
        let gbyte = (g * 255.999) as u8;
        let bbyte = (b * 255.999) as u8;

        println!("{} {} {}", rbyte, gbyte, bbyte);
    }
}

impl Vec3 {
    fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    fn add_vec(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;            // return the struct, maybe
    }

    fn mul_scal(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;             // return the struct, maybe
    }

    fn div_scal(&mut self, s: f64) {
        self.x /= s;
        self.y /= s;
        self.z /= s;         // return the struct, maybe
    }

    fn new_div_scal_vec(&self, s: f64) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s

        }
    }

    fn length_squared(&self) -> f64 {
        self.x*self.x
       +self.y*self.y
       +self.z*self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

    fn new_sum_vec(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }

    fn new_subtract_vec(&self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }

    fn new_mul_vec(&self, v: Self) -> Self {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }

    fn new_mul_scal_vec(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }

    fn dot(&self, v: Self) -> f64 {
        self.x * v.x
       +self.y * v.y
       +self.z * v.z
    }

    fn cross(&self, v: Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }
    
    fn unit_vec(&self) -> Self {
        self.new_div_scal_vec(self.length())
    }

}




fn main() {
    let mut img = Image::new(256, 256, "image.ppm");
    img.make_image_one();
    // img.prepare_metadata();
    // println!("{}", img.data);



 
}
