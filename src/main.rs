mod materials;
mod objects;
mod raytracer;
mod vector;

extern crate image;

use crate::materials::{Lambertian, Metal};
use crate::objects::*;
use crate::vector::Vector;

const WIDTH: usize = 600;

fn main() {
    let aspect_ratio = AspectRatio::new(2, 1);
    let height = aspect_ratio.calc_height(WIDTH);
    let world = get_world();
    let viewplane = raytracer::run(world, WIDTH, aspect_ratio);
    create_image(&viewplane, WIDTH, height);
}

fn get_world() -> HitableList {
    let mut hitables: Vec<Box<dyn Hitable>> = Vec::new();
    let mat1 = Box::new(Lambertian {});
    let mat2 = Box::new(Metal {});
    hitables.push(Box::new(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        0.5,
        mat1.clone(),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        mat2.clone(),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        mat2.clone(),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        mat1.clone(),
    )));
    HitableList::new(hitables)
}

pub struct AspectRatio {
    pub w: usize,
    pub h: usize,
}

impl AspectRatio {
    pub fn new(w: usize, h: usize) -> AspectRatio {
        AspectRatio { w, h }
    }

    pub fn calc_height(&self, width: usize) -> usize {
        (width / self.w) * self.h
    }

    pub fn resize(&self, val: f32) -> f32 {
        (self.w as f32 / self.h as f32) * val
    }
}

type Rgb = Vector;

fn create_image(matrix: &Vec<Vec<Rgb>>, width: usize, height: usize) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgb = matrix[y as usize][x as usize] * 255.0;
        *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    }
    imgbuf.save("fractal.png").unwrap();
}
