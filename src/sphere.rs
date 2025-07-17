use crate::vector3::Point;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;


pub struct Sphere {
  pub center: Point,
  pub radius: f64
}


// impl Hittable for Sphere {
//   fn hit(ray: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
    
//   }
// }