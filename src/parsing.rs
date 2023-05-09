//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//

use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::plan::Plan;


#[derive(Deserialize, Debug)]
pub struct Scene {
    pub camera: Camera,
    pub sphere: Option<Vec<Sphere>>,
    pub plan: Option<Vec<Plan>>,
}

pub struct Parsing {
    pub scene: Scene,
}


impl Parsing {
    pub fn init_parsing(name: String) -> Parsing {

        let mut file = File::open(name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let data: Scene = serde_json::from_str(&contents).unwrap();


        return Parsing {scene: data};
    }
}
