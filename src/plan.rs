//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// plan
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;


#[derive(Debug)]

pub struct Plan {
    pub normal : Vector,
    pub origin : Point3D,
    pub intersection_point: Point3D,
    pub coefficients: f64,
}

impl Plan {
    pub fn init_Plan(normal : Vector, origin : Point3D) -> Plan {
        Plan { normal, origin, intersection_point: Point3D::init_point(0.0, 0.0, 0.0), coefficients: 0.0}
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

    pub fn calcul_coefficients(&mut self, ray: Ray, normal: Vector) -> f64 {
        let ray = self.normalize(ray.direction);
        let coefficients: f64 = normal.dot_product(ray);
        return coefficients;
    }

    pub fn hits(&mut self, ray: Ray) -> bool {
        let product = ray.direction.dot_product(self.normal);

        if product == 0.0 {
            return false;
        } else {

            let d = (self.origin - ray.origin).dot_product(self.normal) / product;

            if d < 0.0 {
                return false;
            }

            self.intersection_point = ray.origin + (ray.direction * d);
            let normal_normalize = self.normalize(self.normal);
            self.coefficients = self.calcul_coefficients(ray, normal_normalize);
            println!("plan: {:?}", self.coefficients);
            return true;
        }
    }
}
