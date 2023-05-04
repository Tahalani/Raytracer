//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// ray
//

use crate::vector::Vector;
use crate::point::Point3D;
use core::ops::Mul;
use core::ops::Add;

#[derive(Debug, Copy, Clone)]

pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector,
}

impl Ray {
    pub fn init_ray(origin: Point3D, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}

impl Mul<f64> for Ray {
    type Output = Ray;
    fn mul(self, other: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction * other,
        }
    }
}

impl Add<Vector> for Ray {
    type Output = Ray;
    fn add(self, other: Vector) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction + other,
        }
    }
}