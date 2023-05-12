//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
pub struct Screen {

}

impl Screen {
    pub fn init_screen() -> Screen {
        Screen { }
    }

    pub fn calcul_pixel_color(&self, shape_color: RGB, coefficients: f64, distance: f64) -> RGB {

        let mut rgb: RGB = RGB::init_rgb(0, 0, 0);
        rgb.r = (((shape_color.r as f64 * coefficients) as u64 / 100) as f64 / (distance as f64 / 255.0 as f64)) as u64;
        rgb.g = (((shape_color.g as f64 * coefficients) as u64 / 100) as f64 / (distance as f64 / 255.0 as f64)) as u64;
        rgb.b = (((shape_color.b as f64 * coefficients) as u64 / 100) as f64 / (distance as f64 / 255.0 as f64)) as u64;
        rgb.r = rgb.r.clamp(0, 255);
        rgb.g = rgb.g.clamp(0, 255);
        rgb.b = rgb.b.clamp(0, 255);
        return rgb;
    }

    pub fn calcul_coefficients(&self, ray: Ray, mut normal: Vector) -> f64 {
        let mut tmp_ray = ray.direction;
        tmp_ray.normalize();
        let coefficients: f64 = normal.dot_product(tmp_ray);
        return coefficients * 100.0;
    }

    pub fn render(&self, ray: Ray, file: &mut File, sphere: &mut sphere::Sphere, plan: &mut plan::Plan, lights: Vec<Light>) {

        let intersection_sphere: Option<Point3D> = sphere.hits(ray);
        let intersection_plan: Option<Point3D> = plan.hits(ray);

            if intersection_sphere != None {
                let mut coefficient = 0.0;
                let mut light_ray = Ray::init_ray(lights[1].origine, sphere.intersection_point.vectorize(lights[1].origine));
                coefficient += self.calcul_coefficients(light_ray, sphere.normal);
                for light in lights {
                    light_ray = Ray::init_ray(light.origine, sphere.intersection_point.vectorize(light.origine));
                    coefficient += self.calcul_coefficients(light_ray, sphere.normal);
                    if (sphere.distance > sphere.calcul_distance_between_point(light_ray)) {
                        sphere.distance = sphere.calcul_distance_between_point(light_ray);
                    }
                    // if (coefficient > 1.0) {
                    //     coefficient = 1.0;
                    // }
                    // if self.calcul_coefficients(light_ray, sphere.normal) > coefficient {
                    // }
                }
                println!("distance: {}", sphere.distance);
                sphere.rgb = self.calcul_pixel_color(sphere.initial_rgb, coefficient, sphere.distance * 100.0);
                write_pixel(file, &sphere.rgb);
                // let light_ray = Ray::init_ray(lights[0].origine, sphere.intersection_point.vectorize(lights[0].origine));
                // let coefficient = self.calcul_coefficients(light_ray, sphere.normal);
                // sphere.rgb = self.calcul_pixel_color(sphere.initial_rgb, coefficient, sphere.distance * 100.0);
                // write_pixel(file, &sphere.rgb);
            } else if intersection_plan != None && plan.distance > 0.0 {
                let light_ray = Ray::init_ray(lights[0].origine, plan.intersection_point.vectorize(lights[0].origine));
                let coefficient = self.calcul_coefficients(light_ray, plan.normal);
                plan.rgb = self.calcul_pixel_color(plan.initial_rgb, coefficient, plan.distance);
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
