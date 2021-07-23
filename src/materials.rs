use crate::objects::HitRecord;
use crate::raytracer::Ray;
use crate::vector::Vector;
use crate::Rgb;

pub trait Material {
    // Produces a new adjusted Ray depending on the material that scatters the original ray
    fn scatter(&self, ray: &Ray, hitrecord: HitRecord) -> Option<Ray>;

    fn get_albedo(&self) -> Rgb;
}

pub struct Diffuse {
    pub albedo: Rgb,
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hitrecord: HitRecord) -> Option<Ray> {
        let target: Vector = hitrecord.at + hitrecord.normal + Vector::random_in_unit_sphere();
        Some(Ray::new(hitrecord.at, target - hitrecord.at))
    }

    fn get_albedo(&self) -> Rgb {
        self.albedo
    }
}

impl Diffuse {
    pub fn new(albedo: Rgb) -> Self {
        Self { albedo }
    }
}

pub struct Metal {
    pub albedo: Rgb,
}

impl Material for Metal {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    fn scatter(&self, _ray: &Ray, hitrecord: HitRecord) -> Option<Ray> {
        let reflected_dir = self.reflect(&_ray.dir.normalize(), &hitrecord.normal);
        let scattered = Ray::new(hitrecord.at, reflected_dir);
        if Vector::dot(&scattered.dir, &hitrecord.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }

    fn get_albedo(&self) -> Rgb {
        self.albedo
    }
}

impl Metal {
    pub fn new(albedo: Rgb) -> Self {
        Self { albedo }
    }

    fn reflect(&self, incoming: &Vector, normal: &Vector) -> Vector {
        *incoming - 2.0 * Vector::dot(incoming, normal) * normal
    }
}
