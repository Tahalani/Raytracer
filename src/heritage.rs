//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// heritage
//

use core::fmt;
use crate::ray::Ray;
use crate::point::Point3D;
use std::option::Option;

pub trait HeritageHits {
    fn hits(&mut self, ray: Ray) -> Option<Point3D>;
    fn who(&self) -> String;
    fn mehdi(&self) -> String {
        return String::from("Mehdi");
    }
}