extern crate image;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel};
use image::png::PNGEncoder;

pub fn write_bytes(buf: &[u8], out_file: &str) -> std::io::Result<()> {
    let mut file = try!(File::create(&Path::new(out_file)));
    try!(file.write_all(buf));

    Ok(())
}

pub fn read_bytes(in_file: &str) -> std::io::Result<(Vec<u8>)> {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = try!(File::open(&Path::new(in_file)));
    try!(file.read_to_end(&mut buf));

    Ok(buf)
}

pub fn encode_file(in_buf: &[u8]) -> Result<Vec<u8>, image::ImageError> {
    let mut buf: Vec<u8> = in_buf.to_vec();

    let len = buf.len() as u32;
    let img_size = ((len as f64) / 4.0 + 1.0).sqrt().ceil() as u32;

    buf.insert(0, ((len >> 24) & 0xFF) as u8);
    buf.insert(1, ((len >> 16) & 0xFF) as u8);
    buf.insert(2, ((len >>  8) & 0xFF) as u8);
    buf.insert(3, ((len >>  0) & 0xFF) as u8);

    while (buf.len() as u32) < img_size.pow(2) * 4 {
        buf.push(0 as u8);
    }

    let mut out_buf: Vec<u8> = Vec::new();

    {
        let mut enc = PNGEncoder::new(&mut out_buf);
        try!(enc.encode(&buf, img_size, img_size, image::RGBA(8)));
    }

    Ok(out_buf)
}

pub fn decode_file(in_buf: &[u8]) -> Result<Vec<u8>, image::ImageError> {
    let img = try!(image::load_from_memory_with_format(in_buf, image::ImageFormat::PNG));

    let mut buf: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.pixels() {
        for i in pixel.channels() {
            buf.push(*i);
        }
    }

    let size: usize = (((buf[0] as u32) << 24)
                     | ((buf[1] as u32) << 16)
                     | ((buf[2] as u32) <<  8)
                     | ((buf[3] as u32) <<  0)) as usize;

    let ret: Vec<u8> = (&buf[4..(size + 4)]).to_vec();

    Ok(ret)
}