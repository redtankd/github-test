use log::{debug, error, info, log_enabled, trace, warn, Level};

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
        warn!("the level info is enabled! the answer was: {}", x);
    }

    info!("the log level was: {:?}", log::max_level());
}

mod target {
    use log::error;

    pub fn log() {
        error!("the default target is the module's path");
        error!(target: "wanted", "this is the wanted target");
    }
}
