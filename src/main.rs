extern crate clap;
extern crate facto;

use facto::qr;
use facto::trains;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Facto {
    Qr(qr::Opts),
    Train(trains::Opts),
}

fn main() {
    let opt = Facto::from_args();
    match opt {
        Facto::Qr(opts) => qr::run(&opts),
        Facto::Train(opts) => trains::run(&opts),
    }
}
