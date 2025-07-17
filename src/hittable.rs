use crate::{ray::Ray, vector3::{Point, Vector3}};
use crate::sphere::Sphere;
use crate::interval::Interval;

pub enum HittableObject {
  Sphere(Sphere)
}

impl Hittable for HittableObject {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    match self {
      HittableObject::Sphere(s) => s.hit(ray, ray_t)
    }
  }
}

pub struct HitRecord {
  pub p: Point,
  pub normal: Vector3,
  pub t: f64,
  pub front_face: bool
}


impl HitRecord {
  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
    self.front_face = Vector3::dot(ray.direction, outward_normal) < 0.0;
    self.normal = if self.front_face {outward_normal} else {-outward_normal};
  }
}


pub trait Hittable {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}



pub struct HittableList {
  pub list: Vec<HittableObject>
}


impl HittableList {
  pub fn new() -> Self {
    Self {
      list: Vec::new()
    }
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let mut hit_rec: Option<HitRecord> = None;
    let mut closest_so_far = ray_t.max;
    for obj in &self.list {
      if let Some(temp_rec) = obj.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
        closest_so_far = temp_rec.t;
        hit_rec = Some(temp_rec); 
      }
    }
    hit_rec
  }
}
