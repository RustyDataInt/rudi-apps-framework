//! Broadly reusable, generalized app components.

// imports
use dioxus::prelude::*;

/// Add `var(--standard-padding)` between two components in a vertical stack.
#[component]
pub fn Spacer() -> Element {
    rsx!{
        div { class: "spacer" }
    }
}

/// Add `var(--wide-padding)` between two components in a vertical stack.
#[component]
pub fn WideSpacer() -> Element {
    rsx!{
        div { class: "wide-spacer" }
    }
}

/// Return a div with rendered Markdown content.
#[component]
pub fn Markdown(markdown: String) -> Element {
    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    rsx!{
        div { dangerous_inner_html: "{html_output}" }
    }
}
