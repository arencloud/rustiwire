use std::io::{Result};
use std::fs::{File, metadata};
use std::{fs};
use std::path::PathBuf;
use clap::{Parser};
use dialoguer::theme::ColorfulTheme;
use serde_json;
use serde::{Serialize, Deserialize};
use dirs::home_dir;

static CONFIGURATION_FILE: &str = ".wire_config";

/// Prompt that takes user input and returns a string.
#[derive(Debug, Parser)]
pub struct Configuration;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id_name: String,
    pub api_uri: String,
    pub token: String,
    pub used: bool,
}


impl Configuration {
    pub fn set(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id_name = input.with_prompt("Name: ").allow_empty(false).interact_text().unwrap();
        let api_uri = input.with_prompt("API URI: ").allow_empty(false).interact_text().unwrap();
        let mut input_token = dialoguer::Password::with_theme(&theme);
        let token = input_token.with_prompt("API Token: ").allow_empty_password(false).interact().unwrap();
        let mut cfg = Config {
            id_name,
            api_uri,
            token,
            used: false,
        };
        Configuration::save(&mut cfg);
    }
    pub fn get_config() -> Result<Config> {
        let path = Configuration::config_home();
        match path.exists() {
            true => {
                let content = fs::read_to_string(path).unwrap();
                let config_deserialized: Config = serde_json::from_str(&content).unwrap();
                Ok(config_deserialized)
            },
            false => {
                println!("set at least single config set, executing following command: wirecli config set");
                std::process::exit(0);
            }
        }
    }
    fn save(config_data: &mut Config) {
        let p = Configuration::config_home();
        match metadata(p.clone()).is_ok() {
            true => println!("configuration is set"),
            false => {
                config_data.used = true;
                let file = File::create(p.clone()).unwrap();
                serde_json::to_writer(file, &config_data).unwrap();
            }
        }
    }
    fn config_home() -> PathBuf {
        let h = home_dir().unwrap();
        h.join(CONFIGURATION_FILE)
    }
}