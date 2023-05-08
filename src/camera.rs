//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// camera
//

use serde::Deserialize;

use crate::rectangle::Rectangle3D;
use crate::point::Point3D;
use crate::ray::Ray;

#[derive(Clone, Copy, Deserialize, Debug)]

pub struct Camera {
    pub origin: Point3D,
    pub screen: Rectangle3D,
}

impl Camera {
    pub fn init_camera(origin: Point3D, screen: Rectangle3D) -> Camera {
        Camera { origin, screen }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let screen_point = self.screen.point_at(u, v);
        let direction = screen_point - self.origin;
        return Ray::init_ray(self.origin, direction);
    }
}
