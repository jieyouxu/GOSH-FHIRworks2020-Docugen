use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path;
use toml;

/// Configuration for the `Docugen` tool.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DocugenConfig {
    pub(crate) web_api: WebApiConfig,
    pub(crate) logging: LoggingConfig,
}

impl Default for DocugenConfig {
    fn default() -> Self {
        DocugenConfig {
            web_api: WebApiConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Configuration for the intermediate Web API.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WebApiConfig {
    pub(crate) ip_address: IpAddr,
    pub(crate) port: u16,
    pub(crate) use_https: bool,
}

impl Default for WebApiConfig {
    fn default() -> Self {
        Self {
            ip_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 5001,
            use_https: true,
        }
    }
}

/// Logging configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoggingConfig {
    pub(crate) log_level: LogLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
        }
    }
}

/// Logging level.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum LogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ConfigError {
    IOError(String),
    IllFormed(String),
}

/// A type alias over possible `ConfigError`s that can be produced when trying
/// to read or parse a configuration file into the `DocugenConfig` struct.
pub(crate) type ConfigResult<T> = Result<T, ConfigError>;

/// Attempt to read configuration from a file of the given `path`.
pub(crate) fn read_config_from_path(path: &str) -> ConfigResult<DocugenConfig> {
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

    let raw_config = read_from_file(&path)?;
    let config = parse_as_toml(&raw_config)?;

    info!("Config successfully parsed as TOML");
    info!("{:#?}", &config);

    Ok(config)
}

fn read_from_file(path: &path::Path) -> ConfigResult<String> {
    let config_content =
        fs::read_to_string(path).map_err(|e| ConfigError::IOError(e.to_string()))?;

    info!("Config read:");
    info!("{:#?}", config_content);

    Ok(config_content)
}

fn parse_as_toml(raw: &str) -> ConfigResult<DocugenConfig> {
    toml::from_str::<DocugenConfig>(raw).map_err(|e| {
        error!("Failed to parse config as TOML. Check your configuration!");
        error!("Provided raw config:");
        error!("\n{}", raw);
        error!("Error cause: {:#?}", &e);
        ConfigError::IllFormed(e.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_logging_config_serialization() -> Result<(), String> {
        let raw_logging_config = r#"
            log_level = "debug"
        "#;

        let expected_logging_config = LoggingConfig {
            log_level: LogLevel::Debug,
        };

        assert_eq!(
            expected_logging_config,
            toml::from_str::<LoggingConfig>(raw_logging_config).map_err(|e| e.to_string())?
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_logging_config_serialization_failed() {
        let invalid_raw_config = "";
        toml::from_str::<LoggingConfig>(invalid_raw_config).unwrap();
    }

    #[test]
    fn test_loggin_config_deserialization() -> Result<(), String> {
        let logging_config = LoggingConfig {
            log_level: LogLevel::Trace,
        };

        let deserialized = &toml::to_string(&logging_config).map_err(|e| e.to_string())?;

        let expected_str = "log_level = \"trace\"\n";

        assert_eq!(expected_str, deserialized);

        Ok(())
    }

    #[test]
    fn test_web_api_config_serialization() -> Result<(), String> {
        let raw_web_api_config = r#"
            ip_address = "127.0.0.1"
            port = 5001
            use_https = true
        "#;

        let expected_web_api_config = WebApiConfig {
            ip_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 5001,
            use_https: true,
        };

        assert_eq!(
            expected_web_api_config,
            toml::from_str::<WebApiConfig>(raw_web_api_config).map_err(|e| e.to_string())?
        );

        Ok(())
    }

    #[test]
    fn test_web_api_config_deserialization() -> Result<(), String> {
        let web_api_config = WebApiConfig {
            ip_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 5001,
            use_https: true,
        };

        let deserialized = toml::to_string(&web_api_config).map_err(|e| e.to_string())?;

        let expected_str = r#"
            ip_address = "127.0.0.1"
            port = 5001
            use_https = true
        "#;

        let expected_str = expected_str
            .split("\n")
            .skip(1) // skip newline after raw string literal start
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(expected_str, deserialized);

        Ok(())
    }

    #[test]
    fn test_combined() -> Result<(), String> {
        let raw_combined_config = r#"
            [web_api]
            ip_address = "127.0.0.1"
            port = 5001
            use_https = true

            [logging]
            log_level = "debug"
        "#;

        let expected_combined_config = DocugenConfig {
            web_api: WebApiConfig {
                ip_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
                port: 5001,
                use_https: true,
            },
            logging: LoggingConfig {
                log_level: LogLevel::Debug,
            },
        };

        assert_eq!(
            expected_combined_config,
            toml::from_str::<DocugenConfig>(raw_combined_config).map_err(|e| e.to_string())?
        );

        Ok(())
    }
}
