//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// point
//

use serde::Deserialize;

use crate::vector::Vector;

use std::ops::{Add};
use std::ops::{Sub};

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sub for Point3D {
    type Output = Vector;
    fn sub(self, other: Point3D) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3D {
    pub fn init_point(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn vectorize(&self, other: Point3D) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
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
