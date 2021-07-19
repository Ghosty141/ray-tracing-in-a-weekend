use crate::materials::Material;
use crate::raytracer::Ray;
use crate::vector::Vector;

pub trait Hitable {
    // For an explanatin why t_min/t_max are needed see the book "Ray Tracing Gems" chapter 2.2
    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord<'h> {
    pub normal: Vector,
    pub ray_pos: f32,
    pub at: Vector,
    pub material: &'h Box<dyn Material>,
}

pub struct HitableList {
    hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hitrec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in &self.hitables {
            if let Some(hr) = hitable.is_hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.ray_pos;
                hitrec = Some(hr);
            }
        }
        hitrec
    }
}

impl HitableList {
    pub fn new(hitables: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { hitables }
    }
}

pub struct Sphere {
    center: Vector,
    rad: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            rad: radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn is_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Formula:
        // raypos² * dot(ray.dir, ray.dir)
        //  + 2raypos * dot(ray.dir, ray.origin - sphere.center)
        //  + dot(ray.origin - sphere.center, ray.origin - sphere.center)
        //  - sphere.radius² == 0
        //  -> this is a quadratic formular -> via the quadratic formula use the discriminant
        let oc = ray.origin - self.center;
        let a = Vector::dot(&ray.dir, &ray.dir);
        let b = 2.0 * Vector::dot(&ray.dir, &oc);
        let c = Vector::dot(&oc, &oc) - self.rad * self.rad;
        // If discriminant < 0  -> no intersection (missed)
        // If discriminant = 0  -> 1 intersection (tangent)
        // If discriminant > 0  -> 2 intersections (intersected)
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        // to get the ray position, solve the quadratic formula
        // then calculate the surface normal (sqhere center to ray_pos)
        let discr_sqrt = discriminant.sqrt();
        let mut ray_pos: f32 = (-b - discr_sqrt) / (2.0 * a);

        if ray_pos < t_min || ray_pos > t_max {
            // calculate the positive result of the quadratic formula
            ray_pos = (-b + discr_sqrt) / (2.0 * a);
            if ray_pos < t_min || ray_pos > t_max {
                return None;
            }
        }

        let intersection_point: Vector = ray.at(ray_pos);
        Some(HitRecord {
            normal: (intersection_point - self.center).normalize(),
            at: intersection_point,
            ray_pos,
            material: &self.material,
        })
    }
}
