// ffmpeg -i .\images\glass_full_hd.ppm -update 1 .\images\glass_full_hd.png

mod image;
mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod interval;
mod material;

use image::Image;
use vector3::{Color, Point};
use camera::Camera;
use crate::hittable::{HittableList, HittableObject};
use crate::material::{Dielectric, Lambertian, MaterialType, Metal};
use crate::sphere::Sphere;
use std::f64::consts::PI;    


const ASPECT_RATIO: f64 = 16.0 / 9.0; 
const IMAGE_WIDTH: u16 = 1920;
const IMAGE_HEIGHT: u16 = {
  let temp = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
  if temp < 1 {1} else {temp}
};


fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}


fn main() {
  let name = std::env::args().collect::<Vec<String>>()
                             .into_iter()
                             .skip(1)
                             .next()
                             .unwrap_or_else(||{
                                println!("No name provided");
                                std::process::exit(0);});  
  let img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, &name).unwrap_or_else(|e| {
    println!("Error occured while creating a file: {}", e);
    std::process::exit(0);
  });
  let mut cam = Camera::new(img);  
  let mut world = HittableList::new();
  let material_ground = MaterialType::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
  let material_center = MaterialType::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
  let material_left = MaterialType::Dielectric(Dielectric::new(1.5));
  let material_bubble = MaterialType::Dielectric(Dielectric::new(1.0 / 1.5));
  let material_right = MaterialType::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
  world.list.push(
    HittableObject::Sphere(
      Sphere::new(
        Point::new(0.0,  0.0, -1.5), 
        0.5,
        material_center
      )
    )
  );
  world.list.push(
    HittableObject::Sphere(
      Sphere::new(
        Point::new(0.0, -100.5, -1.0), 
        100.0,
        material_ground
      )
    )
  );
  world.list.push(
  HittableObject::Sphere(
    Sphere::new(
      // Point::new(-1.0, 0.0, -1.0), 
      Point::new(-0.8, 0.0, -1.0), 
      0.5,
      material_left
    )
  )
  );
  world.list.push(
  HittableObject::Sphere(
    Sphere::new(
      // Point::new(-1.0, 0.0, -1.0), 
      Point::new(-0.8, 0.0, -1.0), 
      0.4,
      material_bubble
    )
  )
  );
  world.list.push(
    HittableObject::Sphere(
      Sphere::new(
        Point::new(1.0, 0.0, -1.0), 
        0.5,
        material_right
      )
    )
  );
  cam.render(&world);
  cam.img.open_image();
}