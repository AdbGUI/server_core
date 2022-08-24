#![allow(dead_code)]
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

mod get_data;
mod upload;

pub use self::get_data::*;
pub use self::upload::*;
use crate::config_path;

pub const CONFIG_FILE: &str = "apk.json";
pub const APK_FOLDER: &str = "apk_builds";

lazy_static! {
    pub static ref APK_SERVICE_CONFG: Mutex<ApkServiceConfig> = Mutex::new(ApkServiceConfig::new());
}

pub fn get_state() -> &'static impl Deref<Target = Mutex<ApkServiceConfig>> {
    &APK_SERVICE_CONFG
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ApkServiceConfig {
    app_name: String,
    version: String,
    last_file: String,
}

impl ApkServiceConfig {
    pub fn new() -> Self {
        let cfg_path = config_path();
        let mut file_cfg = PathBuf::new();
        file_cfg.push(format!("{cfg_path}/{CONFIG_FILE}"));

        if file_cfg.exists() {
            if let Ok(file_content) = fs::read_to_string(file_cfg) {
                serde_json::from_str(&file_content).unwrap()
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }
    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    pub fn set_app_name(&mut self, app_name: String) {
        self.app_name = app_name;
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn last_file(&self) -> String {
        self.last_file.clone()
    }

    pub fn set_last_file(&mut self, last_file: String) {
        self.last_file = last_file;
    }

    pub fn is_empty(&self) -> bool {
        info!("State Name: {}", self.app_name);
        info!("State Version: {}", self.version);
        info!("State Last File: {}", self.last_file);
        self.app_name.is_empty() || self.version.is_empty() || self.last_file.is_empty()
    }
    pub fn save(&self) {
        let cfg_path = config_path();
        let file_cfg = PathBuf::from(format!("{cfg_path}/{CONFIG_FILE}"));

        let content = serde_json::to_string(self).unwrap();
        fs::write(file_cfg, content).unwrap();
    }
}
