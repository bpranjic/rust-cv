use crate::util::bitmap::{Bitmap, read_bitmap_file_header, read_bitmap_info_header};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Debug, Clone)]
pub enum ImageFormat {
    BMP(Bitmap),
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub format: ImageFormat,
}

impl Image {
    pub fn save_image<W: Write + Seek>(&self, writer: W) -> io::Result<()> {
        match &self.format {
            ImageFormat::BMP(_) => self.save_as_bmp(writer),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid format",
            )),
        }
    }

    pub fn save_as_bmp<W: Write + Seek>(&self, mut writer: W) -> io::Result<()> {
        let file_header_size = 14u32;
        let info_header_size = 40u32;
        let row_size = ((self.width * 3 + 3) / 4) * 4;
        let pixel_data_size = row_size * self.height;
        let file_size = file_header_size + info_header_size + pixel_data_size;

        writer.write_u8(b'B')?;
        writer.write_u8(b'M')?;
        writer.write_u32::<LittleEndian>(file_size)?;
        writer.write_u16::<LittleEndian>(0)?;
        writer.write_u16::<LittleEndian>(0)?;
        writer.write_u32::<LittleEndian>(file_header_size + info_header_size)?;

        writer.write_u32::<LittleEndian>(info_header_size)?;
        writer.write_i32::<LittleEndian>(self.width as i32)?;
        writer.write_i32::<LittleEndian>(self.height as i32)?;
        writer.write_u16::<LittleEndian>(1)?;
        writer.write_u16::<LittleEndian>(
            8 * (self.pixels.len() as u32 / (self.width * self.height)) as u16,
        )?;
        writer.write_u32::<LittleEndian>(0)?;
        writer.write_u32::<LittleEndian>(0)?;
        writer.write_i32::<LittleEndian>(0)?;
        writer.write_i32::<LittleEndian>(0)?;
        writer.write_u32::<LittleEndian>(0)?;
        writer.write_u32::<LittleEndian>(0)?;

        let row_size_usize = row_size as usize;
        for y in 0..self.height {
            let start = (y as usize) * row_size_usize;
            let end = start + row_size_usize;
            writer.write_all(&self.pixels[start..end])?;
        }

        Ok(())
    }

    pub fn to_grayscale(&self) -> Image {
        let size = (self.width * self.height) as usize;
        let mut grayscale_pixels = vec![0u8; size * 3];

        for i in 0..size {
            let b = self.pixels[3 * i + 0] as f32;
            let g = self.pixels[3 * i + 1] as f32;
            let r = self.pixels[3 * i + 2] as f32;

            let gray = (0.114 * b + 0.587 * g + 0.299 * r).round() as u8;

            grayscale_pixels[3 * i + 0] = gray;
            grayscale_pixels[3 * i + 1] = gray;
            grayscale_pixels[3 * i + 2] = gray;
        }

        Image {
            width: self.width,
            height: self.height,
            pixels: grayscale_pixels,
            format: self.format.clone(),
        }
    }
}

pub fn load_image<R: Read + Seek>(mut reader: R) -> io::Result<Image> {
    let mut signature = [0u8; 2];
    reader.read_exact(&mut signature)?;
    reader.seek(SeekFrom::Start(0))?;

    match &signature {
        b"BM" => load_bmp(reader),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unknown image format",
        )),
    }
}

pub fn load_bmp<R: Read + Seek>(mut reader: R) -> io::Result<Image> {
    let file_header = read_bitmap_file_header(&mut reader)?;
    if file_header.header_field_a != b'B' || file_header.header_field_b != b'M' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a BMP file.",
        ));
    }

    let info_header = read_bitmap_info_header(&mut reader)?;
    reader.seek(SeekFrom::Start(file_header.image_offset as u64))?;

    let pixel_data_size = info_header.image_size as usize;
    let mut pixels = vec![0u8; pixel_data_size];
    reader.read_exact(&mut pixels)?;

    Ok(Image {
        width: info_header.bitmap_width as u32,
        height: info_header.bitmap_height as u32,
        pixels: pixels,
        format: ImageFormat::BMP(Bitmap {
            file_header: file_header,
            info_header: info_header,
        }),
    })
}
