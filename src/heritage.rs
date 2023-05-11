//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// heritage
//

use crate::ray::Ray;
use crate::point::Point3D;

pub trait HeritageHits {
    fn hits(&mut self, ray: Ray) -> Option<Point3D>;
}
