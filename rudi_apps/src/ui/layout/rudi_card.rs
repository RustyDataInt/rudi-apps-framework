//! A component for a standardized data display panel.
//! This is a wrapper element that defines a RuDI namespace 
//! for its children so that similarly named inputs might
//! be found in different cards without conflict.

// imports
use dioxus::prelude::*;
use crate::ui::*;

/// Dioxus Properties for a `TextInput`.
#[derive(PartialEq, Clone, Props)]
pub struct RudiCardProps {
    // required
    name:      String,
    // optional
    title:     Option<String>,
    // from FluidSpanProps
    n_columns: u8,
    min_width: Option<String>, // allow callers to prevent overly narrow panels
    children: Element,
}

/// A component for a standardized data display panel
/// with a header bar containing action link icons.
#[component]
pub fn RudiCard(props: RudiCardProps) -> Element {

    let this = RudiElement::new::<()>(&props.name);
    use_context_provider(|| Namespace::from(&this));
    let title = props.title.clone().unwrap_or("NO TITLE".to_string());

    rsx!{
        FluidSpan { n_columns: props.n_columns, min_width: props.min_width,
            div { class: "rudi-card",
                div { class: "rudi-card-title", "{title}" }
                div { class: "rudi-card-contents", {props.children} }
            
            }
        }
    }
}
