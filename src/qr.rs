extern crate qrcode_generator;

use std::io;

use clap::{Arg, SubCommand};

use qrcode_generator::*;

use crate::bp::builder::Builder;
use crate::cli::*;

pub fn cmd<'a, 'b>() -> CliApp<'a, 'b> {
    return CliApp {
        app: SubCommand::with_name("qr").arg(
            Arg::with_name("URL")
                .required(true)
                .help("URL to be converted to QR code"),
        ),
        callback: Box::new(|m: &ArgMatches| run(m)),
    };
}

fn run(matches: &ArgMatches) {
    let url = matches.value_of("URL").unwrap();
    let mut b = Builder::new(
        url,
        format!("QR code made from [item=stone-wall] that links to {}", url),
    );
    b.add_qr_code(0.5, 0.5, url);
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.add_icon("stone-wall");
    b.render(&mut io::stdout()).expect("render");
}

pub trait QrGenerator {
    fn add_qr_code(&mut self, x: f32, y: f32, val: &str);
}

impl QrGenerator for Builder {
    fn add_qr_code(&mut self, x: f32, y: f32, val: &str) {
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
