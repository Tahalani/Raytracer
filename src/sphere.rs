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
    pub intersection_point: Point3D,
    pub coefficients: f64,
}

impl Sphere {
    pub fn init_sphere (center: Point3D, radius: f64, intersection_point: Point3D) -> Sphere {
        Sphere { center, radius, intersection_point, coefficients: 0.0}
    }

    pub fn calcul_discriminant(&mut self, ray: Ray, a: &mut f64, mut b: &mut f64) -> f64 {
        let oc = ray.origin - self.center;
        *a = ray.direction.clone().dot_product(ray.direction.clone());
        *b = (2.0 * oc.dot_product(ray.direction));
        let c = oc.dot_product(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * (*a) * c;
        return discriminant;
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

    pub fn calcul_normal(&mut self, ray: Ray, a: f64, b: f64, discriminant: f64) -> Vector {
        let solution = (-b + discriminant.sqrt()) / (2.0 * a);
        self.intersection_point  = ray.origin + (ray.direction * solution);
        let normal = self.intersection_point - self.center;
        let normal = self.normalize(normal);
        return normal;
    }

    pub fn calcul_coefficients(&mut self, ray: Ray, normal: Vector) -> f64 {
        let ray = self.normalize(ray.direction);
        let coefficients: f64 = normal.dot_product(ray);
        return coefficients;
    }

    pub fn hits(&mut self, ray: Ray) -> bool {
        let mut a = 0.0;
        let mut b = 0.0;
        let discriminant = self.calcul_discriminant(ray, &mut a, &mut b);
        let normal = self.calcul_normal(ray, a, b, discriminant);
        self.coefficients = self.calcul_coefficients(ray, normal);

        if discriminant < 0.0 {
            return false;
        } else {
            return true;
        }
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
