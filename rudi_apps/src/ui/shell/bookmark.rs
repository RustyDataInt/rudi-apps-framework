//! Components and associated functions for loading and processing 
//! server state bookmarks. Unlike `DataPackageLoader`, `BookmarkLoader`
//! use is restrcited to the suite launch page.

// imports
use dioxus::prelude::*;
// use crate::server::*;

/// `BookmarkLoader` provides a set of inputs for loading server state 
/// bookmarks from the local file system or back-end server.
#[component]
pub fn BookmarkLoader() -> Element {
    rsx!{
        div { class: "section-title", "Load a bookmark" }
        div { class: "bookmark-loader",
            input {
                id: "bookmark-loader",
                r#type: "file",
                onchange: move |_evt| {}, // let Some(file) = evt.value.clone() else { return; };,
            }
        }
    }
}
