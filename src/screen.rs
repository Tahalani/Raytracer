//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use serde::Deserialize;
use crate::point::Point3D;
use crate::camera;
use crate::sphere;
use crate::cylinder;
use crate::cone;
use crate::rgb::RGB;
use crate::vector::Vector;
use crate::write_ppm::{write_pixel, create_file};
use crate::plan;
use crate::heritage::HeritageHits;
use crate::ray::Ray;
use crate::light::Light;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Screen {

}

impl Screen {
    pub fn init_screen() -> Screen {
        Screen {}
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

    pub fn calcul_rgb(&self, coefficients: f64, mut distance: f64, r1: u64, g1: u64, b1: u64) -> RGB {
        distance *= 100.0;
        let mut rgb: RGB = RGB::init_rgb(0, 0, 0);
            rgb.r = (((((coefficients as u64 * r1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.g = (((((coefficients as u64 * g1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.b = (((((coefficients as u64 * b1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.r = rgb.r.clamp(0, 255);
            rgb.g = rgb.g.clamp(0, 255);
            rgb.b = rgb.b.clamp(0, 255);
        return rgb;
    }

    pub fn calcul_coefficients(&self, ray: Ray, normal: Vector) -> f64 {
        let mut tmp_ray = ray.direction;
        tmp_ray.normalize();
        let coefficients: f64 = normal.dot_product(tmp_ray);
        return coefficients * 100.0;
    }

    pub fn render_sphere(&self, sphere: &mut sphere::Sphere, file: &mut File, lights: Vec<Light>) {

        let mut coefficient = 0.0;
        let mut distance = 1000000.0;
        let mut tmp_distance;

        for light in lights {
            if light.direction.x == 0.0 && light.direction.y == 0.0 && light.direction.z == 0.0 {
                let light_ray = Ray::init_ray(light.origine, sphere.intersection_point.vectorize(light.origine));
                tmp_distance = sphere.calcul_distance_between_point(light_ray);
                if (tmp_distance < distance || distance == 0.0) && tmp_distance > 0.0 {
                    distance = tmp_distance;
                }
                if self.calcul_coefficients(light_ray, sphere.normal) > coefficient {
                    coefficient = self.calcul_coefficients(light_ray, sphere.normal);
                }
            } else {
                let light_ray = Ray::init_ray(light.origine, light.origine.vectorize(sphere.intersection_point));
                if light.direction.dot_product(light_ray.direction) > 0.0 {
                    tmp_distance = sphere.calcul_distance_between_point(light_ray);
                    if tmp_distance < distance && tmp_distance > 0.0 {
                        distance = tmp_distance;
                        coefficient = 80.0;
                    }
                } else {
                    write_pixel(file, &RGB::init_rgb(0, 0, 0));
                    return;
                }
            }
        }
        sphere.rgb = self.calcul_pixel_color(sphere.initial_rgb, coefficient, distance * 100.0);
        write_pixel(file, &sphere.rgb);
    }

    pub fn render_plan(&self, plan: &mut plan::Plan, file: &mut File, lights: Vec<Light>) {
        let mut coefficient = 0.0;
        let mut distance = 0.0;
        for light in lights {
            let light_ray = Ray::init_ray(light.origine, plan.intersection_point.vectorize(light.origine));
            if self.calcul_coefficients(light_ray, plan.normal) > coefficient {
                coefficient = self.calcul_coefficients(light_ray, plan.normal);
                distance = plan.distance;
            }
        }
        plan.rgb = self.calcul_pixel_color(plan.initial_rgb, coefficient, distance);
        write_pixel(file, &plan.rgb);
    }


    pub fn render_cylinder(&self, ray: Ray, file: &mut File, cylinder: &mut cylinder::Cylinder) {
        let coefficient = self.calcul_coefficients(ray, cylinder.normal);
        cylinder.rgb = self.calcul_rgb(coefficient, cylinder.distance, cylinder.initial_rgb.r, cylinder.initial_rgb.g, cylinder.initial_rgb.b);
        write_pixel(file, &cylinder.rgb);
    }

    pub fn render_cone(&self, ray: Ray, file: &mut File, cone: &mut cone::Cone) {
        let coefficient = self.calcul_coefficients(ray, cone.normal);
        cone.rgb = self.calcul_rgb(coefficient, cone.distance, cone.initial_rgb.r, cone.initial_rgb.g, cone.initial_rgb.b);
        write_pixel(file, &cone.rgb);
    }

    pub fn render(&self, ray: Ray, file: &mut File, sphere: &mut sphere::Sphere, plan: &mut plan::Plan, lights: Vec<Light>, cylinder: &mut cylinder::Cylinder, cone: &mut cone::Cone) {

        let intersection_sphere: Option<Point3D> = sphere.hits(ray);
        let intersection_plan: Option<Point3D> = plan.hits(ray);
        let intersection_cylinder: Option<Point3D> = cylinder.hits(ray);
        let intersection_cone: Option<Point3D> = cone.hits(ray);

        if intersection_sphere != None {
            self.render_sphere(sphere, file, lights);
        } else if intersection_cylinder != None && (intersection_cylinder.unwrap() - cylinder.center_bottom).dot_product(cylinder.hauteur) < cylinder.hauteur.norm() {
            self.render_cylinder(ray, file, cylinder);
        } else if intersection_cone != None && (intersection_cone.unwrap() - cone.center_top).dot_product(cone.hauteur) < cone.oc.norm() && (intersection_cone.unwrap() - cone.center_bottom).dot_product(cone.hauteur) > 0.0 {
            self.render_cone(ray, file, cone);
        } else if intersection_plan != None && plan.distance > 0.0 {
            self.render_plan(plan, file, lights);
        } else {
            write_pixel(file, &RGB::init_rgb(0, 0, 0));
        }
    }

    pub fn display_screen(&self, _camera: camera::Camera, mut sphere: sphere::Sphere, mut plan: plan::Plan, lights: Vec<Light>, mut cylinder: cylinder::Cylinder, mut cone: cone::Cone) {
        let width = 1000;
        let height = 1000;
        let mut file = create_file(width + 1, height + 1);

        for y in (0..=height).rev() {
            for x in 0..=width {
                let ray = _camera.ray(x as f64 / width as f64, y as f64 / height as f64);
                self.render(ray, &mut file, &mut sphere, &mut plan, lights.clone(), &mut cylinder, &mut cone);
            }
        }
    }
}
