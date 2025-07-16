use std::{fs::File, io::{self, BufWriter, Write}, path::PathBuf};

use crate::vector3::Color;

pub struct Image {
  pub width: u16,
  pub height: u16,
  writer: BufWriter<File>,
  path: PathBuf
}


impl Image {
  pub fn new(width: u16, height: u16, name: &str) -> io::Result<Self> {
    let path = PathBuf::from(format!("C:\\Users\\sereb\\main_files\\coding\\ray\\images\\{}.ppm", name));
    let img_file = File::create(&path)?;
    let mut writer = BufWriter::new(img_file);
    writeln!(writer, "P3\n{width} {height}\n255")?;
    Ok(Self {
      width,
      height,
      writer,
      path
    })
  }

  pub fn draw() {

  }

  pub fn write_color(&mut self, color: Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;
    let ir = (255.999 * r) as i16;
    let ig = (255.999 * g) as i16;
    let ib = (255.999 * b) as i16;
    let pixel_string = format!("{} {} {}", ir, ig, ib);
    writeln!(self.writer, "{pixel_string}").unwrap_or_else(|e| {
      println!("Error writing a pixel: {}", e);
      std::process::exit(0);
    }); 
  }

  pub fn open_image(self) {
    std::mem::drop(self.writer);
    std::process::Command::new("cmd").args([
      "/C",
      "start",
      self.path.to_str().unwrap()
    ]).status().unwrap_or_else(|e| {
      println!("Error opening the image: {}", e);
      std::process::exit(0);
    });
  }
}