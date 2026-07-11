//! `RudiElement` support for HTML `input<type="text">` 
//! as component `TextInput`.

// imports
use dioxus::prelude::*;
use crate::components::{input::Input, label::Label};
use crate::ui::*;

/// Dioxus Properties for a `TextInput`.
#[derive(PartialEq, Clone, Props)]
pub struct TextInputProps {
    // required
    name:        String,
    value:       Signal<String>,
    // optional
    label:       Option<String>,
    width:       Option<u16>,
    placeholder: Option<String>,
}

/// A `TextInput` creates a stateful  HTML `input<type="text">` 
/// element whose value is `String`.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// `value` is a `Signal<bool>` that is updated whenever the user changes 
/// the input value.
/// 
/// If provided, `label` will place a text label above the input.
/// 
/// `width` is the input pixel width that defaults to `InputWidth(155)`,
/// which mean by default it takes up two single-column input slots.
#[component]
pub fn TextInput(mut props: TextInputProps) -> Element {
    let this = RudiElement::new::<String>(&props.name);
    use_context_provider(|| Namespace::from(&this));
    let default_input_width = use_context::<InputWidth>();
    let prop_width = props.width.unwrap_or(default_input_width.0 * 3 + 5 * 2);
    rsx!{
        div {
            class: "input-wrapper text-input-wrapper",
            style: "width: {prop_width}px;",
            if props.label.is_some() {
                Label { html_for: this.id.clone(), "{props.label.as_ref().unwrap()}" }
            }
            Input {
                id: this.id.clone(),
                style: "width: 100%;",
                r#type: "text",
                value: props.value,
                placeholder: props.placeholder,
                oninput: move |event: FormEvent| {
                    let new = event.value();
                    this.set_state::<String>(&new);
                    props.value.set(new);
                },
            }
        }
    }
}
