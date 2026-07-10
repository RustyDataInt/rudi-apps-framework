//! Components and associated functions for loading and saving 
//! server state bookmarks. 

// imports
use dioxus::prelude::*;
use crate::components::{label::Label, input::Input};
use crate::server::ServerState;

/// `BookmarkLoader` provides a set of inputs for loading server 
/// state bookmarks from the local file system or back-end server.
/// 
/// Unlike `DataPackageLoader`, `BookmarkLoader` use is restricted 
/// to the suite launch page.
#[component]
pub fn BookmarkLoader() -> Element {
    const ID: &str = "bookmark-loader";
    let from_local = "From your computer".to_string();
    rsx!{
        div { class: "section-title", "Load a previously saved bookmark" }
        div { class: "bookmark-loader input-wrapper",
            Label { html_for: ID.to_string(), margin_top: "0", "{from_local}" }
            Input {
                id: ID.to_string(),
                r#type: "file",
                accept: ".json",
                oninput: move |e: FormEvent| {
                    let mut server_state = consume_context::<Signal<ServerState>>();
                    spawn(async move {
                        log::info!("BookmarkLoader: file input changed");
                        let files = e.files();
                        let Some(file_data) = files.first() else {
                            return;
                        };
                        log::info!("BookmarkLoader: file selected: {:?}", file_data.name());
                        let Ok(json) = file_data.read_string().await else {
                            log::error!(
                                "BookmarkLoader: failed to read JSON from file {}", file_data.name()
                            );
                            return;
                        };
                        match serde_json::from_str::<ServerState>(&json) {
                            Ok(state) => {
                                log::info!("BookmarkLoader: bookmark JSON parsed successfully");
                                server_state.set(state);
                            }
                            Err(e) => {
                                log::error!("Failed to parse bookmark JSON: {:?}", e);
                            }
                        }
                    });
                },
            }
        }
    }
}

/// `BookmarkSaver` provides links for saving server state bookmarks
/// to the local file system or back-end server.
#[component]
pub fn BookmarkSaver() -> Element {
    let server_state = use_context::<Signal<ServerState>>();
    if !server_state.read().has_app() {
        return rsx!{};
    }
    rsx!{
        div { id: "save-bookmark-wrapper",
            div { id: "save-bookmark", "Save Your Work!" }
            div {
                id: "save-bookmark-local",
                class: "save-bookmark-type",
                onclick: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let server_state = consume_context::<Signal<ServerState>>();
                        download_json(&*server_state.read(), "my.rudi.bookmark.json")
                            .unwrap_or_else(|e| {
                                log::error!("Failed to download server state as JSON file: {:?}", e);
                            });
                    }
                },
                "- to your computer"
            }
            div { id: "save-bookmark-server", class: "save-bookmark-type", "- to the server" }
        }
    }
}

/// Serialize and download data as a JSON file to the local file system.
#[cfg(target_arch = "wasm32")]
fn download_json<T: serde::Serialize>(data: &T, filename: &str) -> Result<(), Vec<String>> {
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::{js_sys::Array, BlobPropertyBag, Blob, Url, window, HtmlAnchorElement};

    let json = serde_json::to_string_pretty(data)
        .map_err(|e| vec![format!("Serialization failed: {}", e)])?;

    let parts = Array::of1(&JsValue::from_str(&json));
    let blob_properties = BlobPropertyBag::new();
    blob_properties.set_type("application/json");

    let blob = Blob::new_with_str_sequence_and_options(&parts, &blob_properties)
        .map_err(|_| vec!["Failed to create Blob".to_string()])?;

    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|_| vec!["Failed to create Url".to_string()])?;

    let window = window().ok_or_else(|| vec!["No window found".to_string()])?;
    let document = window.document().ok_or_else(|| vec!["No document found".to_string()])?;
    let anchor: HtmlAnchorElement = document.create_element("a")
        .map_err(|_| vec!["Failed to create anchor element".to_string()])?
        .dyn_into::<HtmlAnchorElement>()
        .map_err(|_| vec!["Failed to cast anchor element".to_string()])?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    let _ = Url::revoke_object_url(&url);
    Ok(())
}
