//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//


use std::fs::File;
use std::io::prelude::*;
use crate::camera::Camera;
use crate::heritage::HeritageHits;
use serde::Deserialize;
use crate::sphere::Sphere;
use std::fmt::Pointer;
use crate::plan::Plan;
use crate::ray::Ray;



#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub primitive: Vec<Box<dyn HeritageHits>>,
}

enum Primitivetype {
    Sphere,
    Plan,
}

impl<'de> Deserialize<'de> for Box<dyn HeritageHits> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map::<HeritageHitsVisitor<Sphere>>(HeritageHitsVisitor(std::marker::PhantomData))

    }
}


// impl<'de> Deserialize<'de> for Box<dyn HeritageHits> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let concrete_value: Sphere = Deserialize::deserialize(deserializer)?;
        
//         println!("{:?}", concrete_value);
//         // Convert the concrete value to a Box<dyn HeritageHits>
//         let dynamic_value = Box::new(concrete_value) as Box<dyn HeritageHits>;
//         Ok(dynamic_value)
//     }
// }

struct HeritageHitsVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for HeritageHitsVisitor<T>
where
    T: HeritageHits + Deserialize<'de> + 'static,
{
    type Value = Box<dyn HeritageHits>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a struct that implements the HeritageHits trait")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        
        let value = T::deserialize(serde::de::value::MapAccessDeserializer::new(&mut map))?;

        let dynamic_value = Box::new(value) as Box<dyn HeritageHits>;
        Ok(dynamic_value)
    }
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
