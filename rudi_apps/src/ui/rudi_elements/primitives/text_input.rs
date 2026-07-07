//! `RudiElement` support for HTML `input<type="text">` 
//! as component `TextInput`.

// imports
use std::str::FromStr;
use serde::{de::DeserializeOwned, Serialize};
use dioxus::prelude::*;
use dioxus::dioxus_core::IntoAttributeValue;
use crate::components::{input::Input, label::Label};
use crate::ui::*;

/// Dioxus Properties for a `TextInput`.
#[derive(PartialEq, Clone, Props)]
pub struct TextInputProps<T> 
where T: 'static + 
    Clone + 
    PartialEq + 
    Serialize + DeserializeOwned + FromStr + 
    IntoAttributeValue
{
    // required
    name:      String,
    value:     Signal<T>,
    // optional
    label:     Option<String>,
    width:     Option<String>,
    placeholder: Option<String>,
}

/// A `TextInput` creates a stateful  HTML `input<type="text">` 
/// element whose value is cast to a caller-defined data type.
/// That type is often just `String`, but it can be any type that 
/// implements the required traits.
/// 
/// ## Required Properties
/// 
/// The `name` property defines the input id as
/// `<namespace>-<name>`, where `<namespace>` is the id of the parent.
/// 
/// The `value` property is a `Signal<T>` that is updated whenever the 
/// user changes the input value.
/// 
/// ## Optional Properties
/// 
/// If provided, the `label` property will place that text label above
/// the input.
/// 
/// The `width` property is a requested CSS `width` that defaults to "100%".
#[component]
pub fn TextInput<T>(mut props: TextInputProps<T>) -> Element 
where T: 'static + 
    Clone + 
    PartialEq + 
    Serialize + DeserializeOwned + FromStr + 
    IntoAttributeValue
{
    let this = RudiElement::new::<T>(&props.name);
    use_context_provider(|| Namespace::from(&this));
    let prop_width = props.width.unwrap_or("100%".to_string());
    rsx!{
        div { class: "input-wrapper text-input-wrapper",
            if props.label.is_some() {
                Label { html_for: this.id.clone(), "{props.label.as_ref().unwrap()}" }
            }
            Input {
                id: this.id.clone(),
                style: "width: {prop_width};",
                r#type: "text",
                value: props.value,
                placeholder: props.placeholder,
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
