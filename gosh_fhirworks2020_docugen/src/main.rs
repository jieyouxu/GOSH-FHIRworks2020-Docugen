pub mod config;
pub mod data;

use config::DocugenConfig;
use lazy_static::lazy_static;

/// Default path to search for the configuration file. Defaults to `config.toml`
/// under the project root or the binary root.
const DEFAULT_PATH: &str = "config.toml";

lazy_static! {
    static ref CONFIG: DocugenConfig = config::read_config_from_path(DEFAULT_PATH)
        .expect("Configuration file non-existent or invalid; check log output for details");
}

fn main() {
    println!("Config given: {:?}", &*CONFIG);
}
