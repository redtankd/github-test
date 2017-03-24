#[macro_use] extern crate log;
extern crate log4rs;

use log::LogLevel;

fn main() {
    //TODO log4rs.yml is required in working dir.
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    trace!("this is a trace {}", "message");
    debug!("this is a debug {}", "message");
    info!("this is a info {}", "message");
    warn!("this is a warn {}", "message");
    error!("this is printed by default");

    target::log();

    if log_enabled!(LogLevel::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }

    info!("the log level was: {:?}", log::max_log_level());
}

mod target {

    pub fn log() {
        error!("this is the default target");
        error!(target: "wanted", "this is the wanted target");
    }
}