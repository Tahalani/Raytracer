//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// cylinder
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::heritage::HeritageHits;
use crate::rgb::RGB;
use std::fs::File;
use crate::light::Light;
use crate::screen::Screen;
use serde::Deserialize;
use crate::write_ppm::write_pixel;

#[derive(Debug, Deserialize)]

pub struct Cylinder {
    pub center_top: Point3D,
    pub center_bottom: Point3D,
    pub radius: f64,
    pub intersection_point: Point3D,
    pub normal: Vector,
    pub rgb: RGB,
    pub distance: f64,
    pub initial_rgb: RGB,
    pub hauteur: Vector,
}

impl HeritageHits for Cylinder {
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
    fn who(&self) -> String {
        return String::from("Cylinder");
    }
    fn render_obj(&mut self, file: &mut File, _lights: &Vec<Light>, render: &Screen) {
        let coefficient = render.calcul_coefficients(render.ray.unwrap(), self.normal);
        self.rgb = render.calcul_rgb(coefficient, self.distance, self.initial_rgb.r, self.initial_rgb.g, self.initial_rgb.b);
        write_pixel(file, &self.rgb);
    }
}

impl<'de> Cylinder {
    pub fn create_cylinder<M>(data: serde_json::Value) -> Result<Box<dyn HeritageHits>, M::Error>
        where
            M: serde::de::MapAccess<'de>,
    {
        let cylinder: Cylinder;
        match serde_json::from_value(data) {
            Ok(obj) => cylinder = obj,
            Err(err) => {
                println!("Error: {}", err);
                return Err(serde::de::Error::custom("Error"));
            }
        }
        Ok(Box::new(cylinder) as Box<dyn HeritageHits>)

    }
}

impl Cylinder {
    pub fn init_cylinder(center_top: Point3D, center_bottom: Point3D, radius: f64, intersection_point: Point3D) -> Cylinder {
        Cylinder { center_top, center_bottom, radius, intersection_point, rgb: RGB::init_rgb(255, 0, 255), normal: Vector::init_vector(0.0, 0.0, 0.0),
            distance: 0.0, initial_rgb: RGB::init_rgb(255, 0, 255), hauteur: Vector::init_vector(0.0, 0.0, 0.0)}
    }
    pub fn calcul_discriminant(&mut self, ray: Ray, a: &mut f64, b: &mut f64) -> f64 {
        let oc = ray.origin - self.center_bottom;
        self.hauteur = ray.origin - self.center_bottom;
        let h = (self.center_top - self.center_bottom) / (self.center_top - self.center_bottom).norm();
        *a = (ray.direction.clone().dot_product(ray.direction.clone())) - ((ray.direction.clone().dot_product(h.clone())).powi(2));
        *b = 2.0 * (ray.direction.clone().dot_product(oc.clone()) - (ray.direction.clone().dot_product(h.clone()) * oc.clone().dot_product(h.clone())));
        let c = (oc.clone().dot_product(oc.clone())) - ((oc.clone().dot_product(h.clone())).powi(2)) - (self.radius.powi(2));
        let discriminant = b.powi(2) - 4.0 * (*a) * c;
        return discriminant;
    }
    pub fn calcul_normal(&mut self, ray: Ray, a: f64, b: f64, discriminant: f64) -> Vector {
        let solution = (-b + discriminant.sqrt()) / (2.0 * a);
        self.intersection_point  = ray.origin + (ray.direction * solution);
        let x = self.intersection_point.x - self.center_top.x;
        let y = self.intersection_point.y - self.center_top.y;
        let z = self.intersection_point.z - self.center_top.z;
        let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let normal = Vector::init_vector(x / distance, y / distance, z / distance);
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
