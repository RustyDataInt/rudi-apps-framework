//! `RudiElement` support for a Dioxus RadioGroup, which wraps
//! a set of HTML `radio` elements, as component `RadioGroupInput`.

// imports
use std::str::FromStr;
use std::fmt::Display;
use serde::{de::DeserializeOwned, Serialize};
use dioxus::prelude::*;
use crate::components::{label::Label, radio_group::{RadioGroup, RadioItem}};
use crate::ui::*;

/// Dioxus Properties for a `RadioGroupInput`.
#[derive(PartialEq, Clone, Props)]
pub struct RadioGroupInputProps<T> 
where T: 'static + 
    Clone + PartialEq + 
    Serialize + DeserializeOwned + FromStr + 
    Display
{
    // required
    name:    String,
    value:   Signal<T>,
    choices: Vec<T>,
    labels:  Option<Vec<String>>,
    // optional
    label: Option<String>,
    width: Option<u16>,
}

/// A `RadioGroupInput` creates a stateful HTML radio group.
/// 
/// `choices` is a vector of values of type `T` used to populate 
/// the select options, `labels` is an optional vector of strings 
/// used to label the options, and `default` is the initial value 
/// of type `T` to select.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// `value` is a `Signal<T>` that is updated whenever the user changes 
/// the input value.
/// 
/// If provided, `label` will place a text label above the input.
/// 
/// `width` is the input pixel width that defaults to `InputWidth(235)`,
/// which mean by default it takes up three single-column input slots.
#[component]
pub fn RadioGroupInput<T>(mut props: RadioGroupInputProps<T>) -> Element 
where T: 'static + 
    Clone + PartialEq + 
    Serialize + DeserializeOwned + FromStr + 
    Display
{
    let this = RudiElement::new::<T>(&props.name);
    use_context_provider(|| Namespace::from(&this));
    
    let default_input_width = use_context::<InputWidth>();
    let prop_width = props.width.unwrap_or(default_input_width.0 * 3 + 5 * 2);

    let labels = props.labels.clone()
        .unwrap_or(props.choices.iter().map(|c| c.to_string()).collect());

    let choices = props.choices.iter()
        .enumerate()
        .map(|(i, choice)| {
            rsx! {
                RadioItem { index: i, value: choice.to_string(), "{labels[i]}" }
            }
        });

    rsx!{
        div {
            class: "input-wrapper radio-group-input-wrapper",
            style: "width: {prop_width}px;",
            if props.label.is_some() {
                Label { html_for: this.id.clone(), "{props.label.as_ref().unwrap()}" }
            }
            RadioGroup {
                width: "100%",
                default_value: props.value.read().clone(),
                on_value_change: move |new: String| {
                    if let Ok(new) = new.parse::<T>() {
                        this.set_state::<T>(&new);
                        props.value.set(new);
                    }
                },
                {choices}
            }
        }
    }
}
