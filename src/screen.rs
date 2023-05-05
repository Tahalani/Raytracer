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

                sphere.coefficients = sphere.coefficients * 100.0;
                plan.coefficients = plan.coefficients * 100.0;

                plan.distance = plan.distance * 100.0;
                // println!("plan.distance: {}", (plan.distance as f64 / 400.0));


                let mut r = (255.0 / (plan.distance as f64 / 250.0)) as u32;
                let mut g = (255.0 / (plan.distance as f64 / 250.0)) as u32;
                let mut b = (255.0 / (plan.distance as f64 / 250.0)) as u32;
                
                if r > 255 {
                    r = 0;
                }
                if g > 255{
                    g = 0;
                }
                if b > 255{
                    b = 0;
                }
                if r == 0 {
                    r = 255;
                }
                if g == 0{
                    g = 255;
                }
                if b == 0{
                    b = 255;
                }

                if _hits {
                    write_pixel(&mut file, &RGB {
                        r: (sphere.coefficients as u32 * 255) / 100,
                        g: (sphere.coefficients as u32 * 0) / 100,
                        b: (sphere.coefficients as u32 * 255) / 100,
                });
                } else if _hits_plan && plan.distance > 0.0 {
                    write_pixel(&mut file, &RGB {
                        r,
                        g,
                        b,
                    });
                } else {
                    write_pixel(&mut file, &RGB {
                        r: (plan.coefficients as u32 * 0) / 100,
                        g: (plan.coefficients as u32 * 0) / 100,
                        b: (plan.coefficients as u32 * 0) / 100,
                    });
                }
            }
        }
    }
}
