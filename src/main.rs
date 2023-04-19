//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// main
//

mod vector;
mod point;
mod ray;
mod sphere;

// use vector::{Vector};
use std::process::exit;
use std::env;

fn print_help(binary: &String) {
    println!("Usage: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() == 2 && args[1].eq("--help") {
        print_help(&args[0]);
        exit(0);
    }
    let _vector = vector::Vector::init_vector(10.0, 2.0, 3.0);
    let _vector2 = vector::Vector::init_vector(3.0, 1.0, 8.0);
    let _point = point::Point3D::init_point(1.0, 2.0, 3.0);
    let _ray = ray::Ray::init_ray(_point, _vector);
    // let _sphere = sphere::Sphere::init_sphere(_point, 10.0);
    // let _vector3 = _vector - _vector2;
    let _point2 = _point + _vector;
    // println!("Vector: {:?}", _vector3);
    println!("Point: {:?}", _point2);
    // let _length = _vector.length_vector();
    // println!("Length: {}", _length);
    // dbg!(args);
}
