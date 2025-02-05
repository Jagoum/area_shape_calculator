
use crate::area_of_shape;

pub struct Circle {
    pub rad: f64
 }
 impl Area for Circle  {
    fn area(&self) -> f64 {
        std::f64::consts::PI*self.rad
    }
 }