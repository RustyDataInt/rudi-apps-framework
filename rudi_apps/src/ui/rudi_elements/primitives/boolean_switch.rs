//! `RudiElement` support for a Dioxus Switch, which wraps
//! an HTML `input<type="checkbox">`, as component `BooleanSwitch`.

// imports
use dioxus::prelude::*;
use crate::components::{label::Label, switch::Switch};
use crate::ui::*;

/// Dioxus Properties for a `BooleanSwitch`.
#[derive(PartialEq, Clone, Props)]
pub struct BooleanSwitchProps{
    // required
    name:   String,
    value:  Signal<bool>,
    // optional
    label:  Option<String>,
    width:  Option<u16>,
}

/// A `BooleanSwitch` creates a stateful HTML `input<type="checkbox">` 
/// element whose value is `bool`, wrapped in a Dioxus `Switch` component.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// `value` is a `Signal<bool>` that is updated whenever the user changes 
/// the input value.
/// 
/// If provided, `label` will place a text label above the input.
/// 
/// `width` is the input pixel width that defaults to `InputWidth(75)`,
/// which mean by default it takes up one single-column input slot.
#[component]
pub fn BooleanSwitch(mut props: BooleanSwitchProps) -> Element 
{
    let this = RudiElement::new::<bool>(&props.name);
    use_context_provider(|| Namespace::from(&this));

    let default_input_width = use_context::<InputWidth>();
    let prop_width = props.width.unwrap_or(default_input_width.0);
    
    rsx!{
        div {
            class: "input-wrapper switch-input-wrapper",
            style: "width: {prop_width}px;",
            if props.label.is_some() {
                Label {
                    html_for: this.id.clone(),
                    style: "justify-content: center;",
                    "{props.label.as_ref().unwrap()}"
                }
            }
            div { text_align: "center", padding_top: "5px",
                Switch {
                    checked: *props.value.read(),
                    on_checked_change: move |new| {
                        this.set_state::<bool>(&new);
                        props.value.set(new);
                    },
                }
            }
        }
    }
}
