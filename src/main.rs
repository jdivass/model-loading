pub mod printer;
pub mod line;
pub mod framebuffer;
pub mod triangle;
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    let a = Vector2::new(320.0, 100.0);
    let b = Vector2::new(150.0, 350.0);
    let c = Vector2::new(500.0, 350.0);
    triangle::draw_triangle(&mut framebuffer, a, b, c);
    framebuffer.color_buffer.export_image("triangle.png");
}
