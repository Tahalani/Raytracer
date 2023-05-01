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
mod rectangle;

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
    let _vector = vector::Vector::init_vector(1.0, 0.0, 0.0);
    let _vector2 = vector::Vector::init_vector(0.0, 1.0, 0.0);
    let _point = point::Point3D::init_point(0.0, 0.0, 0.0);
    let _ray = ray::Ray::init_ray(_point, _vector);
    let _rectangle = rectangle::Rectangle3D::init_rectangle(_point, _vector, _vector2);
    let _point2 = _rectangle.pointAt(1.0, 0.0);

    println!("Point: {:?}", _point2);
    // let _sphere = sphere::Sphere::init_sphere(_point, 10.0);
    // let _hits = _sphere.hits(_ray);
    // println!("Hits: {}", _hits);
    // let _sphere = sphere::Sphere::init_sphere(_point, 10.0);
    // let _vector3 = _vector - _vector2;

    // let _point2 = _point + _vector;
    // println!("Vector: {:?}", _vector3);
    // println!("Point: {:?}", _point2);
    // let _length = _vector.length_vector();
    // println!("Length: {}", _length);
    // dbg!(args);
}
