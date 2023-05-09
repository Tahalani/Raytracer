//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// point
//

use crate::vector::Vector;
use crate::ray::Ray;

use std::ops::{Add};
use std::ops::{Sub};

#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn init_point(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
    pub fn length_point(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Add<Vector> for Point3D {
    type Output = Point3D;
    fn add(self, other: Vector) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector> for Point3D {
    type Output = Point3D;
    fn sub(self, other: Vector) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
