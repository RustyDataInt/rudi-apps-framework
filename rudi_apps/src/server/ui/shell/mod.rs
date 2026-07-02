//! UI components to structure the page layout and initial inputs,
//! where "shell" refers to the standard app framework layout and components.

// modules
mod header;

// imports
use dioxus::prelude::*;
use crate::server::*;
use header::*;

/// `SuiteLabel` is the top-left label for the tool suite.
#[component]
pub fn SuiteLabel() -> Element {
    // TODO: handle click action on suite name?  or, may be obsolete?
    let server_config = use_context::<ServerConfig>();
    rsx!{
        div { id: "rudi-suite-name",
            {server_config.suite_config.label}
        }
    }
}

/// `ServerHeader` fills the top-right page header feedback area.
#[component]
pub fn ServerHeaderContent() -> Element {
    rsx!{
        div { id: "rudi-header-content",
            div { id: "rudi-header-content-icons",
                ToggleSidebarLink {}
                GitVersions {}
            }
            div { id: "rudi-header-content-server-data",
                ActiveUser {}
                DataDirectory {}
            }
        }
    }
}

/// `AppStepChooser` is a switchboard for selecting the app step
/// to bring into view.  All app steps components are mounted once
/// an app loads; the chooser only toggles their visibility.
#[component]
pub fn AppStepChooser() -> Element {
    let server_config = use_context::<ServerConfig>();
    let mut server_state  = use_context::<ServerState>();
    let Some(app_name) = server_state.get_app() else { return rsx!{}; };
    let app_steps = &server_config.app_configs[&app_name].app_steps;
    let app_steps = app_steps.iter().map(|app_step_config| {
        let app_step_config_name = app_step_config.name.clone();
        rsx!{
            div { class: "app-step-link",
                key: "{app_step_config.name}",
                onclick: move |_| server_state.set_step(&app_step_config_name),
                "{app_step_config.label}"
            }
        }
    });
    rsx!{
        {app_steps}
    }
}

/// Error handler for malformed config.
#[component]
pub fn AppNotFound(app_name : String) -> Element {
    rsx!{ 
        {format!("Error: App '{}' not found.", app_name)} 
    }
}

/// `AppChooser` is a switchboard for selecting the app to use.
/// It is rendered on first load into an uninitialized server,
/// never to be rendered again once an app is selected.
#[component]
pub fn AppChooser() -> Element {
    let server_config = use_context::<ServerConfig>();
    let mut app_names: Vec<String> = server_config.app_configs.keys().map(|k| k.clone()).collect();
    app_names.sort_by_key(|app_name| server_config.app_configs[app_name].order);
    let apps = app_names.into_iter().map(|app_name| {
        let app_config = &server_config.app_configs[&app_name];
        let app_config_name = app_config.name.clone();
        rsx!{
            DataPackageLoader {}
            Spacer {}
            div{
                class: "section-title",
                "Open an app with no data"
            }
            div { 
                class: "app-chooser-row",
                key: "{app_config.name}",
                onclick: move |_| consume_context::<ServerState>().set_app(&app_config_name),
                div{ class: "app-chooser-label",
                    "{app_config.label}"
                }
                div{ class: "app-chooser-description",
                    "{app_config.description}"
                }
            }   
        }
    });
    rsx!{
        {apps}
    } 
}
