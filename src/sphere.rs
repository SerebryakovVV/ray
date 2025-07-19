use crate::material::{MaterialType};
use crate::vector3::{Vector3, Point};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub mat: MaterialType
}


impl Sphere {
  pub fn new(center: Point, radius: f64, mat: MaterialType) -> Self {
    Self {
      center,
      radius: radius.max(0.0),
      mat
    }
  }
}


impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let oc = self.center - ray.origin;
    let a = ray.direction.length_squared();
    let h = Vector3::dot(ray.direction, oc);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
      return None;
    };
    let sqrtd = discriminant.sqrt();
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return None;
      }
    }
    // TODO: rewrite whatever this is
    let t = root;
    let p = ray.at(t);
    let outward_normal = (p - self.center) / self.radius;
    let material = self.mat;
    let mut hr = HitRecord {
      t,
      p,
      normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
      front_face: false,
      material
    };
    hr.set_face_normal(ray, outward_normal);
    Some(hr) 
  }
}