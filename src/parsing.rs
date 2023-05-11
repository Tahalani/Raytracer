//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//

use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::plan::Plan;
use crate::heritage::HeritageHits;
use crate::vector;
use crate::point;
use crate::rectangle;
use crate::screen;
use crate::camera;
use crate::sphere;

pub struct Scene {
    pub camera: Camera,
    pub sphere: Option<Vec<Sphere>>,
    pub plan: Option<Vec<Plan>>,
    pub Iprimitive: Vec<Box<dyn HeritageHits>>,
}

pub struct Parsing {
    pub scene: Scene,
}

impl Parsing {
    pub fn init_parsing(name: String) -> Parsing {

        let mut file = File::open(name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let bottom_side = vector::Vector::init_vector(1.0, 0.0, 0.0);
        let left_side = vector::Vector::init_vector(0.0, 1.0, 0.0);
        let origin_rectangle = point::Point3D::init_point(0.0, 0.0, 1.0);
        let _rectangle = rectangle::Rectangle3D::init_rectangle(origin_rectangle, bottom_side, left_side);

        let screen = screen::Screen::init_screen();

        let origin_cam = point::Point3D::init_point(0.5, 0.5, 0.0);
        let _camera: camera::Camera = camera::Camera::init_camera(origin_cam, _rectangle);

        let point_sphere = point::Point3D::init_point(0.5, 0.5, 2.0);
        let point_intersection = point::Point3D::init_point(0.0, 0.0, 0.0);
        let sphere = sphere::Sphere::init_sphere(point_sphere, 0.2, point_intersection);

        let data: Scene;
        data.Iprimitive.push(Box::new(sphere));


        let data: Scene = serde_json::from_str(&contents).unwrap();


        return Parsing {scene: data};
    }
}
