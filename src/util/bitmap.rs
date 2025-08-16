use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct Bitmap {
    pub file_header: BitmapFileHeader,
    pub info_header: BitmapInfoHeader,
}

#[derive(Debug, Clone)]
pub struct BitmapFileHeader {
    pub header_field_a: u8,
    pub header_field_b: u8,
    pub file_size: u32,
    pub reserved_a: u16,
    pub reserved_b: u16,
    pub image_offset: u32,
}

#[derive(Debug, Clone)]
pub struct BitmapInfoHeader {
    pub header_size: u32,
    pub bitmap_width: i32,
    pub bitmap_height: i32,
    pub color_planes: u16,
    pub bit_count: u16,
    pub compression_method: u32,
    pub image_size: u32,
    pub horizontal_resolution: i32,
    pub vertical_resolution: i32,
    pub color_palette: u32,
    pub important_colors: u32,
}

pub fn read_bitmap_file_header<R: Read>(mut reader: R) -> io::Result<BitmapFileHeader> {
    Ok(BitmapFileHeader {
        header_field_a: reader.read_u8()?,
        header_field_b: reader.read_u8()?,
        file_size: reader.read_u32::<LittleEndian>()?,
        reserved_a: reader.read_u16::<LittleEndian>()?,
        reserved_b: reader.read_u16::<LittleEndian>()?,
        image_offset: reader.read_u32::<LittleEndian>()?,
    })
}

pub fn read_bitmap_info_header<R: Read>(mut reader: R) -> io::Result<BitmapInfoHeader> {
    Ok(BitmapInfoHeader {
        header_size: reader.read_u32::<LittleEndian>()?,
        bitmap_width: reader.read_i32::<LittleEndian>()?,
        bitmap_height: reader.read_i32::<LittleEndian>()?,
        color_planes: reader.read_u16::<LittleEndian>()?,
        bit_count: reader.read_u16::<LittleEndian>()?,
        compression_method: reader.read_u32::<LittleEndian>()?,
        image_size: reader.read_u32::<LittleEndian>()?,
        horizontal_resolution: reader.read_i32::<LittleEndian>()?,
        vertical_resolution: reader.read_i32::<LittleEndian>()?,
        color_palette: reader.read_u32::<LittleEndian>()?,
        important_colors: reader.read_u32::<LittleEndian>()?,
    })
}
