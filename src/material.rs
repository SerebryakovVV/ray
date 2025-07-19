use crate::{hittable::HitRecord, ray::Ray, vector3::{Color, Vector3}};

#[derive(Clone, Copy)]
pub enum MaterialType {
  Metal(Metal),
  Lambertian(Lambertian)
}

impl Material for MaterialType {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    match self {
      MaterialType::Metal(m)      => m.scatter(r_in, rec),
      MaterialType::Lambertian(l) => l.scatter(r_in, rec),
    }
  }
}




// instead of output parameters we have an option tuple
pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}


#[derive(Clone, Copy)]
pub struct Lambertian {
  albedo: Color
}


impl Lambertian {
  pub fn new(albedo: Color) -> Self {
    Self {albedo}
  } 
}


impl Material for Lambertian {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let mut scattered_direction = rec.normal + Vector3::random_unit_vector();
    if scattered_direction.near_zero() {
      scattered_direction = rec.normal;
    }
    let scattered = Ray::new(rec.p, scattered_direction);
    let attenuation = self.albedo;
    Some((attenuation, scattered))
  }
}



#[derive(Clone, Copy)]
pub struct Metal {
  albedo: Color,
  fuzz: f64
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Self {
    Self {
      albedo, 
      fuzz: if fuzz < 1.0 {fuzz} else {1.0}
    }
  } 
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let mut reflected = Vector3::reflect(r_in.direction, rec.normal);
    reflected = Vector3::unit_vector(&reflected) + (self.fuzz * Vector3::random_unit_vector());
    let scattered = Ray::new(rec.p, reflected);
    let attenuation = self.albedo;
    if Vector3::dot(scattered.direction, rec.normal) > 0.0 {
      Some((attenuation, scattered))
    } else {
      None
    }
  }
}