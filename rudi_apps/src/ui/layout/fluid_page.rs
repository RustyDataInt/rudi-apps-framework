//! Components that define the Boostrap/R Shiny-like fluid grid.

// imports
use dioxus::prelude::*;

/// Dioxus Properties for a `FluidPage`.
#[derive(PartialEq, Clone, Props)]
pub struct FluidPageProps{
    max_width: Option<String>, // allow callers to prevent overly wide pages
    children:  Element,
}

/// Dioxus Properties for a `FluidSpan`.
#[derive(PartialEq, Clone, Props)]
pub struct FluidSpanProps{
    n_columns: u8,
    min_width: Option<String>, // allow callers to prevent overly narrow panels
    children: Element,
}

/// `FluidPage` is the outer wrapper around an app step
/// content area where a grid of stateful panels is displayed.
/// 
/// A `FluidPage` contains `FluidRow`s, which in turn contain 
/// `FluidSpan`s.
#[component]
pub fn FluidPage(props: FluidPageProps) -> Element {
    rsx!{
        div {
            class: "fluid-page",
            max_width: props.max_width.unwrap_or("100%".to_string()),
            {props.children}
        }
    }
}

/// A `FluidRow` defines one row of `FluidSpan`s within a
/// `FluidPage`.
#[component]
pub fn FluidRow(children: Element) -> Element {
    rsx!{
        div { class: "fluid-row", {children} }
    }
}

/// A `FluidSpan` defines one horizontal display span within
/// a `FluidRow`. A `FluidSpan` has an integer width from 
/// 1 to 12 columns, where 12 columns is the full width of 
/// a `FluidRow`. A `FluidSpan` can contain any number of 
/// child components.
#[component]
pub fn FluidSpan(props: FluidSpanProps) -> Element {
    let width = format!("{}%", props.n_columns as f64 * 8.33);
    let min_width = props.min_width.unwrap_or("200px".to_string());
    rsx!{
        div { class: "fluid-span", width, min_width, {props.children} }
    }
}
