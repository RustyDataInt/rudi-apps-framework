//! A component for a standardized `DisplayPanel` carrying a single interactive 
//! plot, built internally from an `rlike::DataFrame` and rendered with an 
//! opinionated deployment of `plotters`.

// modules
mod overlay;

// re-exports
// pub use config::*;

// imports
use dioxus::prelude::*;
// use image::codecs::png::PngEncoder as RgbToPng;
// use image::{ColorType, ImageEncoder};
// use base64::{engine::general_purpose::STANDARD as PngToBase64, Engine as _};
use rlike::data_frame::DataFrame;
use crate::ui::*;
use crate::plot::*;
use super::*;
// use overlay::*;

/// Dioxus Properties for an `PlotPanel`.
#[derive(PartialEq, Clone, Props)]
pub struct PlotPanelProps<T, X, Y> 
where T: 'static + Clone + PartialEq + PartialOrd,
      X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    // required
    data:       Option<Signal<Vec<T>>>, // either `data` or `data_frame` is required
    data_frame: Option<Signal<DataFrame>>,
    config:     Signal<PlotConfig<X, Y>>,
    // optional
    default_input_width: Option<InputWidth>,
    // from DisplayPanelProps
    name:      String,
    title:     Option<String>,
    n_columns: u8,
    min_width: Option<String>,
}

/// A component for a standardized display panel carrying a single interactive 
/// plot rendered with `plotters`.
/// 
/// A `PlotPanel` requires a value for either `data` or `data_frame` to provide 
/// the data to plot as a `Vec<T>` or `rlike::DataFrame`, respectively.
/// 
/// `config` provides instructions for how to intially draw the plot; these 
/// initial values may then be modified by the user.
/// 
/// `name` is used to define the input id as `<namespace>-<name>`, where 
/// `<namespace>` is the id of the parent.
/// 
/// If provided, `title` will be displayed in the panel header, otherwise
/// a thin solid header will be shown.
/// 
/// `n_columns` and `min_width` are passed to the `FluidSpan` to control
/// the size of the panel in the `FluidPage`.
#[component]
pub fn PlotPanel<T, X, Y>(props: PlotPanelProps<T, X, Y>) -> Element 
where T: 'static + Clone + PartialEq + PartialOrd,
      X: 'static + Clone + PartialEq + PartialOrd,
      Y: 'static + Clone + PartialEq + PartialOrd
{
    let default_input_width = props.default_input_width
        .unwrap_or(InputWidth(DEFAULT_INPUT_WIDTH));
    use_context_provider(|| default_input_width);

    let df = get_df(props.data, props.data_frame, false);
    let grid = use_memo(move || {
        let config = &*props.config.read();
        Some(PlotGridConfig::new(&[config], 1, 1)) // a single plot in a 1x1 grid
    });
    let canvas_id = use_signal(|| None::<String>);

    use_effect(move || {
        let Some(df) = df else { return; };
        let df = &*df.read();
        let config = &*props.config.read();
        let Some(grid) = &*grid.read() else { return; };
        let Some(canvas_id) = &*canvas_id.read() else { return; };
        // async move {
        // }
        draw_plots(df, &[config], grid, canvas_id);
    });

    rsx!{
        DisplayPanel {
            name: props.name,
            title: props.title,
            n_columns: props.n_columns,
            min_width: props.min_width,
            if let Some(df) = df {
                div { class: "plot-panel-contents",
                    PlotCanvas { grid, canvas_id }
                                // PlotPanelOverlay { df, config: props.config }
                // PlotPanelCrosshairs {}
                }
            } else {
                div { class: "display-panel-error", "Missing data!" }
            }
        }
    }
}
