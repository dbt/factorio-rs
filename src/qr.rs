extern crate qrcode_generator;

use crate::bp::builder::*;

use qrcode_generator::*;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(required = true, min_values = 1)]
    urls: Vec<String>,
}

pub fn run(opts: &Opts) {
    if opts.urls.len() == 1 {
        let url = &opts.urls.get(0).unwrap();
        let mut b = BlueprintBuilder::new(
            url,
            format!("QR code made from [item=stone-wall] that links to {}", url),
        );
        b.add_qr_code(0.0, 0.0, url);
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.render(&mut io::stdout()).expect("render");
    } else {
        let mut book = BookBuilder::new(
            "QR codes book",
            format!(
                "QR codes for the following URLs: \n\n - {}\n",
                opts.urls.join("\n - ")
            ),
        );
        book.add_icon("stone-wall");
        book.add_icon("stone-wall");
        book.add_icon("stone-wall");
        book.add_icon("stone-wall");
        for url in &opts.urls {
            let mut b = BlueprintBuilder::new(
                url,
                format!("QR code made from [item=stone-wall] that links to {}", url),
            );
            b.add_qr_code(0.0, 0.0, url);
            b.add_icon("stone-wall");
            b.add_icon("stone-wall");
            b.add_icon("stone-wall");
            b.add_icon("stone-wall");
            book.add_blueprint(b);
        }
        // println!("book: {:?}", book);
        book.render(&mut io::stdout()).expect("render");
    }
}

pub trait QrGenerator {
    fn add_qr_code<S: AsRef<str>>(&mut self, x: f32, y: f32, val: S);
}

impl QrGenerator for BlueprintBuilder {
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
