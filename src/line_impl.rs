extern crate nalgebra_glm as glm;
use glm::Vec3;
use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        // Extract coordinates and round them to the nearest integers
        let x1 = start.x.round() as isize;
        let y1 = start.y.round() as isize;
        let x2 = end.x.round() as isize;
        let y2 = end.y.round() as isize;

        // Implement the line drawing (e.g., Bresenham's algorithm)
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x, y);
            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x2 { break; }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 { break; }
                err += dx;
                y += sy;
            }
        }
    }
}
