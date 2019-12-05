extern crate futures;
extern crate log;
extern crate rig;
extern crate tokio;

use log::info;

use rig::config::settings;
use rig::server::server;

fn main() {
    env_logger::init();
    info!("Rig Settings: {:?}", *settings::SETTINGS);
    info!("Rig API Loaded: {:?}", *settings::APIS);

    server::start_server();
}
