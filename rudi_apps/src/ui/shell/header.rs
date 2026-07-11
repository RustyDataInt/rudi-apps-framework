//! The feedback elements in the top page header. These components
//! show metadata on the server and connection and are not stateful,
//! namespaced, or serializable, i.e., they are not RudiElements.

// imports
use std::env;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Menu, FolderGit2};
use crate::ui::*;

// constants
const ICON_SIZE: u32 = 24;

/// `ToggleSidebarLink` provides an icon link to toggle the sidebar.
#[component]
pub fn ToggleSidebarLink() -> Element {
    let onclick = move |_| {
        let mut ui_state = consume_context::<Signal<UiState>>();
        let showing_app_steps = ui_state.read().showing_app_steps;
        if ui_state.read().sidebar_open {
            ui_state.set(UiState::close_sidebar(showing_app_steps));
        } else {
            ui_state.set(UiState::open_sidebar(showing_app_steps));
        }
    };
    rsx!{
        button { onclick,
            Tooltip { text: "Toggle the sidebar".to_string(),
                Menu { size: ICON_SIZE }
            }
        }
    }
}

/// `GitVersions` provides an icon link to a modal with repo version metadata.
#[component]
pub fn GitVersions() -> Element {
    // TODO: implement git version server function
    rsx!{
        button {
            FolderGit2 { size: ICON_SIZE }
        }
    }
}

/// `ActiveUser` displays the currently logged-in user.
#[component]
pub fn ActiveUser() -> Element {
    let user = env::var("USER")
        .map(|val| val.to_string())
        .unwrap_or("unknown_user".to_string());
    let hostname = env::var("HOSTNAME")
        .map(|val| val.to_string())
        .unwrap_or("".to_string());
    let value = if hostname.is_empty() { 
        user
    } else { 
        format!("{user}@{hostname}") 
    };
    rsx!{
        div { id: "active-user", {value} }
    }
}

/// `DataDirectory` displays the path the the server data directory.
#[component]
pub fn DataDirectory() -> Element {
    let data_dir = env::var("RUDI_DATA_DIR")
        .map(|val| val.to_string())
        .unwrap_or("!! RUDI_DATA_DIR not set !!".to_string());
    rsx!{
        div { id: "data-directory", {data_dir} }
    }
}
