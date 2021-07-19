use crate::objects::HitRecord;
use crate::raytracer::Ray;
use crate::vector::Vector;

pub trait Material {
    // Produces a new adjusted Ray depending on the material that scatters the original ray
    fn scatter(&self, ray: &Ray, hitrecord: HitRecord) -> Ray;
}

#[derive(Copy, Clone)]
pub struct Lambertian {}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hitrecord: HitRecord) -> Ray {
        let target: Vector = hitrecord.at + hitrecord.normal + Vector::random_in_unit_sphere();
        return Ray::new(hitrecord.at, target - hitrecord.at);
    }
}

#[derive(Copy, Clone)]
pub struct Metal {}

impl Material for Metal {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    fn scatter(&self, _ray: &Ray, hitrecord: HitRecord) -> Ray {
        let reflected_dir = self.reflect(hitrecord.at, hitrecord.normal);
        Ray::new(hitrecord.at, reflected_dir)
    }
}

impl Metal {
    fn reflect(&self, incoming: Vector, normal: Vector) -> Vector {
        incoming - 2.0 * Vector::dot(&incoming, &normal) * normal
    }
}
