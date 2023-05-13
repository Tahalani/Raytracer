//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// parsing
//

use crate::camera::Camera;
use crate::heritage::HeritageHits;
use crate::plan::Plan;
use crate::ray::Ray;
use crate::sphere::Sphere;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

pub struct Primitive(Box<dyn HeritageHits>);

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub primitive: Vec<Primitive>,
}

#[derive(Deserialize)]
enum Primitivetype {
    Sphere,
    Plan,
}

impl<'de> serde::Deserialize<'de> for Primitive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PrimitiveVisitor;

        impl<'de> serde::de::Visitor<'de> for PrimitiveVisitor {
            type Value = Primitive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A boc of dyn HeritageHits")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut primitive: Option<Box<dyn HeritageHits>> = None;

                while let Some((typ, data)) = map.next_entry::<Primitivetype, serde_json::Value>()?
                {
                    let primitive = match typ {
                        Primitivetype::Sphere => primitive =  Some(Sphere::create_sphere::<M>(data)?),
                        Primitivetype::Plan => primitive = Some(Plan::create_plan::<M>(data)?),
                        _ => return Err(serde::de::Error::custom("Error")),
                    };
                }
                Ok(Primitive(primitive.unwrap()))
            }
        }
        deserializer.deserialize_map(PrimitiveVisitor)
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

        let mut data: Scene = serde_json::from_str(&contents).unwrap();


        for i in data.primitive.iter() {
            println!("{}", i.0.mehdi());
        }
        

        return Parsing { scene: data };
    }
}
