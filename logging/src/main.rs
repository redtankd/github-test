#[macro_use] extern crate log;
extern crate env_logger;

use log::Level;

fn main() {
    env_logger::init();

    trace!("this is a trace {}", "message");
    debug!("this is a debug {}", "message");
    info!("this is a info {}", "message");
    warn!("this is a warn {}", "message");
    error!("this is printed by default");

    target::log();

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }

    info!("the log level was: {:?}", log::max_level());
}

mod target {

	pub fn log() {
		error!("this is the default target");
		error!(target: "wanted", "this is the wanted target");
	}
}
