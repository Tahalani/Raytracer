//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// screen
//

use crate::rectangle::Rectangle3D;
use crate::camera;
use crate::sphere;
use crate::rgb::RGB;
use crate::write_ppm::{write_pixel, create_file};
use crate::plan;

pub struct Screen {
    rectangle: Rectangle3D,
}

impl Screen {
    pub fn init_screen(rectangle: Rectangle3D) -> Screen {
        Screen { rectangle }
    }

    pub fn display_screen(&self, _camera: camera::Camera, mut sphere: sphere::Sphere, mut plan: plan::Plan)
    {
        let width = 1000;
        let height = 1000;
        let mut file = create_file(width + 1, height + 1);

        for y in (0..=height).rev() {
            for x in 0..=width {
                let _ray = _camera.ray(x as f64 / width as f64, y as f64 / height as f64);
                let _hits: bool = sphere.hits(_ray);
                let _hits_plan = plan.hits(_ray);

                if _hits {
                    write_pixel(&mut file, &RGB {
                        r: sphere.rgb.r,
                        g: sphere.rgb.g,
                        b: sphere.rgb.b,
                });
                } else if _hits_plan && plan.distance > 0.0 {
                    write_pixel(&mut file, &RGB {
                        r: plan.rgb.r,
                        g: plan.rgb.g,
                        b: plan.rgb.b,
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
