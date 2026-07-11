//! A component for a standardized `DisplayPanel` carrying a single
//! interactive plot, built from an `rlike::DataFrame` and rendered 
//! with `plotters`.

// imports
use dioxus::prelude::*;
use rlike::data_frame::DataFrame;
use crate::ui::*;

// constants
const DEFAULT_INPUT_WIDTH: u16 = 75; // i.e., a single-column input width, in pixels

/// Description of one XY series to plot from a `DataFrame`.
#[derive(PartialEq, Clone, Props)]
pub struct PlotSeries {
    pub x: String,
    pub y: String,
}

/// Instructions on how to render a plot from a `DataFrame`.
#[derive(PartialEq, Clone, Props)]
pub struct PlotConfig<X, Y>
where X: 'static + Clone + PartialEq + PartialOrd, 
      Y: 'static + Clone + PartialEq + PartialOrd
{
    pub series:    Vec<PlotSeries>,
    pub plot_type: Option<String>,
    pub x_label:   Option<String>,
    pub y_label:   Option<String>,
    pub x_range:   Option<(X, X)>,
    pub y_range:   Option<(Y, Y)>,
    pub color:     Option<Vec<String>>,
}

/// Dioxus Properties for an `PlotPanel`.
#[derive(PartialEq, Clone, Props)]
pub struct PlotPanelProps<T> 
where T: 'static + 
    Clone + 
    PartialEq + PartialOrd
{
    // one is required
    data: Option<Signal<Vec<T>>>,
    data_frame: Option<Signal<DataFrame>>,
    // optional
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
}

/// A component for a standardized display panel carrying a single
/// interactive plot rendered with `plotters`.
/// 
/// An `PlotPanel` requires a value for either the `data` or `data_frame` 
/// property to provide the data to plot as a `Vec<T>` or `rlike::DataFrame`, 
/// respectively.
/// 
/// `name` defines the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` will be displayed in the panel header, otherwise
/// a thin solid header will be shown.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control
/// the size of the panel in the `FluidPage`.
#[component]
pub fn PlotPanel<T>(props: PlotPanelProps<T>) -> Element 
where T: 'static + 
    Clone + 
    PartialEq + PartialOrd
{
    let default_input_width = props.default_input_width
        .unwrap_or(InputWidth(DEFAULT_INPUT_WIDTH));
    use_context_provider(|| default_input_width);

    rsx!{
        DisplayPanel {
            name: props.name,
            title: props.title,
            n_columns: props.n_columns,
            min_width: props.min_width,
            if let Some(df) = get_df(props.data, props.data_frame) {
                div { class: "plot-panel-contents",
                    PlotPanelPlot { df }
                    PlotPanelOverlay { df }
                }
            } else {
                div { class: "data-panel-error", "Missing data!" }
            }
        }
    }
}

/// Perform data processing and render the plot image. 
#[component]
pub fn PlotPanelPlot(df: Signal<DataFrame>) -> Element {
    rsx!{
        div { class: "plot-panel-plot", "pending" }
    }
}

/// Overlay the plot with invisible interactive elements. 
#[component]
pub fn PlotPanelOverlay(df: Signal<DataFrame>) -> Element {
    rsx!{
        div { class: "plot-panel-overlay", "pending" }
    }
}
