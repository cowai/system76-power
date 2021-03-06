extern crate log;
extern crate system76_power;

use log::LevelFilter;
use std::{io, process};
use system76_power::graphics::Graphics;
use system76_power::logging;

fn inner() -> io::Result<()> {

    Graphics::new()?;

    Ok(())
}

fn main() {
    if let Err(why) = logging::setup_logging(LevelFilter::Debug) {
        eprintln!("failed to set up logging: {}", why);
        process::exit(1);
    }

    if unsafe { libc::geteuid() } != 0 {
        eprintln!("must be run as root");
        process::exit(1);
    }

    if let Err(err) = inner() {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
