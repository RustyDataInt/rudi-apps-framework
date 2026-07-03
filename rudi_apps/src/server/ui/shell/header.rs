//! The feedback elements in the top page header.

// imports
use std::env;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Menu, FolderGit2};
// use crate::server::*;

const ICON_SIZE: u32 = 24;

/// `ToggleSidebarLink` provides an icon link to toggle the sidebar.
#[component]
pub fn ToggleSidebarLink() -> Element {
    rsx!{
        button {
            Menu { size: ICON_SIZE }
        }
    }
}

/// `GitVersions` provides an icon link to a modal with repo version metadata.
#[component]
pub fn GitVersions() -> Element {
    rsx!{
        button {
            FolderGit2 { size: ICON_SIZE }
        }
    }
}

/// `ActiveUser` display the currently logged-in user.
#[component]
pub fn ActiveUser() -> Element {
    let user = env::var("USER")
        .map(|val| val.to_string())
        .unwrap_or("USER not set".to_string());
    let hostname = env::var("HOSTNAME")
        .map(|val| val.to_string())
        .unwrap_or("".to_string());
    let hostname = if hostname.is_empty() { 
        "".to_string() 
    } else { 
        format!("@{}", hostname) 
    };
    rsx!{
        div { id: "active-user",
            {user}
            {hostname}
        }
    }
}

/// `DataDirectory` display the path the the server data directory.
#[component]
pub fn DataDirectory() -> Element {
    let data_dir = env::var("RUDI_DATA_DIR")
        .map(|val| val.to_string())
        .unwrap_or("RUDI_DATA_DIR not set".to_string());
    rsx!{
        div { id: "data-directory", {data_dir} }
    }
}
