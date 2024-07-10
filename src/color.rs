#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor que toma valores RGB, ahora público
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Constructor que toma un valor hexadecimal, ahora público
    pub fn from_hex(hex: u32) -> Color {
        Color {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    // Método para convertir a valor hexadecimal, ahora público
    pub fn to_hex(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    // Suma de colores con saturación, ahora público
    pub fn saturating_add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    // Multiplicar color por un flotante (intensidad), ahora público
    pub fn multiply_intensity(self, factor: f32) -> Color {
        Color {
            r: (self.r as f32 * factor).min(255.0).max(0.0) as u8,
            g: (self.g as f32 * factor).min(255.0).max(0.0) as u8,
            b: (self.b as f32 * factor).min(255.0).max(0.0) as u8,
        }
    }

    // Restar colores con clamping, ahora público
    pub fn saturating_sub(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Color{{r: {}, g: {}, b: {}}}", self.r, self.g, self.b)
    }
}

// Tests para verificar el comportamiento
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_color_creation() {
        let color = Color::new(255, 150, 0);
        assert_eq!(color.to_hex(), 0xFF9600);
        let color_from_hex = Color::from_hex(0xFF9600);
        assert_eq!(color_from_hex, color);
    }

    #[test]
    pub fn test_color_operations() {
        let color1 = Color::new(200, 100, 50);
        let color2 = Color::new(55, 155, 205);
        let added_color = color1.saturating_add(color2);
        assert_eq!(added_color, Color::new(255, 255, 255)); // Clamped to 255

        let multiplied_color = color1.multiply_intensity(0.5);
        assert_eq!(multiplied_color, Color::new(100, 50, 25));

    }
}
