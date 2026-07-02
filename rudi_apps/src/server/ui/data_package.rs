//! UI components and functions for loading and processing data packages.

// imports
use dioxus::prelude::*;
use crate::server::*;

/// `DataPackageLoader` provides a set of inputs for loading data packages
/// from the local file system or back-end server.
#[component]
pub fn DataPackageLoader() -> Element {
    rsx!{
        div{
            class: "section-title",
            "Load a data package"
        }
        div { class: "data-package-loader",
            input{
                id: ".data-package-loader",
                r#type: "file",
                onchange: move |evt| {
                    // let Some(file) = evt.value.clone() else { return; };
                    // let file = file.to_rstring();
                    // let server_state = consume_context::<ServerState>();
                    // server_state.load_data_package(&file);
                }
            }
        }
    }
}
