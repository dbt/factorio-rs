extern crate clap;
extern crate facto;

use facto::cli;
use facto::qr;

fn main() {
    let root = cli::cmd_group("facto", vec![qr::cmd()]);
    let m = root.app.get_matches();
    (root.callback)(&m);
}
