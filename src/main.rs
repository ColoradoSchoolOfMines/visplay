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

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate openssl;

mod xsystem;
use xsystem::*;

mod control;

mod config;
use config::Config;

mod auth;

use std::rc::Rc;
use std::fs::File;

use clap::{App, Arg};
use simplelog::{SharedLogger, CombinedLogger, WriteLogger, TermLogger, LogLevelFilter};

fn log_level_to_enum(level: i32) -> LogLevelFilter {
    match level {
        l if l <= -3 => LogLevelFilter::Off,
        -2 => LogLevelFilter::Error,
        -1 => LogLevelFilter::Warn,
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    }
}


fn main() {
    // setup clap command-line stuff
    let args = App::new("visplay-host")
            .arg(Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Makes the log more verbose"))
            .arg(Arg::with_name("quiet")
                .conflicts_with("verbose")
                .short("q")
                .multiple(true)
                .help("Makes the log quieter"))
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Specify a JSON config file"))
        .get_matches();

    // load config
    let config = if let Some(cfgpath) = args.value_of("config") {
        let file = File::open(cfgpath).expect(&format!("Config file \"{}\" not found", cfgpath));
        serde_json::from_reader(file).expect("Could not read json config")
    } else {
        Config::default()
    };

    // get the log level given by the command line arguments
    // this level will be for terminal output
    let term_log_level = match (args.occurrences_of("verbose"), args.occurrences_of("quiet")) {
        (0, 0) => 0,
        (v, 0) => v as i32,
        (0, q) => 0 - q as i32,
        _ => 0, // Illegal state
    };

    // create list of loggers
    let mut loggers: Vec<Box<SharedLogger>> = Vec::new();
    // log to terminal (using command line arg level)
    loggers.push(TermLogger::new(
        log_level_to_enum(term_log_level), 
        simplelog::Config::default()
    ).unwrap());
    // log to file if in config (using the log level in the config)
    if let &Some(ref file) = &config.log_file {
        loggers.push(WriteLogger::new(
            log_level_to_enum(config.log_level),
            simplelog::Config::default(),
            File::create(file).unwrap()));
    }

    // init logger
    let _ = CombinedLogger::init(loggers);

    // load xlib dll
    let xlib = Rc::new(Xlib::open().expect("Could not load Xlib"));

    // start window manager
    //let mut structure = RootStructure::setup(xlib).expect("Could not initialize xsystem");

    // load auth
    let auth = auth::Auth::from_cfg(&config);
    info!("Loaded {} rsa key(s)", auth.keys.len());

    // event loop
    loop {
        //structure.poll_event(); // TODO: handle events
    }
    //structure.teardown();
}