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
use crate::heritage::HeritageHits;

#[derive(Debug, Deserialize, Clone, Copy)]

pub struct Plan {
    pub normal : Vector,
    pub origin : Point3D,
    pub intersection_point: Point3D,
    pub distance: f64,
    pub rgb: RGB,
}

impl HeritageHits for Plan {
    fn hits(&mut self, ray: Ray) -> Option<Point3D> {
        let product = ray.direction.dot_product(self.normal);
        let discriminant = (self.origin - ray.origin).dot_product(self.normal) / product;
        if discriminant < 0.0 {
            return None;
        }
        self.intersection_point = ray.origin + (ray.direction * discriminant);
        self.distance = self.calcul_distance_between_point(ray) * 100.0;
        return Some(self.intersection_point);
    }
}

impl Plan {
    pub fn init_plan(normal : Vector, origin : Point3D) -> Plan {
        Plan { normal, origin, intersection_point: Point3D::init_point(0.0, 0.0, 0.0),
        distance: 0.0, rgb: RGB::init_rgb(0, 255, 255)}
    }
    pub fn calcul_distance_between_point(&mut self, ray: Ray) -> f64 {
        let x = self.intersection_point.x - ray.origin.x;
        let y = self.intersection_point.y - ray.origin.y;
        let z = self.intersection_point.z - ray.origin.z;
        let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        return distance;
    }
}
