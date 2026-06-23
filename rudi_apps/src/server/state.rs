//! Top-level server metadata about the selected app and step,
//! and other global state signals.

// imports
use std::collections::HashMap;
use dioxus::prelude::*;

/// The state of the RuDI server, saveable as a bookmark.
#[derive(Clone, Copy)]
pub struct ServerState {
    _suite_name:    Signal<String>,
    app_name:      Signal<Option<String>>,
    step_name:     Signal<Option<String>>,
    _step_states:   Signal<HashMap<String, String>>,
    _step_outcomes: Signal<HashMap<String, String>>,
}
impl ServerState {
    
    /// Create a new `ServerState` with no app or step selected.
    pub fn new(suite_name: &str) -> Self {
        ServerState {
            _suite_name:    Signal::new(suite_name.to_string()),
            app_name:      Signal::new(None),
            step_name:     Signal::new(None),
            _step_states:   Signal::new(HashMap::new()),
            _step_outcomes: Signal::new(HashMap::new()),
        }
    }

    // ------------------------------------------------------
    // setters
    // ------------------------------------------------------
    /// Set the selected app name in the server state.
    pub fn set_app(&mut self, app_name: &str) {
        self.app_name.set(Some(app_name.to_string()));
    }
    /// Set the selected step name in the server state.
    pub fn set_step(&mut self, step_name: &str) {
        self.step_name.set(Some(step_name.to_string()));
    }

    // ------------------------------------------------------
    // getters
    // ------------------------------------------------------
    /// Get the selected app name from the server state.
    pub fn get_app(&self) -> Option<String> {
        self.app_name.cloned()
    }
    /// Get the selected step name from the server state.
    pub fn get_step(&self) -> Option<String> {
        self.step_name.cloned()
    }

    // ------------------------------------------------------
    // checkers
    // ------------------------------------------------------
    /// Check if any app is selected in the server state.
    pub fn has_app(&self) -> bool {
        self.app_name.read().is_some()
    }
    /// Check if any step is selected in the server state.
    pub fn has_step(&self) -> bool {
        self.step_name.read().is_some()
    }
    /// Check if a specific app is selected in the server state.
    pub fn is_app(&self, app_name: &str) -> bool {
        *self.app_name.read() == Some(app_name.to_string())
    }
    /// Check if a specific step is selected in the server state.
    pub fn is_step(&self, step_name: &str) -> bool {
        *self.step_name.read() == Some(step_name.to_string())
    }
    /// Check if a specific app and step are selected in the server state.
    pub fn is_app_step(&self, app_name: &str, step_name: &str) -> bool {
        *self.app_name.read()  == Some(app_name.to_string()) &&
        *self.step_name.read() == Some(step_name.to_string())
    }
}
