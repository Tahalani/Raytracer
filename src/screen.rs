//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use crate::point::Point3D;
use crate::rectangle::Rectangle3D;
use crate::camera;
use crate::sphere;
use crate::rgb::RGB;
use crate::write_ppm::{write_pixel, create_file};
use crate::plan;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Screen {
    rectangle: Rectangle3D,
}

impl Screen {
    pub fn init_screen(rectangle: Rectangle3D) -> Screen {
        Screen { rectangle }
    }

    pub fn calcul_rgb(&self, coefficients: f64, mut distance: f64, r1: u64, g1: u64, b1: u64) -> RGB {
        distance *= 100.0;
        let mut rgb: RGB = RGB::init_rgb(0, 0, 0);
            rgb.r = (((((coefficients as u64 * r1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.g = (((((coefficients as u64 * g1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            rgb.b = (((((coefficients as u64 * b1) / 100) + 1) as f64 / (distance as f64 / 250.0)) + 1.0) as u64;
            if rgb.r > 255 {
                rgb.r = 0;
            }
            if rgb.g > 255 {
                rgb.g = 0;
            }
            if rgb.b > 255 {
                rgb.b = 0;
            }
            if rgb.r == 0 {
                rgb.r = 255;
            }
            if rgb.g == 0 {
                rgb.g = 255;
            }
            if rgb.b == 0 {
                rgb.b = 255;
            }
        return rgb;
    }

    pub fn calcul_coefficients(&self, ray: Ray, mut normal: Vector) -> f64 {
        let ray = normal.normalize(ray.direction);
        let coefficients: f64 = normal.dot_product(ray);
        return coefficients * 100.0;
    }

    pub fn display_screen(&self, _camera: camera::Camera, mut sphere: sphere::Sphere, mut plan: plan::Plan) {
        let width = 1000;
        let height = 1000;
        let mut file = create_file(width + 1, height + 1);

        for y in (0..=height).rev() {
            for x in 0..=width {
                let _ray = _camera.ray(x as f64 / width as f64, y as f64 / height as f64);
                let hits_sphere: Option<Point3D> = sphere.hits(_ray);
                let _hits_plan = plan.hits(_ray);

                if hits_sphere != None {
                    let coefficient = self.calcul_coefficients(_ray, sphere.normal);
                    sphere.rgb = self.calcul_rgb(coefficient, sphere.distance, sphere.inital_rgb.r, sphere.inital_rgb.g, sphere.inital_rgb.b);
                    write_pixel(&mut file, &RGB {
                        r: sphere.rgb.r,
                        g: sphere.rgb.g,
                        b: sphere.rgb.b,
                });
                } else if _hits_plan && plan.distance > 0.0 {
                    write_pixel(&mut file, &RGB {
                        r: plan.rgb.r,
                        g: plan.rgb.g,
                        b: plan.rgb.b,
                    });
                } else {
                    write_pixel(&mut file, &RGB {
                        r: 0,
                        g: 0,
                        b: 0,
                    });
                }
            }
        }
    }
}
