use std::fs::File;
use std::io::{self, BufWriter, Write};
use crate::framebuffer::Framebuffer;

pub trait BmpRenderable {
    fn render_to_bmp(&self, file_path: &str) -> io::Result<()>;
}

impl BmpRenderable for Framebuffer {
    fn render_to_bmp(&self, file_path: &str) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(file_path)?);
        write_bmp_file(&mut file, &self.buffer, self.width, self.height)
    }
}

fn write_bmp_file(file: &mut BufWriter<File>, buffer: &[u32], width: usize, height: usize) -> io::Result<()> {
    write_bmp_header(file, width, height)?;
    write_pixel_data(file, buffer, width, height)
}

fn write_bmp_header(file: &mut BufWriter<File>, width: usize, height: usize) -> io::Result<()> {
    let file_size = BMP_HEADER_SIZE as u32 + (width as u32 * height as u32 * 3) + (height as u32 * ((width * 3 % 4) as u32));
    let mut header = vec![
        0x42, 0x4D,                         // BM - Windows bitmap
        0, 0, 0, 0,                         // File size
        0, 0,                               // Reserved
        0, 0,                               // Reserved
        BMP_HEADER_SIZE as u8, 0, 0, 0,     // Offset to pixel data
        40, 0, 0, 0,                        // Header size (BITMAPINFOHEADER)
        0, 0, 0, 0,                         // Width
        0, 0, 0, 0,                         // Height
        1, 0,                               // Planes
        24, 0,                              // Bits per pixel
        0, 0, 0, 0,                         // Compression (none)
        0, 0, 0, 0,                         // Image size (can be 0 if no compression)
        0, 0, 0, 0,                         // Horizontal resolution (pixels per meter)
        0, 0, 0, 0,                         // Vertical resolution (pixels per meter)
        0, 0, 0, 0,                         // Colors in color table
        0, 0, 0, 0,                         // Important color count
    ];
    header[2..6].copy_from_slice(&file_size.to_le_bytes());
    header[18..22].copy_from_slice(&(width as u32).to_le_bytes());
    header[22..26].copy_from_slice(&(height as u32).to_le_bytes());
    file.write_all(&header)
}

fn write_pixel_data(file: &mut BufWriter<File>, buffer: &[u32], width: usize, height: usize) -> io::Result<()> {
    let padding = (4 - (width * 3 % 4)) % 4;
    let padding_buffer = vec![0u8; padding];
    for y in (0..height).rev() {
        for x in 0..width {
            let idx = y * width + x;
            let pixel = buffer[idx];
            let pixel_data = [pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8];
            file.write_all(&pixel_data)?;
        }
        if padding > 0 {
            file.write_all(&padding_buffer)?;
        }
    }
    Ok(())
}

const BMP_HEADER_SIZE: usize = 54;
