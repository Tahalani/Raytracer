//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// sphere
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;
use core::ops::Sub;

#[derive(Debug)]

pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
}

impl Sphere {
    pub fn init_sphere(center: Point3D, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
    pub fn hits(&self, ray: Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.clone().dot_product(ray.direction.clone());
        let b = 2.0 * oc.dot_product(ray.direction);
        let c = oc.dot_product(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        discriminant > 0.0
    }
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
