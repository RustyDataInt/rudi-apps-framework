//! Broadly reusable app components that establish page layout.

// modules
mod fluid_page;
mod app_step_page;
mod panels;

// re-exports
pub use fluid_page::*;
pub use app_step_page::*;
pub use panels::*;

// imports
use dioxus::prelude::*;

/// Add `var(--standard-padding)` between two components in a vertical stack.
#[component]
pub fn Spacer() -> Element {
    rsx!{
        div { class: "spacer" }
    }
}

/// Add `var(--wide-padding)` between two components in a vertical stack.
#[component]
pub fn WideSpacer() -> Element {
    rsx!{
        div { class: "wide-spacer" }
    }
}
