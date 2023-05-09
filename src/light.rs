//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// light
//

use crate::point::Point3D;
use crate::rgb::RGB;

#[derive(Clone, Copy, Debug)]

pub struct Light {
    pub origine: Point3D,
    pub rgb: RGB,
    pub efficiant_range: f64,
}

impl Light {
    pub fn init_light(origine: Point3D, rgb: RGB, efficiant_range: f64) -> Light {
        Light {
            origine,
            rgb,
            efficiant_range,
        }
    }
}
