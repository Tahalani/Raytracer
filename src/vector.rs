//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// vector
//

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
#[derive(Clone, Debug, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn init_vector(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
    pub fn length_vector(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn dot_product(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self, vector: Vector) -> Vector {
        let lenght = (vector.x.powi(2) + vector.y.powi(2) + vector.z.powi(2)).sqrt();
        let vector = Vector {
            x: vector.x / lenght,
            y: vector.y / lenght,
            z: vector.z / lenght,
        };
        return vector;
    }
    pub fn norm(&mut self) -> f64 {
        let lenght = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        return lenght;
    }
    pub fn cross_product(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, other: Vector) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
