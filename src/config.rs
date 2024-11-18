use std::fs;
use directories::BaseDirs;

const DEFAULT_CONFIG_FILE_PATH: &str = ".config/aqualaunch/config.toml";

pub fn load_config_from_home_directory() -> toml::Table {
    if let Some(base_dirs) = BaseDirs::new() {
        let path_to_config = base_dirs.home_dir()
            .join(DEFAULT_CONFIG_FILE_PATH);

        if let Ok(config) = fs::read_to_string(&path_to_config) {
            return config.parse::<toml::Table>().unwrap_or_else(|_| {
                log::error!("Unable to parse config at {}", &path_to_config);
                toml::Table::new()
            });
        }
    }

    log::info!("Unable to find config at default location -> HOME_DIRECTORY/{}", DEFAULT_CONFIG_FILE_PATH);
    toml::Table::new()
}
