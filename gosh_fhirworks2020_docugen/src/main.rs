pub mod cli;
pub mod config;
pub mod core;
pub mod data;
pub mod web;

use crate::core::document::{DocumentTemplate, TagPair};
use crate::core::parser;
use config::DocugenConfig;
use log::{error, info};
use pretty_env_logger;
use std::fs;
use std::io::{self, Write};
use std::path;
use tokio;

/// Default path to search for the configuration file. Defaults to `config.toml`
/// under the project root or the binary root.
const DEFAULT_CONFIG_PATH: &str = "config.toml";
const DEFAULT_TEMPLATE_PATH: &str = "document.template";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let matches = cli::cli().get_matches();

    let config_path = matches.value_of("config").unwrap_or(DEFAULT_CONFIG_PATH);
    info!("Trying to read config from {}", &config_path);
    let config = match read_config_from_path(&config_path) {
        Ok(cfg) => {
            info!("config given: {:?}", cfg);
            cfg
        }
        Err(e) => {
            error!("failed to read config: {}", e);
            std::process::exit(1)
        }
    };

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("LOG", "info");

    if let Some(verbosity) = matches.value_of("v") {
        if let Ok(verbosity) = verbosity.parse::<u32>() {
            match verbosity {
                0 => std::env::set_var("LOG", "info"),
                1 => std::env::set_var("LOG", "debug"),
                2 => std::env::set_var("LOG", "trace"),
                _ => std::env::set_var("LOG", "info"),
            };

            std::env::set_var("RUST_LOG", std::env::var("LOG").unwrap());
        }
    }

    let endpoint = matches
        .value_of("ENDPOINT")
        .expect("<ENDPOINT> is required");

    let protocol = if config.web_api.use_https {
        "https"
    } else {
        "http"
    };

    let endpoint = format!(
        "{}://{}:{}{}",
        protocol, &config.web_api.ip_address, &config.web_api.port, &endpoint
    );

    let patients = web::get_patients(&endpoint)
        .await
        .expect("failed to get patients from supplied endpoint");

    let template_path = matches
        .value_of("TEMPLATE")
        .unwrap_or(DEFAULT_TEMPLATE_PATH);
    let template = read_template_from_path(&template_path)
        .expect("failed to read template");

    for patient in &patients[..] {
        // We require that each `Patient` has at least one full name.
        assert!(!patient.names.is_empty());

        let full_name = patient.names[0].clone();

        let given = full_name.given.join(" ");
        let family = match full_name.family {
            Some(f) => f,
            None => "".to_string(),
        };

        let full_name = format!("{} {}", given, family);

        let birth_date = patient.birth_date.to_string();
        let name_tag = TagPair {
            key: "name".to_string(),
            value: full_name,
        };
        let birth_date_tag = TagPair {
            key: "birth_date".to_string(),
            value: birth_date,
        };

        let tag_pairs = vec![name_tag, birth_date_tag];

        let output = template
            .saturate(&tag_pairs)
            .expect("failed to fill template with data fetched from API");

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle
            .write_all(&output.document().as_bytes())
            .expect("failed to write out");
    }
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

fn read_from_file(path: &path::Path) -> std::io::Result<String> {
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
