//! UI components for the RuDI server application interface wrapper.

// imports
use dioxus::prelude::*;
// use rudi::server::config::*;

/// `SuiteLabel` is the top-left label for the tool suite.
#[component]
fn SuiteLabel() -> Element {
    let server_config = use_context::<ServerConfig>();
    rsx!{
        div { id: "rudi-suite-name",
            {server_config.suite_config.label}
        }
    }
}

/// `ServerHeader` fills the top-right page header feedback area.
#[component]
fn ServerHeaderContent() -> Element {
    rsx!{
        div { id: "rudi-header-content",
            "pending"
        }
    }
}

/// `AppStepChooser` is a switchboard for selecting the app step
/// to bring into view.  All app steps components are mounted once
/// an app loads; the chooser only toggles their visibility.
#[component]
fn AppStepChooser() -> Element {
    let server_config = use_context::<ServerConfig>();
    let server_state  = use_context::<ServerState>();
    let Some(app_name) = server_state.get_app() else { return rsx!{}; };
    rsx!{
        for app_step_config in server_config.app_configs[&app_name].app_steps {
            let app_step_name = app_step_config.name.clone();
            button { class: "app-step-link",
                key: "{app_step_config.name}",
                onclick: move |event| {
                    server_state.set_step(&app_step_name);
                },
                "{app_step_config.label}"
            }
        }
    }
}

/// `AppChooser` is a switchboard for selecting the app to use.
/// It is rendered on first load into an uninitialized server,
/// never to be rendered again once an app is selected.
#[component]
fn AppChooser() -> Element {
    let server_config = use_context::<ServerConfig>();
    let mut app_names = server_config.app_configs.keys().clone();
    app_names.sort_by_key(|app_name| server_config.app_configs[app_name].order);
    rsx!{
        for app_name in app_names {
            let app_config = &server_config.app_configs[&app_name];
            button { 
                key = "{app_name}",
                onclick: move |event| {
                    consume_context::<ServerState>().set_app(&app_name);
                },
                "{app_config.label}"
            }
        }
    }
}
