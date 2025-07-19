use crate::{hittable::HitRecord, ray::Ray, vector3::{Color, Vector3}};

#[derive(Clone, Copy)]
pub enum MaterialType {
  Metal(Metal),
  Lambertian(Lambertian),
  Dielectric(Dielectric)
}

impl Material for MaterialType {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    match self {
      MaterialType::Metal(m)      => m.scatter(r_in, rec),
      MaterialType::Lambertian(l) => l.scatter(r_in, rec),
      MaterialType::Dielectric(d) => d.scatter(r_in, rec),
    }
  }
}




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







#[derive(Clone, Copy)]
pub struct Dielectric {
  refraction_index: f64
}

impl Dielectric {
  pub fn new(refraction_index: f64) -> Self {
    Self {refraction_index}
  }

  pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let attenuation = Color::new(1.0, 1.0, 1.0);
    let ri = if rec.front_face {
      1.0 / self.refraction_index
    } else {
      self.refraction_index
    };
    let unit_direction = Vector3::unit_vector(&r_in.direction);
    let cos_theta = 1.0f64.min(Vector3::dot(-unit_direction, rec.normal));
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let cannot_refract = ri * sin_theta > 1.0;
    let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rand::random() {
      Vector3::reflect(unit_direction, rec.normal)
    } else {
      Vector3::refract(unit_direction, rec.normal, ri)
    };
    let scattered = Ray::new(rec.p, direction);
    Some((attenuation, scattered))
  }
}