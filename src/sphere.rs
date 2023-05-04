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
}

impl Sphere {
    pub fn init_sphere (center: Point3D, radius: f64, intersection_point: Point3D) -> Sphere {
        Sphere { center, radius, intersection_point}
    }
    pub fn hits(&mut self, ray: Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.clone().dot_product(ray.direction.clone());
        let b = 2.0 * oc.dot_product(ray.direction);
        let c = oc.dot_product(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        let solution = (-b + discriminant.sqrt()) / (2.0 * a);
        let tmp = ray.origin + (ray.direction * solution);
        let normal = tmp - self.center;
        let lenght = (normal.x.powi(2) + normal.y.powi(2) + normal.z.powi(2)).sqrt();
        let normal = Vector {
            x: normal.x / lenght,
            y: normal.y / lenght,
            z: normal.z / lenght,
        };
        let length_ray = (ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2)).sqrt();
        let ray = Vector {
            x: ray.direction.x / length_ray,
            y: ray.direction.y / length_ray,
            z: ray.direction.z / length_ray,
        };
        let coef = normal.dot_product(ray);
        println!("coef: {}", coef);
        // normal.dot_product(normal);
        // println!("normal: {:?} et point {:?}", normal, self.intersection_point);
        self.intersection_point = tmp;
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
