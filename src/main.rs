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
mod camera;

// use vector::{Vector};
use std::process::exit;
use std::env;

fn print_help(binary: &String) {
    println!("Usage: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}

fn display_shere(_camera: camera::Camera)
{
    let _pointSphere = point::Point3D::init_point(0.5, 0.5, 1.0);
    let sphere = sphere::Sphere::init_sphere(_pointSphere, 0.1);

    for y in (0..=100).rev() {
        for x in 0..=100 {
            let _ray = _camera.ray(x as f64 / 100.0, y as f64 / 100.0);
            let _hits = sphere.hits(_ray);
            if _hits {
                print!("O");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    // println!("Ray: {:?}", _ray2);


}

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() == 2 && args[1].eq("--help") {
        print_help(&args[0]);
        exit(0);
    }
    let _vector = vector::Vector::init_vector(1.0, 0.0, 0.0);
    let _vector2 = vector::Vector::init_vector(0.0, 1.0, 0.0);
    let _pointrec = point::Point3D::init_point(0.0, 0.0, 1.0);
    let _point = point::Point3D::init_point(0.0, 0.0, 0.0);
    // let _ray = ray::Ray::init_ray(_point, _vector);
    let _rectangle =
    rectangle::Rectangle3D::init_rectangle(_pointrec, _vector, _vector2);

    let _camera: camera::Camera = camera::Camera::init_camera(_point, _rectangle);

    display_shere(_camera);

    // println!("Point: {:?}", _point2);
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
