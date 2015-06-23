extern crate image;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel};

pub fn encode_file(in_file: &str, out_file: &str) {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = File::open(&Path::new(in_file)).unwrap();
    let result = file.read_to_end(&mut buf).unwrap() as u32;
    let size = ((result as f64) / 4.0 + 1.0).sqrt().ceil() as u32;

    buf.insert(0, ((result >> 24) & 0xFF) as u8);
    buf.insert(1, ((result >> 16) & 0xFF) as u8);
    buf.insert(2, ((result >>  8) & 0xFF) as u8);
    buf.insert(3, ((result >>  0) & 0xFF) as u8);

    println!("{:?}", (size.pow(2) * 4 - (buf.len() as u32)).to_string().as_bytes());

    println!("{} {}", buf.len(), size.pow(2) * 4);

    while (buf.len() as u32) < size.pow(2) * 4 {
        buf.push(0 as u8);
    }

    image::save_buffer(&Path::new(out_file), &buf, size, size, image::RGBA(8));
}

pub fn decode_file(in_file: &str, out_file: &str) {
    println!("Opening image");
    let img = image::open(&Path::new(in_file)).unwrap();
    println!("Image opened");

    let mut buf: Vec<u8> = Vec::new();

    for (x, y, pixel) in img.pixels() {
        for i in pixel.channels() {
            buf.push(*i);
        }
    }

    let size: usize = (((buf[0] as u32) << 24)
                     | ((buf[1] as u32) << 16)
                     | ((buf[2] as u32) <<  8)
                     | ((buf[3] as u32) <<  0) as usize;

    println!("{}", size);

    let mut file = File::create(&Path::new(out_file)).unwrap();
    file.write_all(&buf[4..(size + 4)]);
}