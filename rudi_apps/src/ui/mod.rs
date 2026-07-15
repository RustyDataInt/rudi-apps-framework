//! UI components for the RuDI apps server.

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

// constants
pub const DEFAULT_INPUT_WIDTH: u16 = 75; // i.e., a single-column input
pub const SCREEN_DPI: u16 = 96;
pub const POINTS_PER_INCH: f64 = 72.0;

/// A `u16` representing the width of an input in pixels
/// suitable for type-specific use in `use_context_provider`.
#[derive(PartialEq, Clone)]
pub struct InputWidth(pub u16);

/// `UiState` holds ephemeral information about the current state 
/// of the app (in contrast to `ServerState` which is stateful).
#[derive(Clone, PartialEq)]
pub struct UiState {
    pub showing_app_steps:    bool, // whether the sidebar is currently rendering app step tabs
    pub sidebar_open:         bool, // whether the sidebar is currently open
}
impl UiState {
    /// Create a new `UiState` with app_steps open.
    pub fn new() -> Self {
        UiState {
            showing_app_steps: true,
            sidebar_open:      true,
        }
    }
    /// Trigger a transition to a new `UiState` with the sidebar closed.
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
    /// Trigger a transition to a new `UiState` showing app_step tabs
    /// in the open state.
    pub fn open_app_steps() -> Self {
        UiState {
            showing_app_steps: true,
            sidebar_open:      true,
        }
    }
    /// Trigger a transition to a new `UiState` showing app_step instructions
    /// in the open state.
    pub fn open_instructions() -> Self {
        UiState {
            showing_app_steps: false,
            sidebar_open:      true,
        }
    }
}

/// A platform-agnostic async sleep for timing animations.
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
