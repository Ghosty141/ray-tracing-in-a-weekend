use rand::Rng;
use std::ops;

#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<f32> for Vector {
    type Output = Vector;

    fn add(self, rhs: f32) -> Vector {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Add<Vector> for f32 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        let k: f32 = 1.0 / rhs;
        Vector {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn new_unit() -> Vector {
        Vector::new(1.0, 1.0, 1.0)
    }

    pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v2.y * v2.z - v2.z * v1.y,
            y: -(v2.z * v2.x - v2.x * v1.z),
            z: v2.x * v2.y - v2.y * v1.x,
        }
    }

    pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Vector {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Vector {
        let mut rng = rand::thread_rng();
        loop {
            let rand_point =
                2.0 * Vector::new(rng.gen(), rng.gen(), rng.gen()) - Vector::new_unit();
            if rand_point.squared_length() < 1.0 {
                return rand_point;
            }
        }
    }
}
