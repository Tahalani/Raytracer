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
use crate::plan;

pub struct Screen {
    rectangle: Rectangle3D,
}

impl Screen {
    pub fn init_screen(rectangle: Rectangle3D) -> Screen {
        Screen { rectangle }
    }

    pub fn display_screen(&self, _camera: camera::Camera, mut sphere: sphere::Sphere, plan: plan::Plan)
    {
        let width = 1000;
        let height = 1000;
        let mut file = create_file(width + 1, height + 1);

        for y in (0..=height).rev() {
            for x in 0..=width {
                let _ray = _camera.ray(x as f64 / width as f64, y as f64 / height as f64);
                let _hits = sphere.hits(_ray);
                let _hits_plan = plan.hits(_ray);
                if _hits {
                    write_pixel(&mut file, &RGB {
                        r: 255,
                        g: 0,
                        b: 255,
                    });
                } else if _hits_plan {
                    write_pixel(&mut file, &RGB {
                        r: 0,
                        g: 255,
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
