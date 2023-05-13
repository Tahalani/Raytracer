//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// torus
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::heritage::HeritageHits;
use crate::rgb::RGB;

#[derive(Debug)]

pub struct Triangle {
    pub left: Point3D,
    pub right: Point3D,
    pub top: Point3D,
    pub x: Vector,
    pub y: Vector,
    pub z: Vector,
    pub radius: f64,
    pub intersection_point: Point3D,
    pub normal: Vector,
    pub rgb: RGB,
    pub distance: f64,
    pub inital_rgb: RGB,
}

impl HeritageHits for Triangle {
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

impl Triangle {
    pub fn init_triangle(left: Point3D, right : Point3D, top : Point3D, radius: f64, intersection_point: Point3D) -> Triangle {
        Triangle {left, right, top, radius, intersection_point, rgb: RGB::init_rgb(255, 0, 255), normal: Vector::init_vector(0.0, 0.0, 0.0),
            distance: 0.0, inital_rgb: RGB::init_rgb(255, 0, 255), x: Vector::init_vector(0.0, 0.0, 0.0), y: Vector::init_vector(0.0, 0.0, 0.0), z: Vector::init_vector(0.0, 0.0, 0.0)}
    }

    pub fn calcul_discriminant(&mut self, ray: Ray, a: &mut f64, b: &mut f64) -> f64 {
        self.x = self.right - self.left;
        self.y = self.top - self.right;
        self.z = self.left - self.top;
        let d = ray.origin - self.left;
        let e = ray.origin - self.right;
        let f = ray.origin - self.top;
        let c = self.x.cross_product(d);
        let g = self.y.cross_product(e);
        let h = self.z.cross_product(f);
        let discriminant = c.dot_product(self.normal);
        *a = g.dot_product(self.normal);
        *b = h.dot_product(self.normal);
        return discriminant;
    }

    pub fn calcul_normal(&mut self, ray: Ray, a: f64, b: f64, discriminant: f64) -> Vector {
        let mut normal = Vector::init_vector(0.0, 0.0, 0.0);
        if discriminant > 0.0 {
            normal = Vector::init_vector(self.normal.x * discriminant, self.normal.y * discriminant, self.normal.z * discriminant);
        } else if a > 0.0 {
            normal = Vector::init_vector(self.normal.x * a, self.normal.y * a, self.normal.z * a);
        } else if b > 0.0 {
            normal = Vector::init_vector(self.normal.x * b, self.normal.y * b, self.normal.z * b);
        }
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
