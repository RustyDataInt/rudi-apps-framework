//! Broadly reusable, generalized app components.

// imports
use dioxus::prelude::*;

/// Add `var(--standard-padding)` between two components in a vertical stack.
#[component]
pub fn Spacer() -> Element {
    rsx!{
        div { class: "spacer" }
    }
}
