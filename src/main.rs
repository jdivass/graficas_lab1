use raylib::prelude::*;

mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::draw_polygon;
use polygon::fill_polygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.clear();

    let polygon3 = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];

    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &polygon3);

    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &polygon3);

    framebuffer.render_to_file("out.png");
    framebuffer.render_to_bmp("out.bmp");
}