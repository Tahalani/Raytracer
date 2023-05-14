//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use serde::Deserialize;
use crate::point::Point3D;
use crate::camera;
use crate::cylinder;
use crate::rgb::RGB;
use crate::vector::Vector;
use crate::write_ppm::{write_pixel, create_file};
use crate::heritage::HeritageHits;
use crate::ray::Ray;
use crate::light::Light;
use std::fs::File;


#[derive(Deserialize, Debug)]
pub struct Screen {
    width: u32,
    height: u32,
    pub ray: Option<Ray>,
}

impl Screen {
    pub fn init_screen(width: u32, height: u32) -> Screen {
        Screen {width, height, ray: Some(Ray::init_ray(Point3D::init_point(0.0, 0.0, 0.0), Vector::init_vector(0.0, 0.0, 0.0)))}
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

    pub fn render_cylinder(&self, ray: Ray, file: &mut File, cylinder: &mut cylinder::Cylinder) {
        let coefficient = self.calcul_coefficients(ray, cylinder.normal);
        cylinder.rgb = self.calcul_rgb(coefficient, cylinder.distance, cylinder.initial_rgb.r, cylinder.initial_rgb.g, cylinder.initial_rgb.b);
        write_pixel(file, &cylinder.rgb);
    }

    pub fn render(&self, ray: Ray, file: &mut File, lights: &Vec<Light>, primitive: & mut Vec<Box<dyn HeritageHits>>) {

        let mut intersection: Option<Point3D> = None;
        for i in primitive.iter_mut() {
            intersection = i.hits(ray);
            if intersection != None {
                i.render_obj(file, lights, self);
                break;
            }
        }
        if intersection == None {
            write_pixel(file, &RGB::init_rgb(0, 0, 0));
        }

    }

    pub fn display_screen(&mut self, _camera: camera::Camera, lights: Vec<Light>, primitive: & mut Vec<Box<dyn HeritageHits>>) {
        let mut file = create_file(self.width + 1, self.height + 1);

        for y in (0..=self.height).rev() {
            for x in 0..=self.width {
                self.ray = Some(_camera.ray(x as f64 / self.width as f64, y as f64 / self.height as f64));
                self.render(self.ray.unwrap(), &mut file, &lights, primitive);
            }
        }
    }
}
