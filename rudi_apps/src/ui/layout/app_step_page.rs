//! Components that define the standardized app step page layout.

// imports
use dioxus::prelude::*;
use crate::ui::layout::*;
use crate::ui::rudi_elements::*;

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

    // Pass the namespace of the app step to all child components.
    use_context_provider(|| Namespace::from(&props.app_step));

    // TODO: add FluidRows for header and instructions before caller's child rows

    rsx! {
        div { id: props.app_step.id.clone(),
            FluidPage { max_width: props.max_width.unwrap_or_else(|| "100%".to_string()),

                {props.children}
            }
        }
    }
}
