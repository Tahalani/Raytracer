//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// rgb
//

#[derive(Debug, Copy, Clone)]

pub struct RGB {
    pub r: u64,
    pub g: u64,
    pub b: u64,
}

impl RGB {
    pub fn init_rgb(r: u64, g: u64, b: u64) -> RGB {
        RGB { r, g, b }
    }
}
