extern crate image_hider;
extern crate image;

fn main() {
    println!("Encoding file");
    let res = image_hider::encode_file(&image_hider::read_bytes("contact.mp3").unwrap()).unwrap();
    image_hider::write_bytes(&res, "output2.png").unwrap();

    println!("Decoding file");
    let res = image_hider::decode_file(&image_hider::read_bytes("output2.png").unwrap()).unwrap();
    image_hider::write_bytes(&res, "contact2.mp3").unwrap();
}