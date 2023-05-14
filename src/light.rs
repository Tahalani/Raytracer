//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// light
//


use crate::point::Point3D;
use crate::rgb::RGB;
use crate::vector::Vector;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]

pub struct Light {
    pub origin: Point3D,
    pub rgb: RGB,
    pub efficiant_range: f64,
    pub direction: Vector,
}

impl Light {
    pub fn init_light(origin: Point3D, rgb: RGB, efficiant_range: f64, direction: Vector) -> Light {
        Light {
            origin,
            rgb,
            efficiant_range,
            direction,
        }
    }
}
