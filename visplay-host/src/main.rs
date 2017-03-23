extern crate clap;
extern crate protobuf;
extern crate wtftw_core;
extern crate wtftw_xlib;
#[macro_use]
extern crate log;
extern crate zombie;

mod bs;

use clap::{App, Arg};


fn main() {
    let args = App::new("visplay-host")
        .get_matches();

    bs::main();
}
