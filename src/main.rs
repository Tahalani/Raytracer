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
mod cylinder;
mod cone;
mod triangle;
mod light;
mod parsing;

use std::process::exit;
use std::env;

fn print_help(binary: &String) {
    println!("Usage: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}

fn algo() {

    // let parsing = parsing::Parsing::init_parsing("test.json".to_string());

    // let screen = screen::Screen::init_screen();

    // screen.display_screen(parsing.scene.camera, parsing.scene.sphere.unwrap()[0], parsing.scene.plan.unwrap()[0]);

    let bottom_side = vector::Vector::init_vector(1.0, 0.0, 0.0);
    let left_side = vector::Vector::init_vector(0.0, 1.0, 0.0);
    let origin_rectangle = point::Point3D::init_point(0.0, 0.0, 1.0);
    let _rectangle = rectangle::Rectangle3D::init_rectangle(origin_rectangle, bottom_side, left_side);

    let screen = screen::Screen::init_screen();

    let origin_cam = point::Point3D::init_point(0.5, 0.5, 0.0);
    let _camera: camera::Camera = camera::Camera::init_camera(origin_cam, _rectangle);


    let point_sphere = point::Point3D::init_point(0.3, 0.17, 2.0);
    let point_intersection = point::Point3D::init_point(0.0, 0.0, 0.0);
    let sphere = sphere::Sphere::init_sphere(point_sphere, 0.2, point_intersection);

    let point_plan = point::Point3D::init_point(0.0, 0.0, 0.0);
    let normal_plan = vector::Vector::init_vector(0.0, 1.0, 0.0);
    let plan = plan::Plan::init_plan(normal_plan, point_plan);

    let point_bottom_cylinder = point::Point3D::init_point(0.1, 0.7, 2.0);
    let point_top_cylinder = point::Point3D::init_point(0.3, 0.9, 2.0);
    let point_intersection_cylinder = point::Point3D::init_point(0.0, 0.0, 0.0);
    let cylinder = cylinder::Cylinder::init_cylinder(point_bottom_cylinder, point_top_cylinder, 0.2, point_intersection_cylinder);

    let point_bottom_cone = point::Point3D::init_point(0.9, 0.1, 1.4);
    let point_top_cone = point::Point3D::init_point(0.4, 0.2, 1.4);
    let point_intersection_cone = point::Point3D::init_point(0.0, 0.0, 0.0);
    let cone = cone::Cone::init_cone(point_bottom_cone, point_top_cone, 0.2, point_intersection_cone);

    let point_triangle_left = point::Point3D::init_point(0.2, 0.4, 2.0);
    let point_triangle_right = point::Point3D::init_point(0.4, 0.6, 2.0);
    let point_triangle_top = point::Point3D::init_point(0.1, 0.8, 2.0);
    let point_intersection = point::Point3D::init_point(0.0, 0.0, 0.0);
    let triangle = triangle::Triangle::init_triangle(point_triangle_left, point_triangle_right, point_triangle_top, 0.2, point_intersection);

    let cam_light = point::Point3D::init_point(1.0, 1.0, 0.0);
    let cam_light2 = point::Point3D::init_point(0.1, 0.1, 0.0);
    let lights = vec![
        light::Light::init_light(cam_light, rgb::RGB::init_rgb(125, 125, 125), 255.0, vector::Vector::init_vector(0.0, 0.0, 0.0)), 
        light::Light::init_light(cam_light2, rgb::RGB::init_rgb(125, 125, 125), 255.0, vector::Vector::init_vector(0.0, 0.0, 0.0))
    ];
    screen.display_screen(_camera, sphere, plan, lights, cylinder, cone, triangle);
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
