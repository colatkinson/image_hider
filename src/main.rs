extern crate image_hider;
extern crate getopts;

use std::path::Path;
use getopts::Options;
use std::env;

fn main() {
    /*println!("Encoding file");
    let res = image_hider::encode_file(&image_hider::read_bytes("contact.mp3").unwrap()).unwrap();
    image_hider::write_bytes(&res, "output2.png").unwrap();

    println!("Decoding file");
    let res = image_hider::decode_file(&image_hider::read_bytes("output2.png").unwrap()).unwrap();
    image_hider::write_bytes(&res, "contact2.mp3").unwrap();*/

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    println!("{:?}", args);

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("", "enc", "Encode the given file");
    opts.optflag("", "dec", "Decode the given file");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let enc_mode = if matches.opt_present("enc") {
        true
    } else if matches.opt_present("dec") {
        false
    } else {
        println!("Please specify either --enc or --dec mode");
        return;
    };

    let in_file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("You need to give a file name, dingus");
        return;
    };

    let out_file = if matches.opt_present("o") {
        matches.opt_str("o").unwrap().to_string()
    } else {
        let path = Path::new(&in_file);
        let filen = path.file_name().unwrap();

        if enc_mode {
            let file_str = filen.to_str().unwrap().to_string();
            file_str + ".png"
        } else {
            let fake_path = Path::new(filen.to_str().unwrap());
            let stem = fake_path.file_stem().unwrap();
            stem.to_str().unwrap().to_string()
        }
    };

    println!("{}", out_file);

    if enc_mode {
        let res = image_hider::encode_bytes(&image_hider::read_file_bytes(&in_file).unwrap()).unwrap();
        image_hider::write_file_bytes(&res, &out_file).unwrap();
    } else {
        let res = image_hider::decode_bytes(&image_hider::read_file_bytes(&in_file).unwrap()).unwrap();
        image_hider::write_file_bytes(&res, &out_file).unwrap();
    }
}