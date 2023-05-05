//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// rectangle
//

use crate::vector::Vector;
use crate::point::Point3D;

#[derive(Clone, Copy, Debug)]

pub struct Rectangle3D {
    pub origin: Point3D,
    pub bottom_side: Vector,
    pub left_side: Vector,
}

impl Rectangle3D {
    pub fn init_rectangle(origin: Point3D, bottom_side: Vector, left_side: Vector) -> Rectangle3D {
        Rectangle3D { origin, bottom_side, left_side }
    }

    pub fn point_at(&self, u: f64, v: f64) -> Point3D {
        self.origin + self.bottom_side * u + self.left_side * v
    }
}
