//! A tooltip with a customizable delay before it appears. 

// imports
use dioxus::prelude::*;
use super::async_delay;

/// Dioxus Properties for a `Tooltip`.
#[derive(PartialEq, Clone, Props)]
pub struct TooltipProps {
    // required
    text:     String,
    children: Element,
    // optional
    delay_ms: Option<u32>,
}

/// A tooltip with a customizable delay before it appears. 
/// Wrap your component as follows. Option `delay_ms` is 
/// in optional and defaults to 1000ms, i.e., 1 second.
/// 
/// ```
/// Tooltip {
///    text: "This is a tooltip".to_string(),
///   delay_ms: 500,
///   div { "Hover over me" }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let delay_ms = props.delay_ms.unwrap_or(1000);
    let mut is_hovering = use_signal(|| false);
    let mut delay_is_over = use_signal(|| false);
    let show_tooltip = use_memo(move || {
        if is_hovering(){
            spawn(async move { 
                async_delay(delay_ms).await; 
                delay_is_over.set(true);
            });
            delay_is_over()
        } else {
            false
        }
    });
    rsx! {
        div {
            position: "relative",
            display: "inline-block",
            onmouseenter: move |_| {
                delay_is_over.set(false);
                is_hovering.set(true);
            },
            onmouseleave: move |_| {
                delay_is_over.set(false);
                is_hovering.set(false);
            },
            {props.children}
            if show_tooltip() {
                div { class: "tooltip", "{props.text}" }
            }
        }
    }
}
