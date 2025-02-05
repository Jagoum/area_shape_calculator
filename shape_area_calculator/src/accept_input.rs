use std::io;
pub fn get_num(msg:&str) -> f64{
    println!("{}",msg);
    let mut  num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: f64 = match num.trim().parse() {
        Ok(num) => {num},
        Err(_) => {panic!("Expected a number ")},
    };
    num
}