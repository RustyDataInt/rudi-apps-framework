//! UI components to structure the page grid layout and initial 
//! inputs, where "shell" refers to the standard app framework 
//! layout and components. Of these, `AppChooser` and `AppStepChooser`
//! are stateful and recorded in  `use_context::<Signal<ServerState>>`,
//! although they are not themselves RudiElements.

mod header;
mod bookmark;

// imports
use dioxus::prelude::*;
use crate::server::*;
use crate::ui::*;
use header::*;
// use bookmark::*;

// constants
const APP_OVERVIEW_STEP_NAME: &str = "app_overview";

/// `SuiteLabel` is the top-left label for the tool suite.
#[component]
pub fn SuiteLabel() -> Element {
    // TODO: handle click action on suite name?  or, may be obsolete?
    let server_config = use_context::<ServerConfig>();
    rsx!{
        div { id: "rudi-suite-name", {server_config.suite_config.label} }
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
    let mut server_state  = use_context::<Signal<ServerState>>();
    let has_selected_step = server_state.read().has_step();
    let selected_app_step = server_state.read().get_step().unwrap_or(APP_OVERVIEW_STEP_NAME.to_string());
    let Some(app_name) = server_state.read().get_app() else { return rsx!{}; };
    let app_steps = &server_config.app_configs[&app_name].app_steps;
    let mut app_step_divs = vec![{
        let selected_class = if !has_selected_step { 
            "app-step-link-selected" 
        } else { "" };
        rsx!{
                div {
                class: format!("app-step-link {}", selected_class),
                key: "{APP_OVERVIEW_STEP_NAME}",
                onclick: move |_| server_state.write().set_step(APP_OVERVIEW_STEP_NAME),
                "{server_config.app_configs[&app_name].label}"
            }
        }
    }];
    app_steps.iter().for_each(|app_step_config| {
        let app_step_config_name = app_step_config.name.clone();
        let selected_class = if selected_app_step == app_step_config_name { 
            "app-step-link-selected" 
        } else { "" };
        app_step_divs.push(rsx!{
            div {
                class: format!("app-step-link {}", selected_class),
                key: "{app_step_config.name}",
                onclick: move |_| server_state.write().set_step(&app_step_config_name),
                "{app_step_config.label}"
            }
        });
    });
    rsx!{
        {app_step_divs.iter()}
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
    use_context_provider(|| Namespace("app_chooser".to_string()));
    let server_config = use_context::<ServerConfig>();
    let mut app_names: Vec<String> = server_config.app_configs.keys().map(|k| k.clone()).collect();
    app_names.sort_by_key(|app_name| server_config.app_configs[app_name].order);
    let apps = app_names.into_iter().map(|app_name| {
        let app_config = &server_config.app_configs[&app_name];
        let app_config_name = app_config.name.clone();
        rsx!{
            div { key: "{app_config.name}",
                DataPackageLoader { name: "TMP".to_string() }
                WideSpacer {}
                div { class: "section-title", "Open an app with no data" }
                div {
                    class: "app-chooser-row",
                    onclick: move |_| consume_context::<Signal<ServerState>>().write().set_app(&app_config_name),
                    div { class: "app-chooser-label", "{app_config.label}" }
                    div { class: "app-chooser-description", "{app_config.description}" }
                }
            }
        }
    });
    rsx!{
        {apps}
    } 
}

/// Render the app's overview.md as an app step.
#[component]
pub fn AppOverview() -> Element {
    let server_config = use_context::<ServerConfig>();
    let server_state = use_context::<Signal<ServerState>>();
    let app_step_name = server_state.read().get_step();
    let Some(app_name) = server_state.read().get_app() else { return rsx!{}; };
    rsx!{
        if app_step_name.is_none() || app_step_name.unwrap() == APP_OVERVIEW_STEP_NAME {
            Markdown { markdown: server_config.app_overviews[&app_name].clone() }
        }
    }
}
