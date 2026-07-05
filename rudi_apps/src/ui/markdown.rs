//! Support for rendering Markdown content.

// imports
use dioxus::prelude::*;

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
