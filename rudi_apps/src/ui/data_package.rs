//! Components and associated functions for loading and processing 
//! data packages. Unlike `BookmarkLoader`, `DataPackageLoader` may 
//! be used on both the suite launch page, alongside `BookmarkLoader` 
//! and `AppChooser`, as well as on a first data loading app step. 

// imports
use dioxus::prelude::*;
use crate::components::{label::Label, input::Input};
use super::rudi_elements::*;

/// `DataPackageLoader` provides a set of inputs for loading data 
/// packages from the local file system or back-end server.
#[component]
pub fn DataPackageLoader(name: String) -> Element {
    let this = RudiElement::new::<Option<String>>(&name);
    let _value = use_signal(|| None::<String>);
    let from_local = "From your computer".to_string();
    rsx!{
        div { class: "section-title", "Load a data package from a pipeline" }
        div { class: "data-package-loader input-wrapper",
            Label { html_for: this.id.clone(), margin_top: "0", "{from_local}" }
            Input {
                id: this.id,
                r#type: "file",
                accept: ".rudi.package.zip",
                onchange: move |_evt| {},
            }
        }
    }
}
