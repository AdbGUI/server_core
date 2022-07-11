#![allow(dead_code)]
use std::ops::Deref;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

mod get_data;
mod upload;

pub use self::get_data::*;
pub use self::upload::*;

pub const CONFIG_FILE: &str = "apk.json";
pub const APK_FOLDER: &str = "apk_builds";

lazy_static! {
    pub static ref APK_SERVICE_CONFG: Mutex<ApkServiceConfig> = Mutex::new(Default::default());
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
        self.app_name.is_empty() || self.version.is_empty() || self.last_file.is_empty()
    }
}
