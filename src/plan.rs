//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// plan
//

use serde::Deserialize;

use crate::heritage::HeritageHits;
use crate::light::Light;
use crate::point::Point3D;
use crate::ray::Ray;
use crate::rgb::RGB;
use crate::screen::Screen;
use crate::vector::Vector;
use std::fs::File;
use crate::write_ppm::write_pixel;

#[derive(Debug, Deserialize, Clone, Copy)]

pub struct Plan {
    pub normal: Vector,
    pub origin: Point3D,
    pub intersection_point: Point3D,
    pub distance: f64,
    pub rgb: RGB,
    pub initial_rgb: RGB,
}

impl HeritageHits for Plan {
    fn hits(&mut self, ray: Ray) -> Option<Point3D> {
        let product = ray.direction.dot_product(self.normal);
        let discriminant = (self.origin - ray.origin).dot_product(self.normal) / product;
        self.intersection_point = ray.origin + (ray.direction * discriminant);
        self.distance = self.calcul_distance_between_point(ray) * 100.0;
        if discriminant < 0.0 {
            return None;
        }
        return Some(ray.origin + (ray.direction * discriminant));
    }
    fn who(&self) -> String {
        return String::from("Plan");
    }
    fn render_obj(&mut self, file: &mut File, lights: &Vec<Light>, render: &Screen) {
        let mut coefficient = 0.0;
        let mut distance = 0.0;
        for light in lights {
            let light_ray = Ray::init_ray(
                light.origin,
                self.intersection_point.vectorize(light.origin),
            );
            if render.calcul_coefficients(light_ray, self.normal) > coefficient {
                coefficient = render.calcul_coefficients(light_ray, self.normal);
                distance = self.distance;
            }
        }
        self.rgb = render.calcul_pixel_color(self.initial_rgb, coefficient, distance);
        write_pixel(file, &self.rgb);
    }
}

impl Plan {
    pub fn _init_plan(normal: Vector, origin: Point3D) -> Plan {
        Plan {
            normal,
            origin,
            intersection_point: Point3D::init_point(0.0, 0.0, 0.0),
            distance: 0.0,
            rgb: RGB::init_rgb(255, 255, 255),
            initial_rgb: RGB::init_rgb(255, 255, 255),
        }
    }
    pub fn calcul_distance_between_point(&mut self, ray: Ray) -> f64 {
        let x = self.intersection_point.x - ray.origin.x;
        let y = self.intersection_point.y - ray.origin.y;
        let z = self.intersection_point.z - ray.origin.z;
        let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        return distance;
    }
}

impl<'de> Plan {
    pub fn create_plan<M>(data: serde_json::Value) -> Result<Box<dyn HeritageHits>, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let plan: Plan;
        match serde_json::from_value(data) {
            Ok(obj) => plan = obj,
            Err(err) => {
                println!("Error: {}", err);
                return Err(serde::de::Error::custom("Error"));
            }
        }
        Ok(Box::new(plan) as Box<dyn HeritageHits>)
    }
}
