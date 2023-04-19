//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// ray
//

use crate::vector::Vector;
use crate::point::Point3D;

#[derive(Debug)]

pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector,
}

impl Ray {
    pub fn init_ray(origin: Point3D, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}
