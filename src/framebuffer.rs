use crate::line_impl::Line; 
extern crate nalgebra_glm as glm;
use glm::Vec3;
extern crate image;
use image::{ImageBuffer, RgbaImage, Rgba};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    pub current_color: u32,
}

impl Framebuffer {
    pub fn draw_polygon(&mut self, points: &[(f64, f64)]) {
        if points.len() < 3 {
            return; // A polygon needs at least 3 points
        }
        
        for i in 0..points.len() - 1 {
            // Ensure conversion to f32 for compatibility with nalgebra-glm expectations
            let start = glm::vec3(points[i].0 as f32, points[i].1 as f32, 0.0f32); 
            let end = glm::vec3(points[i + 1].0 as f32, points[i + 1].1 as f32, 0.0f32); 
            self.line(start, end);
        }
        
        // Draw a line from the last point back to the first point using f32
        let last_point = points.len() - 1;
        let start = glm::vec3(points[last_point].0 as f32, points[last_point].1 as f32, 0.0f32);
        let end = glm::vec3(points[0].0 as f32, points[0].1 as f32, 0.0f32);
        self.line(start, end);
    }
}

impl Framebuffer {
    // Function to save the framebuffer content as a BMP file
    pub fn save_as_bmp(&self, file_path: &str) -> Result<(), image::ImageError> {
        let mut img: RgbaImage = ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = (y as usize) * self.width + x as usize;
            if idx < self.buffer.len() {
                let val = self.buffer[idx];
                // Assuming the framebuffer stores pixel values as RGBA
                *pixel = Rgba([
                    ((val >> 24) & 0xFF) as u8, // Red
                    ((val >> 16) & 0xFF) as u8, // Green
                    ((val >> 8) & 0xFF) as u8,  // Blue
                    (val & 0xFF) as u8,         // Alpha
                ]);
            }
        }

        img.save(file_path)
    }
}


impl Framebuffer {
    // Constructor que inicializa el framebuffer con dimensiones y colores por defecto
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height], // Inicializa el buffer con cero
            background_color: 0x00000000, // Color de fondo negro
            current_color: 0xFFFFFFFF, // Color de dibujo blanco
        }
    }

    // Función para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    // Función para dibujar un punto en las coordenadas (x, y) con el color actual
    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            let index = (y as usize) * self.width + x as usize;
            self.buffer[index] = self.current_color;
        }
    }

    // Función para obtener el color de un punto en las coordenadas (x, y)
    pub fn get_color(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            0 // Retorna negro o un valor de error si está fuera de los límites
        }
    }

    // Método para establecer el color de fondo
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    // Método para establecer el color de dibujo actual
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_framebuffer() {
        let fb = Framebuffer::new(10, 20);
        assert_eq!(fb.width, 10);
        assert_eq!(fb.height, 20);
        assert_eq!(fb.buffer.len(), 200); // 10 * 20
    }

    #[test]
    fn test_clear() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(0x12345678);
        fb.clear();
        assert!(fb.buffer.iter().all(|&color| color == 0x12345678));
    }

    #[test]
    fn test_point() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0xABCDEF);
        fb.point(5, 5);
        assert_eq!(fb.get_color(5, 5), 0xABCDEF);
        // Check that out-of-bounds does nothing
        fb.point(10, 10);
        fb.point(-1, -1);
        assert_eq!(fb.get_color(10, 10), 0); // Assuming default color is 0 for out-of-bounds
    }

    #[test]
    fn test_get_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0xFF00FF);
        fb.point(3, 3);
        assert_eq!(fb.get_color(3, 3), 0xFF00FF);
        assert_eq!(fb.get_color(0, 0), 0); // Default color
    }

    #[test]
    fn test_set_background_color() {
        let mut fb = Framebuffer::new(5, 5);
        assert_eq!(fb.background_color, 0); // Default
        fb.set_background_color(0x12345678);
        assert_eq!(fb.background_color, 0x12345678);
    }

    #[test]
    fn test_set_current_color() {
        let mut fb = Framebuffer::new(5, 5);
        assert_eq!(fb.current_color, 0xFFFFFFFF); // Default
        fb.set_current_color(0x87654321);
        assert_eq!(fb.current_color, 0x87654321);
    }
}

