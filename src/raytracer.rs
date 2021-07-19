use crate::objects::*;
use crate::vector::Vector;
use crate::{AspectRatio, Rgb};
use rand::Rng;

const RAYS_PER_PIXEL: u16 = 100;

pub fn run(world: HitableList, width: usize, aspect_ratio: AspectRatio) -> Vec<Vec<Rgb>> {
    let height = aspect_ratio.calc_height(width);
    let mut image_matrix: Vec<Vec<Rgb>> = vec![vec![Rgb::default(); width]; height];

    let camera = Camera::new(aspect_ratio, -1.0);
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut color = Rgb::default();
            for _ in 0..RAYS_PER_PIXEL {
                // get UV coordinates
                // https://stackoverflow.com/questions/3314219/how-do-u-v-coordinates-work
                let rand_x: f32 = rng.gen();
                let rand_y: f32 = rng.gen();
                let u: f32 = (x as f32 + rand_x) / (width as f32);
                let v: f32 = (y as f32 + rand_y) / (height as f32);

                let r = camera.get_ray(u, v);
                color = color + calc_color(&r, &world);
            }
            image_matrix[height - y - 1][x] = color / RAYS_PER_PIXEL as f32; // todo implement div
        }
    }

    image_matrix
}

fn calc_color(ray: &Ray, world: &HitableList) -> Rgb {
    if let Some(hitrecord) = world.is_hit(ray, 0.001, f32::MAX) {
        let target: Vector = hitrecord.at + hitrecord.normal + Vector::random_in_unit_sphere();
        let new_ray = Ray::new(hitrecord.at, target - hitrecord.at);
        // 0.5 is just the factor by which the brightness gets reduced per bounce
        return 0.5 * calc_color(&new_ray, world);
    }
    let unit_direction = ray.dir.normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    let rgb_vec = (1.0 - t) * Vector::new_unit() + t * Vector::new(0.5, 0.7, 1.0);
    rgb_vec
}

pub struct Ray {
    pub origin: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(origin: Vector, dir: Vector) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, raypos: f32) -> Vector {
        self.origin + (raypos * self.dir)
    }
}

struct Camera {
    origin: Vector,
    // viewport
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: AspectRatio, focal_len: f32) -> Self {
        let origin = Vector::default();
        let viewp_h = 2.0;
        let viewp_w = aspect_ratio.resize(viewp_h);
        let lower_left_corner = Vector::new(
            origin.x - viewp_w / 2.0,
            origin.y - viewp_h / 2.0,
            focal_len,
        );
        Camera {
            origin,
            lower_left_corner,
            horizontal: Vector::new(viewp_w, 0.0, 0.0),
            vertical: Vector::new(0.0, viewp_h, 0.0),
        }
    }

    // UV-Coordinates
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.calculate_ray_dir(u, v),
        }
    }

    fn calculate_ray_dir(&self, u: f32, v: f32) -> Vector {
        self.lower_left_corner + u * self.horizontal + v * self.vertical
    }
}
