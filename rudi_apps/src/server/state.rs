//! Top-level server metadata about the selected app and step,
//! and other global state signals, suitable for (de)serialization 
//! to|from a JSON bookmark.

// imports
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Clone, Deserialize, Serialize)]
pub struct ServerState {
    // the first three fields determine the active content page
    _suite_name:    String,
    app_name:      Option<String>,
    step_name:     Option<String>,
    // inputs and outcomes are collected over all content pages
    // values are stored as JSON to allow for arbitrary types and structures
    inputs:   HashMap<String, json::Value>,
    outcomes: HashMap<String, json::Value>,
}

impl ServerState {
    
    /// Create a new `ServerState` with no app or step selected.
    pub fn new(suite_name: &str) -> Self {
        ServerState {
            _suite_name:    suite_name.to_string(),
            app_name:      None,
            step_name:     Some("app_overview".to_string()),
            inputs:   HashMap::new(),
            outcomes: HashMap::new(),
        }
    }

    // ------------------------------------------------------
    // setters
    // ------------------------------------------------------
    /// Set the selected app name in the server state.
    pub fn set_app(&mut self, app_name: &str) {
        self.app_name = Some(app_name.to_string());
    }
    /// Set the selected step name in the server state.
    pub fn set_step(&mut self, step_name: &str) {
        self.step_name = Some(step_name.to_string());
    }
    /// Set an input value into the server state, where `id`
    /// is a globally unique, namespaced RuDI input id.
    pub fn set_input(&mut self, id: String, json: json::Value) {
        self.inputs.insert(id, json);
    }
    /// Set an outcome value into the server state, where `key`
    /// is determined by the caller.
    pub fn set_outcome(&mut self, key: String, json: json::Value) {
        self.outcomes.insert(key, json);
    }

    // ------------------------------------------------------
    // getters
    // ------------------------------------------------------
    /// Get the selected app name from the server state.
    pub fn get_app(&self) -> Option<String> {
        self.app_name.clone()
    }
    /// Get the selected step name from the server state.
    pub fn get_step(&self) -> Option<String> {
        self.step_name.clone()
    }
    /// Get an input value from the server state, where `id`
    /// is a globally unique, namespaced RuDI input id.
    pub fn get_input(&self, id: &str) -> Option<json::Value> {
        self.inputs.get(id).cloned()
    }
    /// Get an outcome value from the server state, where `key`
    /// is determined by the caller.
    pub fn get_outcome(&self, key: &str) -> Option<json::Value> {
        self.outcomes.get(key).cloned()
    }

    // ------------------------------------------------------
    // checkers
    // ------------------------------------------------------
    /// Check if any app is selected in the server state.
    pub fn has_app(&self) -> bool {
        self.app_name.is_some()
    }
    /// Check if any step is selected in the server state.
    pub fn has_step(&self) -> bool {
        self.step_name.is_some()
    }
    /// Check if a specific app is selected in the server state.
    pub fn is_app(&self, app_name: &str) -> bool {
        self.app_name == Some(app_name.to_string())
    }
    /// Check if a specific step is selected in the server state.
    pub fn is_step(&self, step_name: &str) -> bool {
        self.step_name == Some(step_name.to_string())
    }
    /// Check if a specific app and step are selected in the server state.
    pub fn is_app_step(&self, app_name: &str, step_name: &str) -> bool {
        self.app_name  == Some(app_name.to_string()) &&
        self.step_name == Some(step_name.to_string())
    }
}
