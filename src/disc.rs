use crate::options::Message;
use crate::color::{Color};

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const MAIN_DIR_NAME:        &str = env!("CARGO_PKG_NAME");
const DOT_CONFIG_DIR_NAME:  &str = ".config";
const CONFIG_FILE_NAME:     &str = "config.json";
const LOG_FILE_NAME:        &str = ".log";
const DEFAULT_CONFIG_FILE:  &str = include_str!("default_config.json");

type BoxErr = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub api_key:        Option<String>,
    pub model:          String,
    pub temperature:    f32,
    pub color:          Color,
}

impl Config {
    pub fn print(&self) {
        let api_key = self.api_key
            .as_deref()
            .unwrap_or("Not set in config");
        println!("Current configuration:");
        println!("      api_key:      {}", api_key);
        println!("      model:        {}", self.model);
        println!("      temperature:  {}", self.temperature);
        println!("      color:        {}{}{}",
                 self.color.ansi(),
                 self.color.as_str(),
                 Color::NoColor.ansi());
    }
}

fn get_path_to_main_dir() -> Result<PathBuf, BoxErr> {
    let path = dirs::home_dir()
        .ok_or("Failed to get home directory")?
        .join(DOT_CONFIG_DIR_NAME)
        .join(MAIN_DIR_NAME);
    fs::create_dir_all(&path)?;

    Ok(path)
}

fn get_path_to_config_file() -> Result<PathBuf, BoxErr> {
    let path_to_config_file = get_path_to_main_dir()?
        .join(CONFIG_FILE_NAME);
    if !path_to_config_file.exists() {
        fs::write(&path_to_config_file, DEFAULT_CONFIG_FILE)?;
    }

    Ok(path_to_config_file)
}

pub fn load_config() -> Result<Config, BoxErr> {
    let path_to_config_file = get_path_to_config_file()?;
    let json_string = fs::read_to_string(&path_to_config_file)?;
    let config: Config = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    Ok(config)
}

pub fn write_config(config: &Config) -> Result<(), BoxErr> {
    let path_to_config_file = get_path_to_config_file()?;
    let json = serde_json::to_string(&config)?;
    fs::write(&path_to_config_file, json)?;

    Ok(())
}

fn get_path_to_log() -> Result<PathBuf, BoxErr> {
    let path_to_log = get_path_to_main_dir()?
        .join(LOG_FILE_NAME);

    Ok(path_to_log)
}

pub fn clear_log() -> Result<(), BoxErr> {
    let path_to_log = get_path_to_log()?;
    fs::remove_file(&path_to_log)?;

    Ok(())
}

pub fn load_log() -> Result<Vec<Message>, BoxErr> {
    let path_to_log = get_path_to_log()?;
    if !path_to_log.exists() {
        return Ok(Vec::new());
    }

    let log_str = fs::read_to_string(&path_to_log)?;
    let messages = serde_json::from_str(&log_str)?;

    Ok(messages)
}

pub fn write_log(messages: &[Message]) -> Result<(), BoxErr> {
    let path_to_log = get_path_to_log()?;
    let json = serde_json::to_string(messages)?;
    fs::write(&path_to_log, json)?;

    Ok(())
}
