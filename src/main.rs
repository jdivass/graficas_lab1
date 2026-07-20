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

    let polygon2 = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    framebuffer.set_current_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &polygon2);

    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &polygon2);

    framebuffer.render_to_file("out.png");
    framebuffer.render_to_bmp("out.bmp");
}