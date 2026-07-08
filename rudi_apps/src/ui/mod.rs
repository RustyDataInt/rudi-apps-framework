//! UI components for the RuDI server application interface wrapper.

// modules
mod shell;
mod data_package;
mod markdown;
mod tooltip;
mod layout;
mod rudi_elements;

// re-exports
pub use shell::*;
pub use data_package::*;
pub use markdown::*;
pub use tooltip::*;
pub use layout::*;
pub use rudi_elements::*;

// imports
use dioxus::prelude::*;

/// `UiState` holds ephemeral information about the current state 
/// of the app (in contrast to `ServerState` which is stateful).
#[derive(Clone, PartialEq)]
pub struct UiState {
    pub showing_app_steps:    bool,
    pub sidebar_open:         bool,
}
impl UiState {
    /// Create a new `UiState` with app_steps open.
    pub fn new() -> Self {
        UiState {
            showing_app_steps: true,
            sidebar_open:      true,
        }
    }
    /// Trigger a transition to a new `UiState` with no open sidebar.
    pub fn close_sidebar(showing_app_steps: bool) -> Self {
        UiState {
            showing_app_steps: showing_app_steps,
            sidebar_open:      false,
        }
    }
    /// Trigger a transition to a new `UiState` with the sidebar open.
    pub fn open_sidebar(showing_app_steps: bool) -> Self {
        UiState {
            showing_app_steps: showing_app_steps,
            sidebar_open:      true,
        }
    }
    /// Trigger a transition to a new `UiState` showing app_step tabs.
    pub fn open_app_steps() -> Self {
        UiState {
            showing_app_steps: true,
            sidebar_open:      true,
        }
    }
    /// Trigger a transition to a new `UiState` showing app_step instructions.
    pub fn open_instructions() -> Self {
        UiState {
            showing_app_steps: false,
            sidebar_open:      true,
        }
    }
}

/// A platform-agnostic async sleep helper
pub async fn async_delay(delay_ms: u32) {
    #[cfg(not(feature = "server"))]
    {
        gloo_timers::future::TimeoutFuture::new(delay_ms).await;
    }
    #[cfg(feature = "server")]
    {
        tokio::time::sleep(std::time::Duration::from_millis(delay_ms as u64)).await;
    }
}

/// A simple label for an input. Note that this label component 
/// is just text, and does not claim a connection to the input 
/// it is labeling.
#[component]
pub fn InputLabel(label: String) -> Element {
    rsx! {
        div { class: "rudi-input-label", "{label}" }
    }
}
