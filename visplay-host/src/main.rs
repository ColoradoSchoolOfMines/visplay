extern crate clap;
extern crate protobuf;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate zombie;
extern crate x11_dl as x11;
extern crate libc;
#[macro_use]
extern crate lazy_static;

mod xsystem;
use xsystem::*;

use std::rc::Rc;
use std::fs::File;

use clap::{App, Arg};
use simplelog::{SharedLogger, CombinedLogger, WriteLogger, TermLogger, LogLevelFilter, Config};


fn main() {
    let args = App::new("visplay-host")
            .arg(Arg::with_name("logfile")
                .long("logfile")
                .short("l")
                .takes_value(true)
                .help("Save the log to a file"))
            .arg(Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Makes the log more verbose"))
            .arg(Arg::with_name("quiet")
                .conflicts_with("verbose")
                .short("q")
                .multiple(true)
                .help("Makes the log quieter"))
        .get_matches();

    let loglevel = match args.occurrences_of("verbose") as i32 - args.occurrences_of("quiet") as i32 {
        x if x <= -3 => LogLevelFilter::Off,
        -2 => LogLevelFilter::Error,
        -1 => LogLevelFilter::Warn,
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        x if x >= 2 => LogLevelFilter::Trace,
        _ => LogLevelFilter::Info,
    };

    let mut loggers: Vec<Box<SharedLogger>> = Vec::new();
    loggers.push(TermLogger::new(loglevel, Config::default()).unwrap());
    if let Some(file) = args.value_of("logfile") {
        loggers.push(WriteLogger::new(loglevel, Config::default(), File::create(file).unwrap()));
    }

    let _ = CombinedLogger::init(loggers);

    let xlib = Rc::new(Xlib::open().expect("Could not load Xlib"));

    let mut structure = RootStructure::setup(xlib).expect("Could not initilize xsystem");
    loop {
        structure.poll_event();
    }
    structure.teardown();
}
