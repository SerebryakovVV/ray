use std::path::{Path, PathBuf};
use std::fmt::Write;



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

    fn clear_data(&mut self) {
        self.data.clear();
    }

}


fn main() {
    // let mut img = Image::new(256, 256, "image.ppm");
    // img.prepare_metadata();
    // println!("{}", img.data);

    println!("P3");
    println!("256 256");
    println!("255");
    for i in 0..256 {
        for k in 0..256 {
            let r = k as f64 / 255.0f64; 
            let g = i as f64 / 255.0f64; 
            let b = 0.0f64; 
        
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
        
            println!("{} {} {}", ir, ig, ib);
        }
    }

 
}
