extern crate image;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel};

pub fn encode_file(in_file: &str, out_file: &str) {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = File::open(&Path::new(in_file)).unwrap();
    let result = file.read_to_end(&mut buf).unwrap();
    let size = ((result as f64) / 4.0).sqrt().ceil() as u32;

    println!("{:?}", size.to_string().as_bytes());

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

    //println!("{:?}", buf);
    let mut file = File::create(&Path::new(out_file)).unwrap();
    file.write_all(&buf);
}