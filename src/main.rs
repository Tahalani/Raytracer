//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// main
//

mod vector;
mod point;

// use vector::{Vector};

use std::env;

fn print_help(binary: &String) {
    println!("Usage: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() == 2 && args[1].eq("--help") {
        print_help(&args[0]);
    }
    let _vector = vector::Vector::init_vector(1.0, 2.0, 3.0);
    let _vector2 = vector::Vector::init_vector(3.0, 1.0, 8.0);
    let _point = point::Point3D::init_vector(1.0, 2.0, 3.0);
    let _vector3 = _vector - _vector2;
    println!("Vector: {:?}", _vector3);
    // let _length = _vector.length_vector();
    // println!("Length: {}", _length);
    dbg!(args);
}
