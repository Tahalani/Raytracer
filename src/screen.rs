//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use crate::point::Point3D;
// use crate::rectangle::Rectangle3D;
use crate::camera;
use crate::sphere;
use crate::rgb::RGB;
use crate::vector;
use crate::write_ppm::{write_pixel, create_file};
use crate::plan;
use crate::heritage::HeritageHits;
use crate::ray::Ray;
use crate::vector::Vector;
use crate::plan::Plan;
use crate::light::Light;
use std::fs::File;

pub struct Screen {

}

impl Screen {
    pub fn init_screen() -> Screen {
        Screen { }
    }

    pub fn calcul_rgb(&self, coefficients: f64, mut distance: f64, r1: u64, g1: u64, b1: u64) -> RGB {
        distance *= 100.0;
        let mut rgb: RGB = RGB::init_rgb(0, 0, 0);
        // rgb.g = (((g1 / (coefficients as u64  + 1)) * 100)) as u64;
        // rgb.r = (((r1 / (coefficients as u64  + 1)) * 100)) as u64;
        // rgb.b = (((b1 / (coefficients as u64  + 1)) * 100)) as u64;
            rgb.r = (((((coefficients as u64 * r1) / 100)) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.g = (((((coefficients as u64 * g1) / 100)) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.b = (((((coefficients as u64 * b1) / 100)) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.r = rgb.r.clamp(0, 255);
            rgb.g = rgb.g.clamp(0, 255);
            rgb.b = rgb.b.clamp(0, 255);
        return rgb;
    }

    pub fn calcul_rgb_plan(&self, plan: &mut Plan, initial_rgb: RGB) -> RGB {

        let mut rgb: RGB = RGB::init_rgb(0, 0, 0);
        rgb.r = (255.0 / (plan.distance as f64 / 255.0 as f64)) as u64;
        rgb.g = (255.0 / (plan.distance as f64 / 255.0 as f64)) as u64;
        rgb.b = (255.0 / (plan.distance as f64 / 255.0 as f64)) as u64;
        rgb.r = rgb.r.clamp(0, 255);
        rgb.g = rgb.g.clamp(0, 255);
        rgb.b = rgb.b.clamp(0, 255);
        return rgb;
    }

    pub fn calcul_coefficients(&self, ray: Ray, mut normal: Vector) -> f64 {
        let ray = normal.normalize(ray.direction);
        let coefficients: f64 = normal.dot_product(ray);
        return coefficients * 100.0;
    }

    pub fn render(&self, ray: Ray, file: &mut File, sphere: &mut sphere::Sphere, plan: &mut plan::Plan, lights: Vec<Light>) {

        let intersection_sphere: Option<Point3D> = sphere.hits(ray);
        let intersection_plan: Option<Point3D> = plan.hits(ray);

            if !intersection_sphere.is_none() {
                let light_ray = Ray::init_ray(lights[0].origine, sphere.intersection_point.vectorize(lights[0].origine));
                let coefficient = self.calcul_coefficients(light_ray, sphere.normal);
                sphere.rgb = self.calcul_rgb(coefficient, sphere.distance, sphere.inital_rgb.r, sphere.inital_rgb.g, sphere.inital_rgb.b);
                write_pixel(file, &sphere.rgb);
            } else if !intersection_plan.is_none() && plan.distance > 0.0 {
                let light_ray = Ray::init_ray(lights[0].origine, plan.intersection_point.vectorize(lights[0].origine));
                plan.rgb = self.calcul_rgb_plan(plan, plan.rgb);
                write_pixel(file, &plan.rgb);
            } else {
                write_pixel(file, &RGB::init_rgb(0, 0, 0));
            }
    }

    pub fn display_screen(&self, _camera: camera::Camera, mut sphere: sphere::Sphere, mut plan: plan::Plan, lights: Vec<Light>) {
        let width = 1000;
        let height = 1000;
        let mut file = create_file(width + 1, height + 1);

        for y in (0..=height).rev() {
            for x in 0..=width {
                let ray = _camera.ray(x as f64 / width as f64, y as f64 / height as f64);
                self.render(ray, &mut file, &mut sphere, &mut plan, lights.clone());
            }
        }
    }
}
