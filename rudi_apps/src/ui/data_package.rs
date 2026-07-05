//! Components and associated functions for loading and processing 
//! data packages. Unlike `BookmarkLoader`, `DataPackageLoader` may 
//! be used on both the suite launch page, alongside `BookmarkLoader` 
//! and `AppChooser`, as well as on a first data loading app step. 

// imports
use dioxus::prelude::*;
use super::rudi_elements::*;

/// `DataPackageLoader` provides a set of inputs for loading data 
/// packages from the local file system or back-end server.
#[component]
pub fn DataPackageLoader(name: String) -> Element {
    let this = RudiElement::new::<Option<String>>(&name);
    let _value = use_signal(|| None::<String>);
    rsx!{
        div { class: "section-title", "Load a data package" }
        div { class: "data-package-loader",
            input { id: this.id, r#type: "file", onchange: move |_evt| {} }
        }
    }
}
