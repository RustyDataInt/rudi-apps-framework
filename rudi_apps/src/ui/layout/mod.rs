//! Broadly reusable app components that establish page layout.
//! These are organizing wrappers, not namespaced components.

// modules
mod fluid_page;
mod app_step_page;
mod data_panel;

// re-exports
pub use fluid_page::*;
pub use app_step_page::*;
pub use data_panel::*;

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
