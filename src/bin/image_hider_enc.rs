extern crate image_hider;
extern crate image;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filen = args[1].clone();
        let res = image_hider::encode_file(&image_hider::read_bytes(&filen).unwrap()).unwrap();
        image_hider::write_bytes(&res, &(filen + ".png")).unwrap();
    } else {
        panic!("Please supply a file name");
    }
}