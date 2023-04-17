//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// point
//

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
#[derive(Debug)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn init_point(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
    pub fn length_point(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
