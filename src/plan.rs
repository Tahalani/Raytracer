//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-taha.alani
// File description:
// plan
//

use crate::point::Point3D;
use crate::ray::Ray;
use crate::vector::Vector;
use core::ops::Sub;

#[derive(Debug)]

pub struct Plan {
    pub center: Point3D,
    pub radius: f64,
}

impl Plan {
    pub fn init_Plan(center: Point3D, radius: f64) -> Plan {
        Plan { center, radius }
    }
    pub fn hits(&self, ray: Ray) -> bool {
        
    }
}
