//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//

use std::fs::File;
use std::io::prelude::*;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub obj_t: String,
    pub name: String,
    pub age: u8,
    pub city: String,
}

pub struct Parsing {
    pub name: String,
}


impl Parsing {
    pub fn init_parsing(name: String) -> Result<()> {
        // lire le fichier JSON dans une chaîne de caractères
        // let data = r#"
        //     {
        //         "name": "Alice",
        //         "age": 30,
        //         "city": "Paris"
        //     }
        // "#;
        let mut file = File::open(name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        let data: Value = serde_json::from_str(&contents)?;


        // convertir la chaîne JSON en une structure de données en Rust

        // accéder aux valeurs de la structure de données
        // let name = data["sphere"][0]["name"].as_str().unwrap();
        // let rayon = data["sphere"][0]["rayon"].as_i64().unwrap();
        // let volume = data["sphere"][0]["volume"].as_f64().unwrap();
        // let name = data["personne1"]["name"].as_str().unwrap();
        // let age = data["personne1"]["age"].as_i64().unwrap();
        // let city = data["personne1"]["city"].as_str().unwrap();

        // afficher les valeurs
        for obj in data["sphere"]["sphere"].as_array().as_iter()${
            println!("{}", obj[0]);
        }
        // println!("Name: {}", data["sphere"][0]);
        // println!("Age: {}", data.age);
        // println!("City: {}", data.citas_object().unwrap()["name"y);
        // let age = data["personne1"]["age"].as_i64().unwrap();
        // let city = data["personne1"]["city"].as_str().unwrap();

        // // afficher les valeurs
        // println!("Name: {}", name);
        // println!("rayon: {}", rayon);
        // println!("volume: {}", volume);

        // println!("City: {}", city);

        Ok(())
    }
}
