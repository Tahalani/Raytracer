//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// heritage
//

use crate::ray::Ray;
use crate::point::Point3D;
use std::option::Option;
use std::fs::File;
use crate::light::Light;
use crate::screen::Screen;

pub trait HeritageHits {
    fn hits(&mut self, ray: Ray) -> Option<Point3D>;
    fn who(&self) -> String;
    fn render_obj(& mut self, file: &mut File, lights: &Vec<Light>, render: &Screen);
}