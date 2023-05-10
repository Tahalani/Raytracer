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
use crate::heritage::HeritageHits;
use crate::rgb::RGB;

#[derive(Debug)]

pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub intersection_point: Point3D,
    pub normal: Vector,
    pub rgb: RGB,
    pub distance: f64,
    pub inital_rgb: RGB,
}

impl HeritageHits for Sphere {
    fn hits(&mut self, ray: Ray) -> Option<Point3D> {
        let mut a = 0.0;
        let mut b = 0.0;
        let discriminant = self.calcul_discriminant(ray, &mut a, &mut b);
        self.normal = self.calcul_normal(ray, a, b, discriminant);
        self.distance = self.calcul_distance_between_point(ray);

        if discriminant < 0.0 {
            return None;
        } else {
            return Some(self.intersection_point);
        }
    }
}

impl Sphere {
    pub fn init_sphere (center: Point3D, radius: f64, intersection_point: Point3D) -> Sphere {
        Sphere { center, radius, intersection_point, rgb: RGB::init_rgb(255, 0, 255), normal: Vector::init_vector(0.0, 0.0, 0.0),
            distance: 0.0, inital_rgb: RGB::init_rgb(255, 0, 255)}
    }

    pub fn calcul_discriminant(&mut self, ray: Ray, a: &mut f64, b: &mut f64) -> f64 {
        let oc = ray.origin - self.center;
        *a = ray.direction.clone().dot_product(ray.direction.clone());
        *b = 2.0 * oc.dot_product(ray.direction);
        let c = oc.dot_product(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * (*a) * c;
        return discriminant;
    }

    pub fn calcul_normal(&mut self, ray: Ray, a: f64, b: f64, discriminant: f64) -> Vector {
        if discriminant == 0.0 {
            self.intersection_point  = ray.origin + (ray.direction * (-b / (2.0 * a)));
        } else {
            let solution1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let solution2 = (-b - discriminant.sqrt()) / (2.0 * a);
            if solution1 < solution2 {
                self.intersection_point  = ray.origin + (ray.direction * solution1);
            } else {
                self.intersection_point  = ray.origin + (ray.direction * solution2);
            }
        }
        // self.intersection_point  = ray.origin + (ray.direction * solution);
        let mut normal = self.intersection_point - self.center;
        normal.normalize();
        return normal;
    }

    pub fn calcul_distance_between_point(&mut self, ray: Ray) -> f64 {
        let x = self.intersection_point.x - ray.origin.x;
        let y = self.intersection_point.y - ray.origin.y;
        let z = self.intersection_point.z - ray.origin.z;
        let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        return distance;
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
