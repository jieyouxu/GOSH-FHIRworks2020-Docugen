mod config;
mod parser;
mod document;

use config::DocugenConfig;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: DocugenConfig = config::read_config_from_path("config.toml")
        .expect("Configuration file non-existent or invalid; check log output for details");
}

fn main() {
    println!("Config given: {:?}", &*CONFIG);
}
