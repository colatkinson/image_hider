use std::io::prelude::*;
use std::fs::File;

extern crate image_hider;
extern crate image;

fn main() {
    image_hider::encode_file("valedictory.txt", "output.png");
}
