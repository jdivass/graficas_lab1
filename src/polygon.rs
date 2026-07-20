use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_polygon(
    framebuffer: &mut Framebuffer,
    points: &[Vector2],
) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];

        line(framebuffer, start, end);
    }
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 3 {
        return;
    }

    let min_y = points
        .iter()
        .map(|p| p.y as i32)
        .min()
        .unwrap();

    let max_y = points
        .iter()
        .map(|p| p.y as i32)
        .max()
        .unwrap();

    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();
        for i in 0..points.len() {

            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];
            let y1 = p1.y as i32;
            let y2 = p2.y as i32;

            if y1 == y2 {
                continue;
            }

            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            if y >= ymin && y < ymax {

                let x = p1.x
                    + (y as f32 - p1.y)
                        * (p2.x - p1.x)
                        / (p2.y - p1.y);

                intersections.push(x.round() as i32);
            }
        }

        intersections.sort();

        let mut i = 0;

        while i + 1 < intersections.len() {
            let x_start = intersections[i];
            let x_end = intersections[i + 1];

            for x in x_start..=x_end {
                if x >= 0 && y >= 0 {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
            i += 2;
        }
    }
}