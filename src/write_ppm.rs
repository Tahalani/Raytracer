//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// write_ppm
//

use std::fs::File;
use std::io::Write;

pub struct RGB {
    r: u32,
    g: u32,
    b: u32,
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

fn main() {
    let pixel = RGB {
        r: 255,
        g: 255,
        b: 255,
    };

    let mut file = create_file(10, 10);
    write_pixel(&mut file, &pixel);
    write_pixel(&mut file, &pixel);
}
// header

// si 10*10 faut 300 octets

// 255 255 255