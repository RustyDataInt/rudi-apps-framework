//! `RudiElement` support for HTML `input<type="number">` 
//! as component `NumericInput`.

// imports
use std::str::FromStr;
use serde::{de::DeserializeOwned, Serialize};
use dioxus::prelude::*;
use dioxus::dioxus_core::IntoAttributeValue;
use crate::components::input::Input;
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
    name:      String,
    width:     Option<String>,
    min:       Option<T>,
    max:       Option<T>,
    value:     Signal<T>,
}

/// A `NumericInput` creates a stateful  HTML `input<type="number">` 
/// element whose value is cast to a caller-defined numeric data type.
/// 
/// The `name` property is required to define the input id as
/// `<namespace>-<name>`, where `<namespace>` is the id of the parent.
/// 
/// Properties `min` and `max` are optional, and if provided, will 
/// constrain the value to the specified range.
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
    let prop_width = props.width.unwrap_or("100%".to_string());
    rsx!{
        div { class: "numeric-input-wrapper",
            Input {
                id: this.id.clone(),
                style: "width: {prop_width};",
                r#type: "number",
                value: props.value,
                oninput: move |event: FormEvent| {
                    let new = event.parsed::<T>().ok();
                    if let Some(mut new) = new {
                        if let Some(min) = &props.min {
                            if new < *min {
                                new = *min;
                            }
                        }
                        if let Some(max) = &props.max {
                            if new > *max {
                                new = *max;
                            }
                        }
                        this.set_state::<T>(&new);
                        props.value.set(new);
                    }
                },
            }
        }
    }
}
