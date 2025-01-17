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
mod light;
mod parsing;

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
    } else if args.len() == 2 {
        let mut parsing = parsing::Parsing::init_parsing(args[1].to_string());
        parsing.render.display_screen(parsing.camera, parsing.light, & mut parsing.primitive);
    }
    else {
        println!("Error: invalid number of arguments");
        print_help(&args[0]);
        exit(84);
    }
}
