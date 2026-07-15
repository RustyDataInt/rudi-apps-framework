//! A component for a standardized `DisplayPanel` of user inputs.

// imports
use dioxus::prelude::*;
use crate::ui::*;

/// Dioxus Properties for an `InputPanel`.
#[derive(PartialEq, Clone, Props)]
pub struct InputPanelProps {
    // optional
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
    children:  Element,
}

/// A component for a standardized `DisplayPanel` to organize inputs on screen.
/// 
/// An `InputPanel` expects one or more `InputRow` as children, each with one or 
/// more RuDI input elements. 
/// 
/// `name` is used to define the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` will be displayed in the panel header, otherwise a thin 
/// solid header will be shown.
/// 
/// `default_input_width` is the default pixel width for all inputs in the panel, 
/// which defaults to 75px. Each input element width can be overridden using its 
/// `width` property.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control the 
/// size of the panel in the `FluidPage`.
#[component]
pub fn InputPanel(props: InputPanelProps) -> Element {
    let default_input_width = props.default_input_width
        .unwrap_or(InputWidth(DEFAULT_INPUT_WIDTH));
    use_context_provider(|| default_input_width);
    rsx!{
        DisplayPanel {
            name: props.name,
            title: props.title,
            n_columns: props.n_columns,
            min_width: props.min_width,
            div { class: "input-panel-contents", {props.children} }
        }
    }
}

/// One row of user inputs to be placed into an `InputPanel` and populated with 
/// one or more RuDI input elements. 
#[component]
pub fn InputRow(children: Element) -> Element {
    rsx!{
        div { class: "input-panel-row", {children} }
    }
}
