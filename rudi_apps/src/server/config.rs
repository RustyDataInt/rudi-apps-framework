//! Suite, App, and AppStep configuration structures, that match the
//! expected format of `suite_config.toml` and `app_config.toml` files.

// imports
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// /// Collect server configurations across multiple tool suites. The 
// /// different suites may be part of a multi-suite installation, 
// /// or result from a single-suite installation that declares a 
// /// suite dependency.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct InstallationConfig {
//     pub single_suite_name: Option<String>,
//     pub server_configs: HashMap<String, ServerConfig>,
// }

/// Collect all configurations for one tool suite.
/// Instantiated by `build.rs` for `server::main.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub dir:  String,
    pub name: String,
    pub suite_config:  SuiteConfig,
    pub app_configs:   HashMap<String, AppConfig>,
    pub app_overviews: HashMap<String, String>,
}

/// Tool suite configuration as read from `suite_config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteConfig {
    pub name:  String,
    pub label: String,
    pub description: String,
    pub max_upload_megabytes: u16,
}

/// Collect the app step configurations for one app 
/// as read from its `app_config.toml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub order: u8,
    pub name:  String,
    pub label: String,
    pub description: String,
    pub package_types: HashMap<String, Vec<String>>, // values are required content file types
    pub app_steps: Vec<AppStepConfig>,
}

/// The terminal configuration object that defines the Dioxus 
/// component used by an app step. The component must be 
/// re-exported in the app's `lib.rs` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStepConfig {
    #[serde(default)]
    pub order:        u8,
    pub name:         String,
    pub label:        String,
    pub component:    String, // e.g. "MyAppStepComponent"
    pub title:        String,
    #[serde(default)]
    pub tooltip:      Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,}
