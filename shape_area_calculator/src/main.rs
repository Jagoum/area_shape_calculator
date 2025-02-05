use std::io;
use accept_input::get_num;
use area_of_shape::Area;
mod area_of_shape;
mod rectangle;
mod circle;
mod accept_input;
fn main() {

    println!("*********** Welcome to my SHAPE AREA CALCULATOR ************\n\n");
    println!("Eneter the shape you whould like to compute the Area (rectangle[1] or circle[2])");
    let mut shape = String::new();
    let mut obj = "";
   io::stdin().read_line(&mut shape).unwrap();
   let shape:u8 =shape.trim().parse().unwrap();

   let area: f64 = { 
    if shape == 2 {
         obj ="Circle";
        let rad = get_num("Please enter the radius of the Circle in cm");
        let ans: area_of_shape::Circle = area_of_shape::Circle {rad};
        ans.area()
    }
    else if shape == 1 {
         obj ="Rectangle";
        let len = get_num("Please enter the length of the rectangle in cm");
        let wid = get_num("Please enter the width of the rectangle in cm");
        let ans = area_of_shape::Rectangle {len,wid};
        ans.area()
    }
    else {
        println!("Please enter one of the above shapes (Rectangle[1] or Circle[2]) ");
        0.0
    }
};
    println!("Area of {}  = {} cm^2",obj,area);
   
}


