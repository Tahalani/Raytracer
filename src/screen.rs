//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use crate::rectangle::Rectangle3D;
use crate::camera;
use crate::sphere;
use crate::write_ppm::{write_pixel, create_file, RGB};

pub struct Screen {
    rectangle: Rectangle3D,
}

impl Screen {
    pub fn init_screen(rectangle: Rectangle3D) -> Screen {
        Screen { rectangle }
    }

    pub fn display_screen(&self, _camera: camera::Camera, sphere: sphere::Sphere)
    {
        let mut file = create_file(101, 101);

        for y in (0..=100).rev() {
            for x in 0..=100 {
                let _ray = _camera.ray(x as f64 / 100.0, y as f64 / 100.0);
                let _hits = sphere.hits(_ray);
                if _hits {
                    write_pixel(&mut file, &RGB {
                        r: 255,
                        g: 0,
                        b: 255,
                    });
                } else {
                    write_pixel(&mut file, &RGB {
                        r: 0,
                        g: 0,
                        b: 0,
                    });
                }
            }
        }
    }
}
