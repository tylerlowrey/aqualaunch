use std::fs;
use directories::BaseDirs;

const DEFAULT_CONFIG_FILE_PATH: &str = ".config/aqualaunch/config.toml";

pub fn load_config_from_home_directory() -> toml::Table {
    log::info!("Loading config from home directory");
    if let Some(base_dirs) = BaseDirs::new() {
        let path_to_config = base_dirs.home_dir()
            .join(DEFAULT_CONFIG_FILE_PATH);

        log::info!("Loading config from file path: {}", path_to_config.display());

        return load_config(path_to_config.as_os_str().to_str().unwrap());
    }

    log::info!("Unable to find config at default location -> HOME_DIRECTORY/{}", DEFAULT_CONFIG_FILE_PATH);
    toml::Table::new()
}

pub fn load_config(path: &str) -> toml::Table {
    if let Ok(config) = fs::read_to_string(path) {
        config.parse::<toml::Table>().unwrap_or_else(|_| {
            log::error!("Unable to parse config at {}", path);
            toml::Table::new()
        })
    } else {
        log::warn!("Unable to read config at {}", path);
        toml::Table::new()
    }
}
