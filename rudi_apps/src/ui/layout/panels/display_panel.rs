//! A component for a standardized app step display panel. This is a wrapper 
//! element that defines a RuDI namespace for its children. A `DisplayPanel` is 
//! used by `InputPanel`, `PlotPanel`, `TablePanel`, etc., but can also be used 
//! as a generic panel for any content.

// imports
use dioxus::prelude::*;
use dioxus_icons::lucide::{Plus, Minus, Ellipsis};
use crate::ui::*;

// constants
const ICON_SIZE: u32 = 20;

/// Dioxus Properties for a `DisplayPanel`.
#[derive(PartialEq, Clone, Props)]
pub struct DisplayPanelProps {
    // required
    name: String,
    // optional
    title: Option<String>,
    // header_links: Option<Vec<HeaderLink>>,
    // from FluidSpanProps
    n_columns: u8,
    min_width: Option<String>, // allow callers to prevent overly narrow panels
    children:  Element,
}

/// A component for a standardized app step display panel with a header bar 
/// containing action link icons.
/// 
/// `name` is used to define the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` is placed in a thicker header bar with a +/- 
/// expand/collapse link. If omitted, a thin solid header bar with an ellipsis 
/// icon is itself activated for expand/collapse.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control the 
/// size of the panel in the `FluidPage`.
#[component]
pub fn DisplayPanel(props: DisplayPanelProps) -> Element {
    let this = RudiElement::new::<()>(&props.name);
    use_context_provider(|| Namespace::from(&this));
    let title = props.title.clone().unwrap_or("".to_string());
    let mut is_open = use_signal(|| true);
    rsx!{
        FluidSpan { n_columns: props.n_columns, min_width: props.min_width,
            div { class: "display-panel",
                if title.is_empty() {
                    div {
                        class: "display-panel-title display-panel-title-empty",
                        onclick: move |_| is_open.toggle(),
                        div { class: "display-panel-icon display-panel-ellipsis",
                            Ellipsis { size: ICON_SIZE }
                        }
                    }
                } else {
                    div { class: "display-panel-title display-panel-title-full",
                        "{title}"
                        div {
                            class: "display-panel-icon display-panel-plus-minus",
                            onclick: move |_| is_open.toggle(),
                            if is_open() {
                                Minus { size: ICON_SIZE }
                            } else {
                                Plus { size: ICON_SIZE }
                            }
                        }
                    }
                }
                if is_open() {
                    div { class: "display-panel-contents", {props.children} }
                }
            }
        }
    }
}
