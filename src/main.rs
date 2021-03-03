extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate facto;
use facto::bp::builder;
use facto::qr::*;
use std::io;

fn main() {
    let matches = App::new("facto")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("qr").arg(
                Arg::with_name("URL")
                    .required(true)
                    .help("URL to be converted to QR code"),
            ),
        )
        .get_matches();
    if let Some(qr) = matches.subcommand_matches("qr") {
        let url = qr.value_of("URL").unwrap();
        let mut b = builder::Builder::new(
            url,
            format!("QR code made from [item=stone-wall] that links to {}", url),
        );
        b.add_qr_code(0.5, 0.5, url);
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.add_icon("stone-wall");
        b.render(&mut io::stdout());
    }
}
