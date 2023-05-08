//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//

use std::fs::File;
use std::io::prelude::*;
use serde_json::{Result};
use serde::{Deserialize};
use crate::{camera::Camera};


#[derive(Deserialize, Debug)]
struct Scene {
    camera: Camera,
}

pub struct Parsing {
    pub name: String,
}


impl Parsing {
    pub fn init_parsing(name: String) -> Result<()> {

        let mut file = File::open(name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let data: Scene = serde_json::from_str(&contents)?;
        println!("{:?}", data);

        Ok(())
    }
}
