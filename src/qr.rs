extern crate qrcode_generator;

use crate::bp::builder::Builder;

use qrcode_generator::*;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    url: String,
}

pub fn run(opts: &Opts) {
    let url = &opts.url;
    let mut b = Builder::new(
        url,
        format!("QR code made from [item=stone-wall] that links to {}", url),
    );
    b.add_qr_code(0.0, 0.0, url);
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.render(&mut io::stdout()).expect("render");
}

pub trait QrGenerator {
    fn add_qr_code<S: AsRef<str>>(&mut self, x: f32, y: f32, val: S);
}

impl QrGenerator for Builder {
    fn add_qr_code<S: AsRef<str>>(&mut self, x: f32, y: f32, val: S) {
        let val = qrcode_generator::to_matrix_from_str(val, QrCodeEcc::High).unwrap();
        for (ix, v) in val.iter().enumerate() {
            for (iy, b) in v.iter().enumerate() {
                if *b {
                    self.add("stone-wall", x + ix as f32, y + iy as f32);
                }
            }
        }
    }
}
