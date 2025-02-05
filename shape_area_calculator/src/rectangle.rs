use crate::area_of_shape;
pub struct Rectangle {
    pub len: f64,
    pub wid: f64
}

 impl Area for Rectangle  { 

     fn area(&self)  -> f64 {
        self.len*self.wid
    } 
 }