use std::io::prelude::*;
use std::fs::File;

extern crate image_hider;
extern crate image;

fn main() {
    println!("Encoding file");
    image_hider::encode_file("/home/colin/Videos/TV/Freaks and Geeks/Season 1/FreaksAndGeeks - s01e01.mkv", "output.png");

    println!("Decoding file");
    image_hider::decode_file("output.png", "freaks.mkv");
}