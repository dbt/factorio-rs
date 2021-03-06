extern crate clap;
extern crate facto;

use facto::qr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Facto {
    Qr(qr::Opts),
}

fn main() {
    let opt = Facto::from_args();
    match opt {
        Facto::Qr(opts) => qr::run(&opts),
    }
}
