//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// write_ppm
//

use std::fs::File;
use std::io::Write;

pub struct RGB {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub fn create_file(width: u32, height: u32) -> File {
    let mut file = File::create("output.ppm").unwrap();
    let content = format!("P3\n{} {}\n255\n", width, height);

    file.write_all(content.as_bytes()).unwrap();
    return file;
}

pub fn write_pixel(file: &mut File, pixel: &RGB) {
    let content = format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);
    file.write_all(content.as_bytes()).unwrap();
}
