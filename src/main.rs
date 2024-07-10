extern crate nalgebra_glm as glm;
use glm::Vec3;
mod framebuffer;
mod line_impl;

use framebuffer::Framebuffer;
use line_impl::Line;

fn main() {
    let mut fb = Framebuffer::new(800, 600);

    let vertices = vec![
        glm::vec3(100.0, 100.0, 0.0),
        glm::vec3(400.0, 100.0, 0.0),
        glm::vec3(250.0, 300.0, 0.0),
    ];

    draw_polygon(&mut fb, &vertices);

    fb.save_as_bmp("output.bmp").expect("Failed to save BMP file");
    println!("Polygon drawn and saved to output.bmp");
}

fn draw_polygon(fb: &mut Framebuffer, vertices: &[Vec3]) {
    if vertices.len() < 3 {
        eprintln!("Polygon drawing failed: A polygon requires at least three vertices.");
        return;
    }

    // Initialize a buffer to store the vertices
    let mut vertex_buffer: Vec<Vec3> = vertices.to_vec(); // This copies vertex data into a local buffer

    // Draw lines between consecutive vertices
    for i in 0..vertex_buffer.len() {
        let next_index = (i + 1) % vertex_buffer.len();  // Wrap around to close the polygon
        fb.line(vertex_buffer[i], vertex_buffer[next_index]);
    }

    // Filling the polygon using scan-line fill
    fill_polygon(fb, &vertex_buffer);
}

fn fill_polygon(fb: &mut Framebuffer, vertices: &[Vec3]) {
    // This is a simplified scan-line fill algorithm
    let y_min = vertices.iter().map(|v| v.y as isize).min().unwrap();
    let y_max = vertices.iter().map(|v| v.y as isize).max().unwrap();

    for y in y_min..=y_max {
        let mut intersections: Vec<isize> = Vec::new();
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];
            if (v1.y as isize <= y && v2.y as isize > y) || (v2.y as isize <= y && v1.y as isize > y) {
                let x = v1.x + (v2.x - v1.x) * ((y as f32 - v1.y) / (v2.y - v1.y));
                intersections.push(x as isize);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x1 = intersections[i];
                let x2 = intersections[i + 1];
                for x in x1..=x2 {
                    fb.point(x, y);
                }
            }
        }
    }
}
