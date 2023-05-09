//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// plan
//

use serde::Deserialize;

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::rgb::RGB;


#[derive(Debug, Deserialize, Clone, Copy)]

pub struct Plan {
    pub normal : Vector,
    pub origin : Point3D,
    pub intersection_point: Point3D,
    pub coefficients: f64,
    pub distance: f64,
    pub rgb: RGB,
}

impl Plan {
    pub fn init_plan(normal : Vector, origin : Point3D) -> Plan {
        Plan { normal, origin, intersection_point: Point3D::init_point(0.0, 0.0, 0.0),
        coefficients: 0.0, distance: 0.0, rgb: RGB::init_rgb(0, 255, 255)}
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

    pub fn calcul_distance_between_point(&mut self, ray: Ray) -> f64 {
        let x = self.intersection_point.x - ray.origin.x;
        let y = self.intersection_point.y - ray.origin.y;
        let z = self.intersection_point.z - ray.origin.z;
        let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        return distance;
    }

    pub fn calcul_rgb(&mut self) {

        self.rgb.r = (255.0 / (self.distance as f64 / 250.0)) as u64;
        self.rgb.g = (255.0 / (self.distance as f64 / 250.0)) as u64;
        self.rgb.b = (255.0 / (self.distance as f64 / 250.0)) as u64;

        if self.rgb.r > 255 {
            self.rgb.r = 0;
        }
        if self.rgb.g > 255 {
            self.rgb.g = 0;
        }
        if self.rgb.b > 255 {
            self.rgb.b = 0;
        }
        if self.rgb.r == 0 {
            self.rgb.r = 255;
        }
        if self.rgb.g == 0 {
            self.rgb.g = 255;
        }
        if self.rgb.b == 0 {
            self.rgb.b = 255;
        }
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
            self.distance = self.calcul_distance_between_point(ray) * 100.0;

            self.calcul_rgb();

            return true;
        }
    }
}
