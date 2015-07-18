//! This module contains methods that allow the encoding of a byte stream to and from a PNG
//!
//! # Examples
//!
//! ```
//! use image_hider;
//! let enc = image_hider::encode_bytes("abcde".as_bytes()).unwrap();
//! let _ = image_hider::write_file_bytes(&enc, "test.png");
//!
//! let read = image_hider::read_file_bytes("test.png").unwrap();
//! let dec = image_hider::decode_bytes(&read).unwrap();
//! assert_eq!(dec, "abcde".to_string().into_bytes());
//! std::fs::remove_file("test.png");
//! ```

extern crate image;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel};
use image::png::PNGEncoder;

/// Writes the given byte buffer to a file
///
/// # Examples
///
/// ```no_run
/// use image_hider::write_file_bytes;
///
/// let _ = write_file_bytes("abcde".as_bytes(), "foo.txt");
/// ```
pub fn write_file_bytes(buf: &[u8], out_file: &str) -> std::io::Result<()> {
    let mut file = try!(File::create(&Path::new(out_file)));
    try!(file.write_all(buf));

    Ok(())
}

/// Reads the given file into a byte vector
///
/// # Examples
///
/// ```no_run
/// use image_hider::read_file_bytes;
///
/// let bytes = read_file_bytes("foo.txt").unwrap();
/// ```
pub fn read_file_bytes(in_file: &str) -> std::io::Result<(Vec<u8>)> {
    let mut buf: Vec<u8> = Vec::new();

    let mut file = try!(File::open(&Path::new(in_file)));
    try!(file.read_to_end(&mut buf));

    Ok(buf)
}

/// Encodes a byte array into a PNG image and returns a byte vector of its data
///
/// The image is RGBA, and so each pixel contains 4 bytes of data.
///
/// # Examples
///
/// ```no_run
/// use image_hider::encode_bytes;
///
/// let bytes = encode_bytes("abcde".as_bytes()).unwrap();
/// ```
pub fn encode_bytes(in_buf: &[u8]) -> Result<Vec<u8>, image::ImageError> {
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

/// Decodes file data from a PNG image and returns it as a byte vector
///
/// The image is RGBA, and so each pixel contains 4 bytes of data.
///
/// # Examples
///
/// ```no_run
/// use image_hider::{decode_bytes, read_file_bytes};
///
/// let png = read_file_bytes("foo.png").unwrap();
/// let bytes = decode_bytes(&png).unwrap();
/// ```
pub fn decode_bytes(in_buf: &[u8]) -> Result<Vec<u8>, image::ImageError> {
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