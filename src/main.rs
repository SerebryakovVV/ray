mod image;
mod vector3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod interval;
use image::Image;
use vector3::{Color, Point, Vector3};
use ray::Ray;
use camera::Camera;
use crate::hittable::{Hittable, HittableList, HittableObject};
use crate::interval::Interval;
use crate::sphere::Sphere;
use std::f64::consts::PI;    
use std::f64::INFINITY;
use rand::{self, Rng};

// look into implementing Deref trait and newtype pattern

// the direction of the vector is just the ammount of movement in each coordinate direction
// normalization is the same but its mapped onto 0-1 range


// pixel spacing - distance between pixels



// optimize the random generation


const ASPECT_RATIO: f64 = 16.0 / 9.0; 
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = {
  let temp = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
  if temp < 1 {1} else {temp}
};
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);


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
  world.list.push(HittableObject::Sphere(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
  world.list.push(HittableObject::Sphere(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
  cam.render(&world);
  cam.img.open_image();
}