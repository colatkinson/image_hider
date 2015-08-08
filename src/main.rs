extern crate image_hider;
extern crate getopts;

use std::path::Path;
use getopts::Options;
use std::env;
use std::io::prelude::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} (--enc|--dec) [options] <file>", program);
    print!("{}", opts.usage(&brief));
    println!("");
    println!("If a value of - is given for <file>, then the program will accept data from stdin");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "Set output file name [default: <file>.png]", "<name>");
    opts.optflag("", "enc", "Encode the given file");
    opts.optflag("", "dec", "Decode the given file");
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let enc_mode = if matches.opt_present("enc") {
        true
    } else if matches.opt_present("dec") {
        false
    } else {
        println!("Please specify either --enc or --dec mode");
        return;
    };

    let in_file = if !matches.free.is_empty() {
        let z = matches.free[0].clone();
        if z == "-" {
            "stdin".to_string()
        } else {
            z
        }
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

    let mut bytes: Vec<u8> = Vec::new();

    if in_file == "stdin" {
        std::io::stdin().read_to_end(&mut bytes).unwrap();
    } else {
        let res = image_hider::read_file_bytes(&in_file);
        match res {
            Ok(v) => {
                bytes = v;
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                return;
            }
        }
    }

    if enc_mode {
        let res = image_hider::encode_bytes(&bytes).ok()
                                                   .expect("Error encoding image");
        image_hider::write_file_bytes(&res, &out_file).ok()
                                                      .expect("Error writing image to file");
    } else {
        let res = image_hider::decode_bytes(&bytes).ok()
                                                   .expect("Error decoding image");
        image_hider::write_file_bytes(&res, &out_file).ok()
                                                      .expect("Error writing file");
    }
}
