use log::{info,trace,warn,error};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("Hello world!");
    info!("Hello info!");
    warn!("Warn!");
    error!("ERROR!");
}
