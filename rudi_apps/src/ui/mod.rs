//! UI components for the RuDI server application interface wrapper.

// modules
mod shell;
mod data_package;
mod markdown;
mod layout;
mod rudi_elements;

// re-exports
pub use shell::*;
pub use data_package::*;
pub use markdown::*;
pub use layout::*;
pub use rudi_elements::*;

// imports
use dioxus::prelude::*;

/// A simple label for an input. Note that this label component 
/// is just text, and does not claim a connection to the input 
/// it is labeling.
#[component]
pub fn InputLabel(label: String) -> Element {
    rsx! {
        div { class: "rudi-input-label", "{label}" }
    }
}
