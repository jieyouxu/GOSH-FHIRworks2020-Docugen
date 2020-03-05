pub mod cli;
pub mod config;
pub mod core;
pub mod data;

use crate::core::document::DocumentTemplate;
use crate::core::parser;
use config::DocugenConfig;
use log::{error, info};
use pretty_env_logger;
use std::fs;
use std::path;

/// Default path to search for the configuration file. Defaults to `config.toml`
/// under the project root or the binary root.
const DEFAULT_CONFIG_PATH: &str = "config.toml";
const DEFAULT_TEMPLATE_PATH: &str = "document.template";

fn main() {
    pretty_env_logger::init();

    let matches = cli::cli().get_matches();

    let config_path = matches.value_of("config").unwrap_or(DEFAULT_CONFIG_PATH);
    info!("Trying to read config from {}", &config_path);
    let config = read_config_from_path(&config_path);

    match &config {
        Ok(cfg) => info!("config given: {:?}", cfg),
        Err(e) => error!("failed to read config: {}", e),
    };

    let _endpoint = matches
        .value_of("ENDPOINT")
        .expect("<ENDPOINT> is required");
    // TODO: implement `web` module.

    let template_path = matches
        .value_of("TEMPLATE")
        .unwrap_or(DEFAULT_TEMPLATE_PATH);
    let _template = read_template_from_path(&template_path);

    unimplemented!();
}

/// Attempt to read configuration from a file of the given `path`.
pub fn read_config_from_path(path: &str) -> Result<DocugenConfig, String> {
    info!("Trying to read configuration from path: \"{}\"", path);
    let path = path::Path::new(path);

    // We require that the configuration file exists at the provided `path`.
    // This will trigger a panic if the configuration file does not exist as it
    // is likely a programmer mistake.
    assert!(
        path.exists(),
        "Configuration file at path \"{:?}\" does not exist!",
        &path
    );

    let raw_config = read_from_file(&path).map_err(|e| e.to_string())?;
    let config = parse_as_toml(&raw_config)?;

    info!("Config successfully parsed as TOML");
    info!("{:#?}", &config);

    Ok(config)
}

fn read_from_file<'a>(path: &'a path::Path) -> std::io::Result<String> {
    let content = fs::read_to_string(path)?;

    info!("File read:");
    info!("{}", &content);

    Ok(content)
}

fn parse_as_toml(raw: &str) -> Result<DocugenConfig, String> {
    toml::from_str::<DocugenConfig>(raw).map_err(|e| {
        error!("Failed to parse config as TOML. Check your configuration!");
        error!("Provided raw config:");
        error!("\n{}", raw);
        error!("Error cause: {:#?}", &e);
        e.to_string()
    })
}

pub fn read_template_from_path(path: &str) -> Result<DocumentTemplate, String> {
    info!("Trying to read template from path: \"{}\"", path);

    let path = path::Path::new(path);

    assert!(
        path.exists(),
        "A template file does not exist or is unreadable at the provided path"
    );

    let raw_template = read_from_file(&path).map_err(|e| e.to_string())?;
    let template = parser::document_template()
        .parse(raw_template.as_bytes())
        .map_err(|e| e.to_string())?;

    info!("DocumentTemplate successfully parsed");
    info!("{:#?}", &template);

    Ok(template)
}
