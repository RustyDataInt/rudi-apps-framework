//! `RudiElement` support for HTML `input<type="number">` 
//! as component `NumericInput`.

// imports
use std::str::FromStr;
use serde::{de::DeserializeOwned, Serialize};
use dioxus::prelude::*;
use dioxus::dioxus_core::IntoAttributeValue;
use crate::components::{input::Input, label::Label};
use crate::ui::*;

/// Dioxus Properties for a `NumericInput`.
#[derive(PartialEq, Clone, Props)]
pub struct NumericProps<T> 
where T: 'static + 
    Copy + 
    PartialEq + PartialOrd + 
    Serialize + DeserializeOwned + FromStr + 
    IntoAttributeValue
{
    // required
    name:  String,
    value: Signal<T>,
    // optional
    label: Option<String>,
    width: Option<u16>,
    min:   Option<T>,
    max:   Option<T>,
    step:  Option<T>,
}

/// A `NumericInput` creates a stateful  HTML `input<type="number">` 
/// element whose value is cast to a caller-defined numeric data type.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// `value` is a `Signal<T>` that is updated whenever the user changes 
/// the input value.
/// 
/// If provided, `label` will place a text label above the input.
/// 
/// `width` is the input pixel width that defaults to `InputWidth(155)`,
/// which mean by default it takes up two single-column input slots.
/// 
/// `min`, `max`, and `step` constrain the input to the specified range 
/// and dictate the behavior of the up/down arrows in the input field. 
#[component]
pub fn NumericInput<T>(mut props: NumericProps<T>) -> Element 
where T: 'static + 
    Copy + 
    PartialEq + PartialOrd + 
    Serialize + DeserializeOwned + FromStr + 
    IntoAttributeValue
{
    let this = RudiElement::new::<T>(&props.name);
    use_context_provider(|| Namespace::from(&this));

    let default_input_width = use_context::<InputWidth>();
    let prop_width = props.width.unwrap_or(default_input_width.0 * 2 + 5);
    
    rsx!{
        div {
            class: "input-wrapper numeric-input-wrapper",
            style: "width: {prop_width}px;",
            if props.label.is_some() {
                Label { html_for: this.id.clone(), "{props.label.as_ref().unwrap()}" }
            }
            Input {
                id: this.id.clone(),
                style: "width: 100%;",
                r#type: "number",
                value: props.value,
                min: props.min,
                max: props.max,
                step: props.step,
                oninput: move |event: FormEvent| {
                    let new = event.parsed::<T>().ok();
                    if let Some(new) = new {
                        this.set_state::<T>(&new);
                        props.value.set(new);
                    }
                },
            }
        }
    }
}
