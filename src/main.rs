pub mod printer;
pub mod line;
pub mod framebuffer;
use raylib::prelude::*;

fn main() {
    let punto = Vector2::new(100.00,200.00);
    let vertice = Vector3::new(3.0, 5.0, -2.0);
    println!("x = {}", vertice.x);
    println!("y = {}", vertice.y);
    println!("z = {}", vertice.z);
    printer::print_point(punto);
    printer::print_vertex(vertice);
}
