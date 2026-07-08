//! Components that define the standardized app step page layout.

// imports
use dioxus::prelude::*;
use dioxus_icons::lucide::{Info, Settings};
use crate::server::*;
use crate::ui::*;

// constants
const ICON_SIZE: u32 = 24;

/// Dioxus Properties for an `AppStep`.
#[derive(PartialEq, Clone, Props)]
pub struct AppStepPageProps{
    app_step:  RudiElement,
    max_width: Option<String>, // allow callers to prevent overly wide pages
    children:  Element, // one or more FluidRow components with their children
}

/// The `AppStep` component makes it easy to create a standardized 
/// app step page in RuDI style. The step header, links, and  
/// instructions are automatically added to the top of the page.
/// 
/// An `app_step` created with 
/// `RudiElement::app_step::<()>("app_step_name")` is required,
/// as are one or more `FluidRow` components with their children.
/// 
/// The `max_width` property is optional and allows the caller to
/// prevent overly wide pages. The default value is '100%'.
#[component]
pub fn AppStepPage(props: AppStepPageProps) -> Element {
    use_context_provider(|| Namespace::from(&props.app_step));
    rsx! {
        div { id: props.app_step.id.clone(),
            FluidPage { max_width: props.max_width.unwrap_or_else(|| "100%".to_string()),
                RudiAppStepHeader { app_step_name: props.app_step.name.clone() }
                {props.children}
            }
        }
    }
}

#[component]
fn RudiAppStepHeader(app_step_name: String) -> Element {
    let server_config = use_context::<ServerConfig>();
    let server_state  = use_context::<Signal<ServerState>>();
    let Some(app_name) = server_state.read().get_app() else { return rsx!{}; };
    let app_step_config = server_config.app_configs[&app_name].app_steps.iter()
        .find(|step| step.name == app_step_name);
    let Some(app_step_config) = app_step_config else { return rsx!{}; };
    rsx!{
        FluidRow {
            FluidSpan { n_columns: 12,
                div { class: "app-step-header",
                    div { margin_bottom: "7px", "{app_step_config.title}" }
                    if app_step_config.instructions.is_some() {
                        div { class: "app-step-header-icon",
                            div {
                                onclick: move |_| {
                                    let mut ui_state = consume_context::<Signal<UiState>>();
                                    if ui_state.read().showing_app_steps {
                                        ui_state.set(UiState::open_instructions());
                                    } else {
                                        ui_state.set(UiState::open_app_steps());
                                    }
                                },
                                Tooltip { text: "Toggle app step instructions".to_string(),
                                    Info { size: ICON_SIZE }
                                }
                            }
                        }
                    }
                    if app_step_config.settings.is_some() {
                        div { class: "app-step-header-icon",
                            div {
                                onclick: move |_| {
                                    let mut _ui_state = consume_context::<Signal<UiState>>();
                                }, // TODO: settings modal
                                Settings { size: ICON_SIZE }
                            }
                        }
                    }
                }
            }
        }
    }
}
