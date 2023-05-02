//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// plan
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Debug)]

pub struct Plan {
    pub normal : Vector,
    pub origin : Point3D,
}

impl Plan {
    pub fn init_Plan(normal : Vector, origin : Point3D) -> Plan {
        Plan {normal, origin}
    }
    pub fn hits(&self, ray: Ray) -> bool {
        let res = ray.direction.dot_product(self.normal);

        if res == 0.0 {
            return false;
        } else {
            let d = (self.origin - ray.origin).dot_product(self.normal) / res;
            // println!("d = {}", d);
            if d <= 0.0 {
                return false;
            }
            return true;
        }
    }
}

// point d'intersection {
//      p = l0 + l * d
// }
