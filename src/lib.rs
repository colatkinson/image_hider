extern crate image;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn encode_file(in_file: &str, out_file: &str) {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = File::open(&Path::new(in_file)).unwrap();
    let result = file.read_to_end(&mut buf).unwrap();
    let size = ((result as f64) / 4.0).sqrt().ceil() as u32;

    while (buf.len() as u32) < size.pow(2) {
        buf.push(0);
    }

    image::save_buffer(&Path::new(out_file), &buf, size, size, image::RGBA(8));
}