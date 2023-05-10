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
mod screen;
mod write_ppm;
mod plan;
mod rgb;
mod heritage;
mod light;

use std::process::exit;
use std::env;

fn print_help(binary: &String) {
    println!("Usage: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}

fn algo() {

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

    let point_plan = point::Point3D::init_point(0.0, 0.0, 0.0);
    let normal_plan = vector::Vector::init_vector(0.0, 1.0, 0.0);
    let plan = plan::Plan::init_plan(normal_plan, point_plan);

    let cam_light = point::Point3D::init_point(1.0, 1.0, 2.0);
    let lights = vec![light::Light::init_light(cam_light, rgb::RGB::init_rgb(125, 125, 125), 255.0)];
    screen.display_screen(_camera, sphere, plan, lights);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() == 2 && args[1].eq("--help") {
        print_help(&args[0]);
        exit(0);
    } else {
        algo();
    }

}
