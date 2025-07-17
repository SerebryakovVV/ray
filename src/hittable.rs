use crate::{ray::Ray, vector3::{Point, Vector3}};


pub struct HitRecord {
  pub p: Point,
  pub normal: Vector3,
  pub t: f64
}


pub trait Hittable {
  fn hit(ray: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}